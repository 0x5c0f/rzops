<script lang="ts">
  import AttachmentSection from '$lib/components/shared/AttachmentSection.svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { datacentersApi } from '$lib/api/datacenters';
  import { serversApi } from '$lib/api/servers';
  import type { DataCenterResponse } from '$lib/types/datacenter';
  import type { ServerResponse } from '$lib/types/server';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import * as Table from '$lib/ui/table';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { getOptionLabel, commonStatusOptions } from '$lib/utils/enum-options';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';

  let datacenter = $state<DataCenterResponse | null>(null);
  let servers = $state<ServerResponse[]>([]);
  let loading = $state(true);
  let providerMap = $state<Record<string, string>>({});

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/datacenters'); return; }

    try {
      const [dcData, serverData, provOptions] = await Promise.all([
        datacentersApi.getById(id),
        serversApi.list({ data_center_id: id, per_page: 100 }),
        getProviderOptions(),
      ]);
      datacenter = dcData;
      servers = serverData.data;
      providerMap = Object.fromEntries(provOptions.map(o => [o.value, o.label]));
    } catch (err) {
      console.error('Failed to load datacenter:', err);
      goto('/datacenters');
    } finally {
      loading = false;
    }
  });

  async function handleDelete() {
    if (!datacenter) return;
    if (!confirm(`确定要删除数据中心 "${datacenter.name}" 吗？`)) return;
    try {
      await datacentersApi.delete(datacenter.id);
      goto('/datacenters');
    } catch (err) {
      console.error('Failed to delete datacenter:', err);
    }
  }
</script>

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '数据中心', href: '/datacenters' },
    { label: datacenter?.name || '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if datacenter}
    <!-- 页面标题 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold">{datacenter.name}</h1>
        <StatusBadge status={datacenter.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/datacenters')}>返回列表</Button>
        <Button onclick={() => goto(`/datacenters/${datacenter?.id}/edit`)}>编辑</Button>
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
              <dt class="text-muted-foreground">名称</dt>
              <dd>{datacenter.name}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">供应商</dt>
              <dd>
                {#if datacenter.provider_id}
                  <a href="/providers/{datacenter.provider_id}" class="text-primary hover:underline">
                    {providerMap[datacenter.provider_id] || datacenter.provider_id}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">状态</dt>
              <dd>{getOptionLabel($commonStatusOptions, datacenter.status)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">描述</dt>
              <dd>{datacenter.description || '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>

      <!-- 地理信息 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>地理信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">国家</dt>
              <dd>{datacenter.country || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">省份</dt>
              <dd>{datacenter.province || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">城市</dt>
              <dd>{datacenter.city || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">线路类型</dt>
              <dd>{datacenter.line_type || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">地址</dt>
              <dd>{datacenter.address || '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>

      <!-- 联系信息 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>联系信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">电话</dt>
              <dd>{datacenter.phone || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">创建时间</dt>
              <dd>{formatDate(datacenter.created_at)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">更新时间</dt>
              <dd>{formatDate(datacenter.updated_at)}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>
    </div>

    <!-- 关联服务器 -->
    <Card.Root>
      <Card.Header>
        <Card.Title>关联服务器 ({servers.length})</Card.Title>
      </Card.Header>
      <Card.Content>
        {#if servers.length === 0}
          <p class="text-sm text-muted-foreground">暂无关联服务器</p>
        {:else}
          <Table.Root>
            <Table.Header>
              <Table.Row>
                <Table.Head>名称</Table.Head>
                <Table.Head>资产编号</Table.Head>
                <Table.Head>主IP</Table.Head>
                <Table.Head>服务器类型</Table.Head>
                <Table.Head>状态</Table.Head>
              </Table.Row>
            </Table.Header>
            <Table.Body>
              {#each servers as server}
                <Table.Row>
                  <Table.Cell>
                    <a href="/servers/{server.id}" class="text-primary hover:underline">
                      {server.name}
                    </a>
                  </Table.Cell>
                  <Table.Cell class="font-mono">{server.asset_code || '-'}</Table.Cell>
                  <Table.Cell class="font-mono">{server.primary_ip || '-'}</Table.Cell>
                  <Table.Cell>{server.server_type || '-'}</Table.Cell>
                  <Table.Cell><StatusBadge status={server.status} /></Table.Cell>
                </Table.Row>
              {/each}
            </Table.Body>
          </Table.Root>
        {/if}
      </Card.Content>
    </Card.Root>
    <AttachmentSection targetType="data_center" targetId={datacenter.id} />
  {/if}
</div>
