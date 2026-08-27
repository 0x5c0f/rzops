<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { serversApi } from '$lib/api/servers';
  import { serverIpsApi } from '$lib/api/server-ips';
  import { serverPortsApi } from '$lib/api/server-ports';
  import type { ServerResponse } from '$lib/types/server';
  import type { ServerIpResponse } from '$lib/types/server_ip';
  import type { ServerPortResponse } from '$lib/types/server_port';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import * as Table from '$lib/ui/table';
  import { Badge } from '$lib/ui/badge';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import AttachmentSection from '$lib/components/shared/AttachmentSection.svelte';
  import { siteRelationsApi, type SiteRefByServer } from '$lib/api/site-relations';
  import {
    serverTypeOptions, hostingTypeOptions, serverRoleOptions,
    serverStatusOptions, architectureOptions, raidLevelOptions,
    webServerSoftwareOptions, getOptionLabel, getOptionLabels
  } from '$lib/utils/enum-options';
  import { getDataCenterOptions, getProviderOptions, ensureOption } from '$lib/utils/entity-options';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';

  let server = $state<ServerResponse | null>(null);
  let ips = $state<ServerIpResponse[]>([]);
  let ports = $state<ServerPortResponse[]>([]);
  let sites = $state<SiteRefByServer[]>([]);
  let loading = $state(true);
  let dataCenterMap = $state<Record<string, string>>({});
  let providerMap = $state<Record<string, string>>({});

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/servers'); return; }

    try {
      const [serverData, ipData, portData, dcOptions, provOptions] = await Promise.all([
        serversApi.getById(id),
        serverIpsApi.list({ server_id: id, per_page: 100 }),
        serverPortsApi.list({ server_id: id, per_page: 100 }),
        getDataCenterOptions(),
        getProviderOptions(),
      ]);
      server = serverData;
      ips = ipData.data;
      ports = portData.data;
      dataCenterMap = Object.fromEntries(dcOptions.map(o => [o.value, o.label]));
      providerMap = Object.fromEntries(provOptions.map(o => [o.value, o.label]));
      siteRelationsApi.listSitesByServer(id).then(s => { sites = s; }).catch(() => {});
    } catch (err) {
      console.error('Failed to load server:', err);
      goto('/servers');
    } finally {
      loading = false;
    }
  });

  async function handleDelete() {
    if (!server) return;
    if (!confirm(`确定要删除服务器 "${server.name}" 吗？`)) return;
    try {
      await serversApi.delete(server.id);
      goto('/servers');
    } catch (err) {
      console.error('Failed to delete server:', err);
    }
  }
</script>

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '服务器', href: '/servers' },
    { label: server?.name || '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if server}
    <!-- 页面标题 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold">{server.name}</h1>
        <StatusBadge status={server.status} />
        {#if server.is_database_server}
          <Badge variant="secondary">数据库服务器</Badge>
        {/if}
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/servers')}>返回列表</Button>
        <Button onclick={() => goto(`/servers/${server?.id}/edit`)}>编辑</Button>
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
      </div>
    </div>

    <div class="grid gap-6 lg:grid-cols-2">
      <!-- 基本信息 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>基本信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">资产编号</dt>
              <dd class="font-mono">{server.asset_code || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">主IP</dt>
              <dd class="font-mono">{server.primary_ip || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">位置</dt>
              <dd>{server.location || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">服务器类型</dt>
              <dd>{getOptionLabel($serverTypeOptions, server.server_type)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">托管类型</dt>
              <dd>{getOptionLabel($hostingTypeOptions, server.hosting_type)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">架构</dt>
              <dd>{getOptionLabel($architectureOptions, server.architecture)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">操作系统</dt>
              <dd>{server.operating_system || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">品牌</dt>
              <dd>{server.brand || '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>

      <!-- 关联信息 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>关联信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">数据中心</dt>
              <dd>
                {#if server.data_center_id}
                  <a href="/datacenters/{server.data_center_id}" class="text-primary hover:underline">
                    {dataCenterMap[server.data_center_id] || server.data_center_id}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">ISP供应商</dt>
              <dd>
                {#if server.isp_provider_id}
                  <a href="/providers/{server.isp_provider_id}" class="text-primary hover:underline">
                    {providerMap[server.isp_provider_id] || server.isp_provider_id}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">服务器供应商</dt>
              <dd>
                {#if server.server_provider_id}
                  <a href="/providers/{server.server_provider_id}" class="text-primary hover:underline">
                    {providerMap[server.server_provider_id] || server.server_provider_id}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">软件供应商</dt>
              <dd>
                {#if server.software_provider_id}
                  <a href="/providers/{server.software_provider_id}" class="text-primary hover:underline">
                    {providerMap[server.software_provider_id] || server.software_provider_id}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">双线</dt>
              <dd>{server.is_dual_line ? '是' : '否'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">角色标签</dt>
              <dd>{getOptionLabels($serverRoleOptions, server.role_tags).join(', ') || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">Web服务器</dt>
              <dd>{getOptionLabels($webServerSoftwareOptions, server.web_server_type).join(', ') || '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>

      <!-- 硬件配置 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>硬件配置</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">CPU</dt>
              <dd class="font-mono">{server.cpu || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">内存</dt>
              <dd>{server.memory_gb ? `${server.memory_gb} GB` : '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">磁盘配置</dt>
              <dd>{server.disk_layout || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">硬件配置</dt>
              <dd>{server.hardware_config || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">RAID</dt>
              <dd>{server.is_raid ? `是 (${getOptionLabel($raidLevelOptions, server.raid_level)})` : '否'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">保修信息</dt>
              <dd>{server.warranty_info || '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>

      <!-- 租赁信息 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>租赁信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">租赁开始日期</dt>
              <dd>{formatDate(server.lease_start_date)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">租赁结束日期</dt>
              <dd>{formatDate(server.lease_end_date)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">价格</dt>
              <dd>{server.price ? `${server.price} ${server.price_currency}` : '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">创建时间</dt>
              <dd>{formatDate(server.created_at)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">更新时间</dt>
              <dd>{formatDate(server.updated_at)}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>
    </div>

    <!-- IP 地址 -->
    <Card.Root>
      <Card.Header>
        <Card.Title>IP 地址 ({ips.length})</Card.Title>
      </Card.Header>
      <Card.Content>
        {#if ips.length === 0}
          <p class="text-sm text-muted-foreground">暂无 IP 记录</p>
        {:else}
          <Table.Root>
            <Table.Header>
              <Table.Row>
                <Table.Head>IP 地址</Table.Head>
                <Table.Head>类型</Table.Head>
                <Table.Head>主IP</Table.Head>
                <Table.Head>状态</Table.Head>
                <Table.Head>描述</Table.Head>
              </Table.Row>
            </Table.Header>
            <Table.Body>
              {#each ips as ip}
                <Table.Row>
                  <Table.Cell class="font-mono">{ip.ip_address}</Table.Cell>
                  <Table.Cell>{ip.ip_type}</Table.Cell>
                  <Table.Cell>{ip.is_primary ? '是' : '-'}</Table.Cell>
                  <Table.Cell><StatusBadge status={ip.status} /></Table.Cell>
                  <Table.Cell>{ip.description || '-'}</Table.Cell>
                </Table.Row>
              {/each}
            </Table.Body>
          </Table.Root>
        {/if}
      </Card.Content>
    </Card.Root>

    <!-- 服务端口 -->
    <Card.Root>
      <Card.Header>
        <Card.Title>服务端口 ({ports.length})</Card.Title>
      </Card.Header>
      <Card.Content>
        {#if ports.length === 0}
          <p class="text-sm text-muted-foreground">暂无端口记录</p>
        {:else}
          <Table.Root>
            <Table.Header>
              <Table.Row>
                <Table.Head>协议</Table.Head>
                <Table.Head>端口</Table.Head>
                <Table.Head>服务名</Table.Head>
                <Table.Head>访问范围</Table.Head>
                <Table.Head>启用</Table.Head>
              </Table.Row>
            </Table.Header>
            <Table.Body>
              {#each ports as port}
                <Table.Row>
                  <Table.Cell class="font-mono">{port.protocol}</Table.Cell>
                  <Table.Cell class="font-mono">{port.port}</Table.Cell>
                  <Table.Cell>{port.service_name || '-'}</Table.Cell>
                  <Table.Cell>{port.access_scope || '-'}</Table.Cell>
                  <Table.Cell>{port.is_enabled ? '是' : '否'}</Table.Cell>
                </Table.Row>
              {/each}
            </Table.Body>
          </Table.Root>
        {/if}
      </Card.Content>
    </Card.Root>

    <Card.Root>
      <Card.Header>
        <Card.Title>所属站点</Card.Title>
        <Card.Description>该服务器部署承载的站点（在站点详情页维护关联）</Card.Description>
      </Card.Header>
      <Card.Content>
        {#if sites.length === 0}
          <p class="text-sm text-muted-foreground">暂未关联站点</p>
        {:else}
          <Table.Root>
            <Table.Header>
              <Table.Row>
                <Table.Head>站点</Table.Head>
                <Table.Head>部署角色</Table.Head>
                <Table.Head>主用</Table.Head>
              </Table.Row>
            </Table.Header>
            <Table.Body>
              {#each sites as s}
                <Table.Row>
                  <Table.Cell>
                    <a href="/ops-sites/{s.site_id}" class="text-primary hover:underline">{s.site_name}</a>
                  </Table.Cell>
                  <Table.Cell>{s.deploy_role || '-'}</Table.Cell>
                  <Table.Cell>{s.is_primary ? '是' : '-'}</Table.Cell>
                </Table.Row>
              {/each}
            </Table.Body>
          </Table.Root>
        {/if}
      </Card.Content>
    </Card.Root>
    <AttachmentSection targetType="server" targetId={server.id} />
  {/if}
</div>
