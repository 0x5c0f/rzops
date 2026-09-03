<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { serverIpsApi } from '$lib/api/server-ips';
  import type { ServerIpResponse } from '$lib/types/server_ip';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { ipTypeOptions, ipStatusOptions, getOptionLabel } from '$lib/utils/enum-options';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { formatResourceWithStatus, getResourceStatusClass } from '$lib/utils/resource-status';
  import { onMount } from 'svelte';
  import ConfirmDialog from '$lib/components/shared/ConfirmDialog.svelte';

  let serverIp = $state<ServerIpResponse | null>(null);

  let confirmOpen = $state(false);
  let loading = $state(true);
  let providerMap = $state<Record<string, string>>({});

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/server-ips'); return; }
    try {
      const [ip, providers] = await Promise.all([
        serverIpsApi.getById(id),
        getProviderOptions(),
      ]);
      serverIp = ip;
      providerMap = Object.fromEntries(providers.map(o => [o.value, o.label]));
    } catch (err) {
      console.error('Failed to load server IP:', err);
      goto('/server-ips');
    } finally {
      loading = false;
    }
  });

  function handleDelete() {
    if (!serverIp) return;
    confirmOpen = true;
  }

  async function doDelete() {
    if (!serverIp) return;
    try {
      await serverIpsApi.delete(serverIp.id);
      goto('/server-ips');
      } catch (err) {
      console.error('Failed to delete server IP:', err);
    }
  }
</script>

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '服务器IP', href: '/server-ips' },
    { label: serverIp?.ip_address || '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if serverIp}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold font-mono">{serverIp.ip_address}</h1>
        <StatusBadge status={serverIp.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/server-ips')}>返回列表</Button>
        <Button onclick={() => goto(`/server-ips/${serverIp?.id}/edit`)}>编辑</Button>
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
      </div>
    </div>

    <div class="grid gap-6 lg:grid-cols-2">
      <Card.Root>
        <Card.Header>
          <Card.Title>基本信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">服务器</dt>
              <dd>
                {#if serverIp.server_id}
                  {#if serverIp.server_name}
                    <a href="/servers/{serverIp.server_id}" class="text-primary hover:underline">
                      <span class={getResourceStatusClass(serverIp.server_status, 'server')}>
                        {formatResourceWithStatus(serverIp.server_name, serverIp.server_status, 'server')}
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
              <dt class="text-muted-foreground">IP地址</dt>
              <dd class="font-mono">{serverIp.ip_address}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">网卡名称</dt>
              <dd>{serverIp.nic_name || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">IP类型</dt>
              <dd>{getOptionLabel($ipTypeOptions, serverIp.ip_type)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">主 IP</dt>
              <dd>{serverIp.is_primary ? '是' : '否'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">ISP供应商</dt>
              <dd>
                {#if serverIp.isp_provider_id}
                  <a href="/providers/{serverIp.isp_provider_id}" class="text-primary hover:underline">
                    {providerMap[serverIp.isp_provider_id] || serverIp.isp_provider_id}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">状态</dt>
              <dd>{getOptionLabel($ipStatusOptions, serverIp.status)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">描述</dt>
              <dd>{serverIp.description || '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>
    </div>
  {/if}

    <ConfirmDialog
      bind:open={confirmOpen}
      title="确认删除"
      description={`确定要删除服务器IP「${serverIp?.ip_address}」吗？此操作可在回收站恢复。`}
      confirmLabel="删除"
      onConfirm={doDelete}
    />
</div>
