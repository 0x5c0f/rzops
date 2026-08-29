<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { serverPortsApi } from '$lib/api/server-ports';
  import type { CreateServerPortRequest, ServerPortResponse } from '$lib/types/server_port';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import ServerPortForm from '$lib/components/forms/ServerPortForm.svelte';
  import { onMount } from 'svelte';

  let serverPort = $state<ServerPortResponse | null>(null);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/server-ports'); return; }
    try {
      serverPort = await serverPortsApi.getById(id);
    } catch (err) {
      console.error('Failed to load server port:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(p: ServerPortResponse): CreateServerPortRequest {
    return {
      server_ids: p.server_ids,
      protocol: p.protocol,
      port: p.port,
      service_name: p.service_name,
      access_scope: p.access_scope ?? '',
      is_enabled: p.is_enabled ?? true,
      description: p.description ?? '',
    };
  }

  async function handleUpdate(data: CreateServerPortRequest) {
    const id = $page.params.id;
    if (!id) return;
    await serverPortsApi.update(id, data);
    goto(`/server-ports/${id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '服务器端口', href: '/server-ports' },
    { label: serverPort ? `${serverPort.protocol}/${serverPort.port}` : '详情', href: serverPort ? `/server-ports/${serverPort.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑服务器端口</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !serverPort}
    <p class="text-sm text-muted-foreground">加载失败，服务器端口可能不存在。</p>
  {:else}
    <ServerPortForm
      initial={toForm(serverPort)}
      initialServers={serverPort.servers}
      editing
      submitLabel="保存"
      onSubmit={handleUpdate}
    />
  {/if}
</div>
