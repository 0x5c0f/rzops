<script lang="ts">
  import { goto } from '$app/navigation';
  import { serverIpsApi } from '$lib/api/server-ips';
  import type { CreateServerIpRequest } from '$lib/types/server_ip';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import ServerIpForm from '$lib/components/forms/ServerIpForm.svelte';

  async function handleCreate(data: CreateServerIpRequest) {
    await serverIpsApi.create(data);
    goto('/server-ips');
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '服务器IP', href: '/server-ips' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建服务器IP</h1>
  </div>

  <ServerIpForm submitLabel="创建" onSubmit={handleCreate} />
</div>
