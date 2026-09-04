<script lang="ts">
  import { goto } from '$app/navigation';
  import { serverPortTemplatesApi } from '$lib/api/server-port-templates';
  import type { CreateServerPortTemplateRequest } from '$lib/types/server_port_template';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import ServerPortTemplateForm from '$lib/components/forms/ServerPortTemplateForm.svelte';

  async function handleCreate(data: CreateServerPortTemplateRequest) {
    await serverPortTemplatesApi.create(data);
    goto('/server-port-templates');
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '端口模板', href: '/server-port-templates' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建端口模板</h1>
  </div>

  <ServerPortTemplateForm submitLabel="创建" onSubmit={handleCreate} />
</div>
