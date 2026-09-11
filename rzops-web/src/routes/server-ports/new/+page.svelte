<script lang="ts">
  import { goto } from '$app/navigation';
  import { serverPortsApi } from '$lib/api/server-ports';
  import type { CreateServerPortRequest } from '$lib/types/server_port';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import ServerPortForm from '$lib/components/forms/ServerPortForm.svelte';

  async function handleCreate(data: CreateServerPortRequest) {
    await serverPortsApi.create(data);
    goto('/server-ports');
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '服务器端口', href: '/server-ports' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建服务器端口</h1>
  </div>

  <ServerPortForm submitLabel="创建" onSubmit={handleCreate} />
</div>
