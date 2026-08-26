<script lang="ts">
  import { goto } from '$app/navigation';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import type { CreateOpsSiteRequest } from '$lib/types/ops_site';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import OpsSiteForm from '$lib/components/forms/OpsSiteForm.svelte';

  async function handleCreate(data: CreateOpsSiteRequest) {
    await opsSitesApi.create(data);
    goto('/ops-sites');
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '站点', href: '/ops-sites' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建站点</h1>
  </div>

  <OpsSiteForm submitLabel="创建" onSubmit={handleCreate} />
</div>
