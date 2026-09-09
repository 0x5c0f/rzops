<script lang="ts">
  import { recycleApi } from '$lib/api/recycle';
  import type { RecycleItem, ListRecycleQuery } from '$lib/types/recycle';
  import { resourceTypeLabels } from '$lib/types/recycle';
  import { canDelete } from '$lib/utils/permissions';
  import { formatDate } from '$lib/utils/format';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import * as Select from '$lib/ui/select';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';

  let data = $state<RecycleItem[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let q = $state('');
  let resourceType = $state('');
  let page = $state(1);
  let perPage = $state(20);

  const columns = [
    {
      key: 'resource_type', label: '资源类型',
      render: (v: unknown) => resourceTypeLabels[v as string] || String(v),
    },
    { key: 'name', label: '名称' },
    { key: 'id', label: 'ID', hideBelow: 'md', render: (v: unknown) => String(v).slice(0, 8) + '…' },
    { key: 'deleted_at', label: '删除时间', hideBelow: 'md', render: (v: unknown) => formatDate(v as string) },
  ];

  async function loadData() {
    loading = true;
    try {
      const params: ListRecycleQuery = {
        q: q || undefined,
        resource_type: resourceType || undefined,
        page,
        per_page: perPage,
      };
      const res = await recycleApi.list(params);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load recycle bin:', err);
    } finally {
      loading = false;
    }
  }

  onMount(loadData);

  async function handleRestore(item: RecycleItem) {
    try {
      await recycleApi.restore({ resource_type: item.resource_type, id: item.id });
      await loadData();
    } catch (err) {
      console.error('Failed to restore item:', err);
    }
  }

  async function handlePurge(item: RecycleItem) {
    try {
      await recycleApi.purge(item.resource_type, item.id);
      await loadData();
    } catch (err) {
      console.error('Failed to purge item:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '回收站' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">回收站</h1>
  </div>

  <div class="flex flex-wrap items-center gap-2">
    <Select.Root type="single" bind:value={resourceType} onValueChange={() => { page = 1; loadData(); }}>
      <Select.Trigger class="w-40">
        {resourceType ? resourceTypeLabels[resourceType] : '全部类型'}
      </Select.Trigger>
      <Select.Content>
        <Select.Item value="">全部类型</Select.Item>
        {#each Object.entries(resourceTypeLabels) as [key, label]}
          <Select.Item value={key}>{label}</Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>
    <Input
      placeholder="搜索名称 / ID..."
      class="max-w-sm"
      value={q}
      oninput={(e) => { q = (e.target as HTMLInputElement).value; page = 1; loadData(); }}
    />
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    storageKey="recycle"
    actionsWidth="w-[150px]"
    {page}
    {perPage}
    extraActions={rowActions}
  />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={(p) => { page = p; loadData(); }}
    onPerPageChange={(s) => { perPage = s; page = 1; loadData(); }}
  />
</div>

{#snippet rowActions(item: RecycleItem)}
  <Button variant="outline" size="sm" class="px-1.5" onclick={() => handleRestore(item)}>恢复</Button>
  {#if canDelete('recycle')}
    <Button variant="destructive" size="sm" class="px-1.5" onclick={() => handlePurge(item)}>彻底删除</Button>
  {/if}
{/snippet}
