<script lang="ts">
  import { Card, CardContent, CardHeader, CardTitle } from '$lib/ui/card';
  import { serversApi } from '$lib/api/servers';
  import { datacentersApi } from '$lib/api/datacenters';
  import { domainsApi } from '$lib/api/domains';
  import { certificatesApi } from '$lib/api/certificates';
  import { providersApi } from '$lib/api/providers';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import { onMount } from 'svelte';
  import { formatDate } from '$lib/utils/format';

  let stats = $state({
    servers: 0,
    datacenters: 0,
    domains: 0,
    certificates: 0,
    providers: 0,
    sites: 0,
    databases: 0,
  });

  // 到期提醒
  interface ExpiryItem {
    type: 'server' | 'domain' | 'certificate';
    typeLabel: string;
    id: string;
    name: string;
    expiryDate: string;
    daysLeft: number;
    level: 'expired' | 'urgent' | 'warning' | 'soon';
    href: string;
  }

  let expiryItems = $state<ExpiryItem[]>([]);
  let expiryStats = $state({
    expired: 0,
    urgent: 0,
    warning: 0,
    soon: 0,
  });
  let activeExpiryTab = $state<'all' | 'server' | 'domain' | 'certificate'>('all');

  let filteredExpiryItems = $derived.by(() => {
    if (activeExpiryTab === 'all') return expiryItems;
    return expiryItems.filter(item => item.type === activeExpiryTab);
  });

  let expiryTabCounts = $derived.by(() => ({
    all: expiryItems.length,
    server: expiryItems.filter(i => i.type === 'server').length,
    domain: expiryItems.filter(i => i.type === 'domain').length,
    certificate: expiryItems.filter(i => i.type === 'certificate').length,
  }));

  let loading = $state(true);

  function calcDaysLeft(dateStr: string): number {
    const now = new Date();
    now.setHours(0, 0, 0, 0);
    const expiry = new Date(dateStr);
    expiry.setHours(0, 0, 0, 0);
    return Math.ceil((expiry.getTime() - now.getTime()) / (1000 * 60 * 60 * 24));
  }

  function getExpiryLevel(daysLeft: number): ExpiryItem['level'] {
    if (daysLeft < 0) return 'expired';
    if (daysLeft <= 7) return 'urgent';
    if (daysLeft <= 30) return 'warning';
    return 'soon';
  }

  function getLevelLabel(level: ExpiryItem['level']): string {
    switch (level) {
      case 'expired': return '已过期';
      case 'urgent': return '7天内';
      case 'warning': return '30天内';
      case 'soon': return '90天内';
    }
  }

  function getLevelClass(level: ExpiryItem['level']): string {
    switch (level) {
      case 'expired': return 'bg-red-100 text-red-700';
      case 'urgent': return 'bg-orange-100 text-orange-700';
      case 'warning': return 'bg-yellow-100 text-yellow-700';
      case 'soon': return 'bg-blue-100 text-blue-700';
    }
  }

  function getTypeClass(type: ExpiryItem['type']): string {
    switch (type) {
      case 'server': return 'bg-purple-100 text-purple-700';
      case 'domain': return 'bg-green-100 text-green-700';
      case 'certificate': return 'bg-cyan-100 text-cyan-700';
    }
  }

  onMount(async () => {
    try {
      const [servers, datacenters, domains, certificates, providers, sites, databases] = await Promise.all([
        serversApi.list({ per_page: 200 }).catch(() => ({ count: 0, data: [] })),
        datacentersApi.list({ per_page: 1 }).catch(() => ({ count: 0 })),
        domainsApi.list({ per_page: 200 }).catch(() => ({ count: 0, data: [] })),
        certificatesApi.list({ per_page: 200 }).catch(() => ({ count: 0, data: [] })),
        providersApi.list({ per_page: 1 }).catch(() => ({ count: 0 })),
        opsSitesApi.list({ per_page: 1 }).catch(() => ({ count: 0 })),
        databaseInstancesApi.list({ per_page: 1 }).catch(() => ({ count: 0 })),
      ]);

      stats = {
        servers: servers.count ?? 0,
        datacenters: datacenters.count ?? 0,
        domains: domains.count ?? 0,
        certificates: certificates.count ?? 0,
        providers: providers.count ?? 0,
        sites: sites.count ?? 0,
        databases: databases.count ?? 0,
      };

      // 构建到期提醒列表
      const items: ExpiryItem[] = [];

      // 服务器租赁到期（过滤已退役服务器）
      if (servers.data) {
        for (const s of servers.data) {
          if (s.lease_end_date && s.status !== 'retired') {
            const daysLeft = calcDaysLeft(s.lease_end_date);
            if (daysLeft <= 90) {
              items.push({
                type: 'server',
                typeLabel: '服务器',
                id: s.id,
                name: s.name,
                expiryDate: s.lease_end_date,
                daysLeft,
                level: getExpiryLevel(daysLeft),
                href: `/servers/${s.id}`,
              });
            }
          }
        }
      }

      // 域名到期（过滤未启用域名）
      if (domains.data) {
        for (const d of domains.data) {
          if (d.expiry_date && d.is_enabled !== false) {
            const daysLeft = calcDaysLeft(d.expiry_date);
            if (daysLeft <= 90) {
              items.push({
                type: 'domain',
                typeLabel: '域名',
                id: d.id,
                name: d.domain_name,
                expiryDate: d.expiry_date,
                daysLeft,
                level: getExpiryLevel(daysLeft),
                href: `/domains/${d.id}`,
              });
            }
          }
        }
      }

      // 证书到期
      if (certificates.data) {
        for (const c of certificates.data) {
          if (c.lease_end_date) {
            const daysLeft = calcDaysLeft(c.lease_end_date);
            if (daysLeft <= 90) {
              items.push({
                type: 'certificate',
                typeLabel: '证书',
                id: c.id,
                name: c.name,
                expiryDate: c.lease_end_date,
                daysLeft,
                level: getExpiryLevel(daysLeft),
                href: `/certificates/${c.id}`,
              });
            }
          }
        }
      }

      // 按到期时间升序排序
      items.sort((a, b) => a.daysLeft - b.daysLeft);
      expiryItems = items;

      // 统计
      expiryStats = {
        expired: items.filter(i => i.level === 'expired').length,
        urgent: items.filter(i => i.level === 'urgent').length,
        warning: items.filter(i => i.level === 'warning').length,
        soon: items.filter(i => i.level === 'soon').length,
      };
    } catch (err) {
      console.error('Failed to load stats:', err);
    } finally {
      loading = false;
    }
  });
</script>

<div class="space-y-6">
  <h1 class="text-3xl font-bold">Dashboard</h1>

  <!-- Stats cards -->
  <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
    <Card>
      <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle class="text-sm font-medium">服务器</CardTitle>
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{loading ? '...' : stats.servers}</div>
        <p class="text-xs text-muted-foreground">管理中的服务器</p>
      </CardContent>
    </Card>

    <Card>
      <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle class="text-sm font-medium">站点</CardTitle>
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{loading ? '...' : stats.sites}</div>
        <p class="text-xs text-muted-foreground">运维站点总数</p>
      </CardContent>
    </Card>

    <Card>
      <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle class="text-sm font-medium">域名</CardTitle>
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{loading ? '...' : stats.domains}</div>
        <p class="text-xs text-muted-foreground">已注册域名</p>
      </CardContent>
    </Card>

    <Card>
      <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle class="text-sm font-medium">证书</CardTitle>
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{loading ? '...' : stats.certificates}</div>
        <p class="text-xs text-muted-foreground">SSL 证书</p>
      </CardContent>
    </Card>
  </div>

  <!-- More stats -->
  <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
    <Card>
      <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle class="text-sm font-medium">供应商</CardTitle>
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{loading ? '...' : stats.providers}</div>
        <p class="text-xs text-muted-foreground">服务供应商</p>
      </CardContent>
    </Card>

    <Card>
      <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle class="text-sm font-medium">数据中心</CardTitle>
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{loading ? '...' : stats.datacenters}</div>
        <p class="text-xs text-muted-foreground">数据中心数量</p>
      </CardContent>
    </Card>

    <Card>
      <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle class="text-sm font-medium">数据库实例</CardTitle>
      </CardHeader>
      <CardContent>
        <div class="text-2xl font-bold">{loading ? '...' : stats.databases}</div>
        <p class="text-xs text-muted-foreground">数据库实例数量</p>
      </CardContent>
    </Card>
  </div>

  <!-- 到期提醒统计 -->
  <div class="space-y-4">
    <h2 class="text-xl font-semibold">到期提醒</h2>
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      <Card class="border-l-4 border-l-red-500">
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">已过期</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold text-red-600">{loading ? '...' : expiryStats.expired}</div>
          <p class="text-xs text-muted-foreground">需立即处理</p>
        </CardContent>
      </Card>

      <Card class="border-l-4 border-l-orange-500">
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">7天内到期</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold text-orange-600">{loading ? '...' : expiryStats.urgent}</div>
          <p class="text-xs text-muted-foreground">紧急续费</p>
        </CardContent>
      </Card>

      <Card class="border-l-4 border-l-yellow-500">
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">30天内到期</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold text-yellow-600">{loading ? '...' : expiryStats.warning}</div>
          <p class="text-xs text-muted-foreground">近期关注</p>
        </CardContent>
      </Card>

      <Card class="border-l-4 border-l-blue-500">
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">90天内到期</CardTitle>
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold text-blue-600">{loading ? '...' : expiryStats.soon}</div>
          <p class="text-xs text-muted-foreground">提前规划</p>
        </CardContent>
      </Card>
    </div>

    <!-- 即将到期列表 -->
    <Card>
      <CardHeader>
        <CardTitle>即将到期明细</CardTitle>
        <p class="text-sm text-muted-foreground">按到期时间排序，仅展示90天内到期及已过期资源</p>
      </CardHeader>
      <CardContent>
        <!-- 标签页切换 -->
        <div class="mb-4 flex gap-2 border-b">
          <button
            class={`px-4 py-2 text-sm font-medium transition-colors ${activeExpiryTab === 'all' ? 'border-b-2 border-primary text-primary' : 'text-muted-foreground hover:text-foreground'}`}
            onclick={() => (activeExpiryTab = 'all')}
          >
            全部 ({expiryTabCounts.all})
          </button>
          <button
            class={`px-4 py-2 text-sm font-medium transition-colors ${activeExpiryTab === 'server' ? 'border-b-2 border-primary text-primary' : 'text-muted-foreground hover:text-foreground'}`}
            onclick={() => (activeExpiryTab = 'server')}
          >
            服务器 ({expiryTabCounts.server})
          </button>
          <button
            class={`px-4 py-2 text-sm font-medium transition-colors ${activeExpiryTab === 'domain' ? 'border-b-2 border-primary text-primary' : 'text-muted-foreground hover:text-foreground'}`}
            onclick={() => (activeExpiryTab = 'domain')}
          >
            域名 ({expiryTabCounts.domain})
          </button>
          <button
            class={`px-4 py-2 text-sm font-medium transition-colors ${activeExpiryTab === 'certificate' ? 'border-b-2 border-primary text-primary' : 'text-muted-foreground hover:text-foreground'}`}
            onclick={() => (activeExpiryTab = 'certificate')}
          >
            证书 ({expiryTabCounts.certificate})
          </button>
        </div>

        {#if loading}
          <div class="py-8 text-center text-muted-foreground">加载中...</div>
        {:else if filteredExpiryItems.length === 0}
          <div class="py-8 text-center text-muted-foreground">该类型暂无即将到期的资源</div>
        {:else}
          <div class="space-y-2">
            {#each filteredExpiryItems as item}
              <div class="flex items-center justify-between rounded-md border p-3 hover:bg-accent/50">
                <div class="flex items-center gap-3">
                  <span class={`inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${getTypeClass(item.type)}`}>
                    {item.typeLabel}
                  </span>
                  <a href={item.href} class="font-medium text-primary hover:underline">
                    {item.name}
                  </a>
                </div>
                <div class="flex items-center gap-3">
                  <span class="text-sm text-muted-foreground">{formatDate(item.expiryDate)}</span>
                  <span class={`inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${getLevelClass(item.level)}`}>
                    {getLevelLabel(item.level)}
                  </span>
                  <span class={`text-sm font-medium ${item.daysLeft < 0 ? 'text-red-600' : item.daysLeft <= 7 ? 'text-orange-600' : 'text-muted-foreground'}`}>
                    {item.daysLeft < 0 ? `已过期${Math.abs(item.daysLeft)}天` : `还剩${item.daysLeft}天`}
                  </span>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </CardContent>
    </Card>
  </div>

  <!-- Quick links -->
  <Card>
    <CardHeader>
      <CardTitle>快速入口</CardTitle>
    </CardHeader>
    <CardContent>
      <div class="grid gap-2 md:grid-cols-3">
        <a href="/servers" class="rounded-md border p-4 hover:bg-accent">
          <div class="font-medium">服务器管理</div>
          <div class="text-sm text-muted-foreground">查看和管理所有服务器</div>
        </a>
        <a href="/ops-sites" class="rounded-md border p-4 hover:bg-accent">
          <div class="font-medium">站点管理</div>
          <div class="text-sm text-muted-foreground">运维站点配置</div>
        </a>
        <a href="/audit-logs" class="rounded-md border p-4 hover:bg-accent">
          <div class="font-medium">审计日志</div>
          <div class="text-sm text-muted-foreground">查看操作记录</div>
        </a>
      </div>
    </CardContent>
  </Card>
</div>
