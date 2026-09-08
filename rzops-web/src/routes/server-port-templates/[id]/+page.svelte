<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { serverPortTemplatesApi } from '$lib/api/server-port-templates';
  import type { ServerPortTemplateResponse } from '$lib/types/server_port_template';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';
  import ConfirmDialog from '$lib/components/shared/ConfirmDialog.svelte';
  import { protocolOptions } from '$lib/utils/enum-options';
import { canUpdate, canDelete } from '$lib/utils/permissions';

  let tpl = $state<ServerPortTemplateResponse | null>(null);
  let confirmOpen = $state(false);
  let loading = $state(true);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/server-port-templates'); return; }
    try {
      tpl = await serverPortTemplatesApi.getById(id);
    } catch (err) {
      console.error('Failed to load port template:', err);
      goto('/server-port-templates');
    } finally {
      loading = false;
    }
  });

  function protocolLabel(v: string): string {
    return $protocolOptions.find(o => o.value === v)?.label ?? v;
  }

  async function doDelete() {
    if (!tpl) return;
    try {
      await serverPortTemplatesApi.delete(tpl.id);
      goto('/server-port-templates');
    } catch (err) {
      console.error('Failed to delete port template:', err);
    }
  }
</script>

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '端口模板', href: '/server-port-templates' },
    { label: tpl ? tpl.name : '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if tpl}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold">{tpl.name}</h1>
        <span class={`inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${tpl.is_enabled ? 'bg-green-100 text-green-700' : 'bg-gray-100 text-gray-600'}`}>
          {tpl.is_enabled ? '启用' : '停用'}
        </span>
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/server-port-templates')}>返回列表</Button>
        {#if canUpdate('server_port_template')}
        <Button onclick={() => goto(`/server-port-templates/${tpl.id}/edit`)}>编辑</Button>
      {/if}
        {#if canDelete('server_port_template')}
        <Button variant="destructive" onclick={() => confirmOpen = true}>删除</Button>
      {/if}
      </div>
    </div>

    <Card.Root>
      <Card.Header>
        <Card.Title>基本信息</Card.Title>
      </Card.Header>
      <Card.Content>
        <dl class="grid gap-3 text-sm">
          <div class="flex justify-between">
            <dt class="text-muted-foreground">模板名称</dt>
            <dd>{tpl.name}</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-muted-foreground">协议</dt>
            <dd>{protocolLabel(tpl.protocol)}</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-muted-foreground">端口</dt>
            <dd class="font-mono">{tpl.port}</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-muted-foreground">服务名称</dt>
            <dd>{tpl.service_name}</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-muted-foreground">访问范围</dt>
            <dd>{tpl.access_scope || '-'}</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-muted-foreground">状态</dt>
            <dd>{tpl.is_enabled ? '启用' : '停用'}</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-muted-foreground">描述</dt>
            <dd>{tpl.description || '-'}</dd>
          </div>
        </dl>
      </Card.Content>
    </Card.Root>
  {/if}

  <ConfirmDialog
    bind:open={confirmOpen}
    title="确认删除"
    description={`确定要删除端口模板「${tpl?.name}」吗？此操作仅删除模板，不影响已生成的端口记录。`}
    confirmLabel="删除"
    onConfirm={doDelete}
  />
</div>
