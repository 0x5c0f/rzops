<script lang="ts">
  import { goto } from '$app/navigation';
  import { serversApi } from '$lib/api/servers';
  import type { CreateServerRequest } from '$lib/types/server';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import ServerForm from '$lib/components/forms/ServerForm.svelte';

  async function handleCreate(data: CreateServerRequest) {
    const server = await serversApi.create(data);
    return server.id;
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '服务器', href: '/servers' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建服务器</h1>
  </div>

  <ServerForm submitLabel="创建" onSubmit={handleCreate} />
</div>
