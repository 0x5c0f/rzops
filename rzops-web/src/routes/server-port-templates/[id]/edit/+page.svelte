<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { serverPortTemplatesApi } from '$lib/api/server-port-templates';
  import type { CreateServerPortTemplateRequest, ServerPortTemplateResponse } from '$lib/types/server_port_template';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import ServerPortTemplateForm from '$lib/components/forms/ServerPortTemplateForm.svelte';
  import { onMount } from 'svelte';

  let tpl = $state<ServerPortTemplateResponse | null>(null);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/server-port-templates'); return; }
    try {
      tpl = await serverPortTemplatesApi.getById(id);
    } catch (err) {
      console.error('Failed to load port template:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(t: ServerPortTemplateResponse): CreateServerPortTemplateRequest {
    return {
      name: t.name,
      protocol: t.protocol,
      port: t.port,
      service_name: t.service_name,
      access_scope: t.access_scope ?? '',
      is_enabled: t.is_enabled ?? true,
      description: t.description ?? '',
    };
  }

  async function handleUpdate(data: CreateServerPortTemplateRequest) {
    const id = $page.params.id;
    if (!id) return;
    await serverPortTemplatesApi.update(id, data);
    goto(`/server-port-templates/${id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '端口模板', href: '/server-port-templates' },
    { label: tpl ? tpl.name : '详情', href: tpl ? `/server-port-templates/${tpl.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑端口模板</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !tpl}
    <p class="text-sm text-muted-foreground">加载失败，端口模板可能不存在。</p>
  {:else}
    <ServerPortTemplateForm initial={toForm(tpl)} submitLabel="保存" onSubmit={handleUpdate} />
  {/if}
</div>
