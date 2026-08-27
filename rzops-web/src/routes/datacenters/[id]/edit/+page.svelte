<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { datacentersApi } from '$lib/api/datacenters';
  import type { CreateDataCenterRequest, DataCenterResponse } from '$lib/types/datacenter';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import DataCenterForm from '$lib/components/forms/DataCenterForm.svelte';
  import { onMount } from 'svelte';

  let dc = $state<DataCenterResponse | null>(null);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/datacenters'); return; }
    try {
      dc = await datacentersApi.getById(id);
    } catch (err) {
      console.error('Failed to load datacenter:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(d: DataCenterResponse): CreateDataCenterRequest {
    return {
      name: d.name,
      provider_id: d.provider_id ?? '',
      phone: d.phone ?? '',
      address: d.address ?? '',
      country: d.country ?? '',
      line_type: d.line_type ?? [],
      description: d.description ?? '',
      status: d.status ?? 'active',
    };
  }

  async function handleUpdate(data: CreateDataCenterRequest) {
    const id = $page.params.id;
    if (!id) return;
    await datacentersApi.update(id, data);
    goto(`/datacenters/${id}`);
    return id;
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '数据中心', href: '/datacenters' },
    { label: dc?.name || '详情', href: dc ? `/datacenters/${dc.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑数据中心</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !dc}
    <p class="text-sm text-muted-foreground">加载失败，数据中心可能不存在。</p>
  {:else}
    <DataCenterForm
      initial={toForm(dc)}
      entityId={dc.id}
      submitLabel="保存"
      onSubmit={handleUpdate}
    />
  {/if}
</div>
