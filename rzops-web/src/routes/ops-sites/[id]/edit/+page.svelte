<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import type { CreateOpsSiteRequest, OpsSiteResponse } from '$lib/types/ops_site';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import OpsSiteForm from '$lib/components/forms/OpsSiteForm.svelte';
  import { onMount } from 'svelte';

  let site = $state<OpsSiteResponse | null>(null);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/ops-sites'); return; }
    try {
      site = await opsSitesApi.getById(id);
    } catch (err) {
      console.error('Failed to load ops site:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(s: OpsSiteResponse): CreateOpsSiteRequest {
    return {
      name: s.name,
      url: s.url ?? '',
      service_target: s.service_target ?? '',
      importance: s.importance ?? '',
      purpose: s.purpose ?? '',
      language_runtime: s.language_runtime ?? '',
      web_framework: s.web_framework ?? '',
      code_repo_type: s.code_repo_type ?? '',
      code_repo_url: s.code_repo_url ?? '',
      function_summary: s.function_summary ?? '',
      remarks: s.remarks ?? '',
      status: s.status ?? 'active',
      is_internal_system: s.is_internal_system ?? false,
      uses_cdn: s.uses_cdn ?? false,
      is_test_site: s.is_test_site ?? false,
      backup_plan_id: s.backup_plan_id ?? '',
      monitor_target_id: s.monitor_target_id ?? '',
    };
  }

  async function handleUpdate(data: CreateOpsSiteRequest) {
    const id = $page.params.id;
    if (!id) return;
    await opsSitesApi.update(id, data);
    goto(`/ops-sites/${id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '站点', href: '/ops-sites' },
    { label: site?.name || '详情', href: site ? `/ops-sites/${site.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑站点</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !site}
    <p class="text-sm text-muted-foreground">加载失败，站点可能不存在。</p>
  {:else}
    <OpsSiteForm initial={toForm(site)} submitLabel="保存" onSubmit={handleUpdate} />
  {/if}
</div>
