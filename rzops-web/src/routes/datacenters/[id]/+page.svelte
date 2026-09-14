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
  import { getOptionLabel, getOptionLabels, getOptionColor, commonStatusOptions, lineTypeOptions } from '$lib/utils/enum-options';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { formatResourceWithStatus, getResourceStatusClass } from '$lib/utils/resource-status';
  import { providersApi } from '$lib/api/providers';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';
  import ConfirmDialog from '$lib/components/shared/ConfirmDialog.svelte';
import { canUpdate, canDelete } from '$lib/utils/permissions';

  let datacenter = $state<DataCenterResponse | null>(null);

  let confirmOpen = $state(false);
  let servers = $state<ServerResponse[]>([]);
  let loading = $state(true);
  let providerMap = $state<Record<string, string>>({});
  let providerStatusMap = $state<Record<string, string>>({});

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/datacenters'); return; }

    try {
      const [dcData, serverData, provOptions, providerList] = await Promise.all([
        datacentersApi.getById(id),
        serversApi.list({ data_center_id: id, per_page: 100 }),
        getProviderOptions(),
        providersApi.list({ per_page: 200 }),
      ]);
      datacenter = dcData;
      servers = serverData.data;
      providerMap = Object.fromEntries(provOptions.map(o => [o.value, o.label]));
      providerStatusMap = Object.fromEntries(providerList.data.map((p: {id: string, status?: string}) => [p.id, p.status || 'active']));
    } catch (err) {
      console.error('Failed to load datacenter:', err);
      goto('/datacenters');
    } finally {
      loading = false;
    }
  });

  function handleDelete() {
    if (!datacenter) return;
    confirmOpen = true;
  }

  async function doDelete() {
    if (!datacenter) return;
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
        <StatusBadge status={datacenter.status} label={getOptionLabel($commonStatusOptions, datacenter.status)} color={getOptionColor($commonStatusOptions, datacenter.status)} />
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/datacenters')}>返回列表</Button>
        {#if canUpdate('datacenter')}
        <Button onclick={() => goto(`/datacenters/${datacenter?.id}/edit`)}>编辑</Button>
      {/if}
        {#if canDelete('datacenter')}
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
      {/if}
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
                  {#if providerMap[datacenter.provider_id]}
                    <a href="/providers/{datacenter.provider_id}" class="text-primary hover:underline">
                      <span class={getResourceStatusClass(providerStatusMap[datacenter.provider_id], 'provider')}>
                        {formatResourceWithStatus(providerMap[datacenter.provider_id], providerStatusMap[datacenter.provider_id], 'provider')}
                      </span>
                    </a>
                  {:else}
                    <span class="text-muted-foreground italic">已删除</span>
                  {/if}
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
              <dt class="text-muted-foreground">线路类型</dt>
              <dd>{getOptionLabels($lineTypeOptions, datacenter.line_type).join(', ') || '-'}</dd>
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

    <ConfirmDialog
      bind:open={confirmOpen}
      title="确认删除"
      description={`确定要删除数据中心「${datacenter?.name}」吗？此操作可在回收站恢复。`}
      confirmLabel="删除"
      onConfirm={doDelete}
    />
</div>
