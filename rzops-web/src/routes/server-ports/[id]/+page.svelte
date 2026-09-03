<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { serverPortsApi } from '$lib/api/server-ports';
  import type { ServerPortResponse, ServerBrief } from '$lib/types/server_port';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import RelatedListCard from '$lib/components/shared/RelatedListCard.svelte';
  import { getStatusLabel } from '$lib/utils/resource-status';
  import { onMount } from 'svelte';
  import ConfirmDialog from '$lib/components/shared/ConfirmDialog.svelte';

  let serverPort = $state<ServerPortResponse | null>(null);

  let confirmOpen = $state(false);
  let loading = $state(true);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/server-ports'); return; }
    try {
      serverPort = await serverPortsApi.getById(id);
    } catch (err) {
      console.error('Failed to load server port:', err);
      goto('/server-ports');
    } finally {
      loading = false;
    }
  });

  function handleDelete() {
    if (!serverPort) return;
    confirmOpen = true;
  }

  async function doDelete() {
    if (!serverPort) return;
    try {
      await serverPortsApi.delete(serverPort.id);
      goto('/server-ports');
      } catch (err) {
      console.error('Failed to delete server port:', err);
    }
  }
</script>

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '服务器端口', href: '/server-ports' },
    { label: serverPort ? `${serverPort.protocol}/${serverPort.port}` : '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if serverPort}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold">{serverPort.protocol}/{serverPort.port}</h1>
        <span class={`inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${serverPort.is_enabled ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-600'}`}>
          {serverPort.is_enabled ? '启用' : '停用'}
        </span>
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/server-ports')}>返回列表</Button>
        <Button onclick={() => goto(`/server-ports/${serverPort?.id}/edit`)}>编辑</Button>
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
      </div>
    </div>

    <Card.Root>
      <Card.Header>
        <Card.Title>基本信息</Card.Title>
      </Card.Header>
      <Card.Content>
        <dl class="grid gap-3 text-sm">
          <div class="flex justify-between">
            <dt class="text-muted-foreground">协议</dt>
            <dd>{serverPort.protocol}</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-muted-foreground">端口</dt>
            <dd class="font-mono">{serverPort.port}</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-muted-foreground">服务名称</dt>
            <dd>{serverPort.service_name}</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-muted-foreground">访问范围</dt>
            <dd>{serverPort.access_scope || '-'}</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-muted-foreground">状态</dt>
            <dd>{serverPort.is_enabled ? '启用' : '停用'}</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-muted-foreground">描述</dt>
            <dd>{serverPort.description || '-'}</dd>
          </div>
        </dl>
      </Card.Content>
    </Card.Root>

    <RelatedListCard
      title="关联服务器"
      description="该端口当前绑定并开放的服务器"
      items={serverPort.servers}
      emptyText="暂无关联服务器"
      columns={[
        {
          key: 'name',
          label: '服务器',
          link: (s: ServerBrief) => `/servers/${s.id}`,
          render: (s: ServerBrief) => s.name,
          badge: (s: ServerBrief) => ({ status: s.status, label: getStatusLabel(s.status) })
        }
      ]}
    />
  {/if}

    <ConfirmDialog
      bind:open={confirmOpen}
      title="确认删除"
      description={`确定要删除端口「${serverPort?.protocol}」吗？此操作可在回收站恢复。`}
      confirmLabel="删除"
      onConfirm={doDelete}
    />
</div>
