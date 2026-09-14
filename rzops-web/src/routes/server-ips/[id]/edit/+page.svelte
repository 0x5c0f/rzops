<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { serverIpsApi } from '$lib/api/server-ips';
  import type { CreateServerIpRequest, ServerIpResponse } from '$lib/types/server_ip';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import ServerIpForm from '$lib/components/forms/ServerIpForm.svelte';
  import { formatResourceWithStatus } from '$lib/utils/resource-status';
  import { onMount } from 'svelte';

  let serverIp = $state<ServerIpResponse | null>(null);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/server-ips'); return; }
    try {
      serverIp = await serverIpsApi.getById(id);
    } catch (err) {
      console.error('Failed to load server IP:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(ip: ServerIpResponse): CreateServerIpRequest {
    return {
      server_id: ip.server_id,
      ip_address: ip.ip_address,
      nic_name: ip.nic_name ?? '',
      ip_type: ip.ip_type ?? '',
      is_primary: ip.is_primary ?? false,
      isp_provider_id: ip.isp_provider_id ?? '',
      status: ip.status || 'enabled',
      description: ip.description ?? '',
    };
  }

  async function handleUpdate(data: CreateServerIpRequest) {
    const id = $page.params.id;
    if (!id) return;
    const { server_id: _serverId, ...rest } = data;
    await serverIpsApi.update(id, rest);
    goto(`/server-ips/${id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '服务器IP', href: '/server-ips' },
    { label: serverIp?.ip_address || '详情', href: serverIp ? `/server-ips/${serverIp.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑服务器IP</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !serverIp}
    <p class="text-sm text-muted-foreground">加载失败，服务器IP可能不存在。</p>
  {:else}
    <ServerIpForm initial={toForm(serverIp)} initialServerName={formatResourceWithStatus(serverIp.server_name, serverIp.server_status, 'server')} editing submitLabel="保存" onSubmit={handleUpdate} />
  {/if}
</div>
