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
    { key: 'id', label: 'ID', render: (v: unknown) => String(v).slice(0, 8) + '…' },
    { key: 'deleted_at', label: '删除时间', render: (v: unknown) => formatDate(v as string) },
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

  <div class="rounded-md border">
    <div class="overflow-x-auto">
      <table class="w-full text-sm">
        <thead>
          <tr class="border-b bg-muted/40 text-left text-muted-foreground">
            <th class="w-[60px] px-3 py-2 text-center">#</th>
            <th class="px-3 py-2 font-medium">资源类型</th>
            <th class="px-3 py-2 font-medium">名称</th>
            <th class="px-3 py-2 font-medium">ID</th>
            <th class="px-3 py-2 font-medium">删除时间</th>
            <th class="px-3 py-2 text-right font-medium">操作</th>
          </tr>
        </thead>
        <tbody>
          {#if loading}
            <tr><td colspan="6" class="px-3 py-8 text-center text-muted-foreground">加载中...</td></tr>
          {:else if data.length === 0}
            <tr><td colspan="6" class="px-3 py-8 text-center text-muted-foreground">回收站为空</td></tr>
          {:else}
            {#each data as item, i}
              <tr class="border-b last:border-0 hover:bg-muted/20">
                <td class="px-3 py-2 text-center text-muted-foreground">{(page - 1) * perPage + i + 1}</td>
                <td class="px-3 py-2">{resourceTypeLabels[item.resource_type] || item.resource_type}</td>
                <td class="px-3 py-2 font-medium">{item.name}</td>
                <td class="px-3 py-2 text-muted-foreground">{item.id.slice(0, 8)}…</td>
                <td class="px-3 py-2">{formatDate(item.deleted_at)}</td>
                <td class="px-3 py-2 text-right">
                  <div class="flex justify-end gap-2">
                    <Button variant="outline" size="sm" onclick={() => handleRestore(item)}>恢复</Button>
                    {#if canDelete('recycle')}
                      <Button variant="destructive" size="sm" onclick={() => handlePurge(item)}>彻底删除</Button>
                    {/if}
                  </div>
                </td>
              </tr>
            {/each}
          {/if}
        </tbody>
      </table>
    </div>
  </div>

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={(p) => { page = p; loadData(); }}
    onPerPageChange={(s) => { perPage = s; page = 1; loadData(); }}
  />
</div>
