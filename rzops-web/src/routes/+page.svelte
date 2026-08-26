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

  let stats = $state({
    servers: 0,
    datacenters: 0,
    domains: 0,
    certificates: 0,
    providers: 0,
    sites: 0,
    databases: 0,
  });

  let loading = $state(true);

  onMount(async () => {
    try {
      const [servers, datacenters, domains, certificates, providers, sites, databases] = await Promise.all([
        serversApi.list({ per_page: 1 }).catch(() => ({ count: 0 })),
        datacentersApi.list({ per_page: 1 }).catch(() => ({ count: 0 })),
        domainsApi.list({ per_page: 1 }).catch(() => ({ count: 0 })),
        certificatesApi.list({ per_page: 1 }).catch(() => ({ count: 0 })),
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
