<script lang="ts">
  import { goto } from '$app/navigation';
  import { changeRecordsApi } from '$lib/api/change-records';
  import type { ChangeRecordResponse, ListChangeRecordsQuery } from '$lib/types/change_record';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';
  import { changeTypeOptions } from '$lib/utils/enum-options';

  let data = $state<ChangeRecordResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListChangeRecordsQuery>({ limit: 20, offset: 0 });
  let offset = $derived(query.offset ?? 0);
  let limit = $derived(query.limit ?? 20);

  let changeTypeMap = $derived(Object.fromEntries(changeTypeOptions.map(o => [o.value, o.label])));

  const columns = [
    { key: 'change_type', label: '变更类型', valueMap: changeTypeMap },
    { key: 'resource_type', label: '资源类型' },
    { key: 'resource_id', label: '资源ID' },
    { key: 'actor_id', label: '操作者' },
    { key: 'created_at', label: '创建时间' },
  ];

  async function loadData() {
    loading = true;
    try {
      const res = await changeRecordsApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load change records:', err);
    } finally {
      loading = false;
    }
  }

  onMount(loadData);

  function handleSearch(e: Event) {
    const input = e.target as HTMLInputElement;
    query = { ...query, q: input.value, offset: 0 };
    loadData();
  }

  function handlePageChange(newOffset: number) {
    query = { ...query, offset: newOffset };
    loadData();
  }

  function handleView(item: ChangeRecordResponse) {
    goto(`/change-records/${item.id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '变更记录' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">变更记录</h1>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索变更记录..."
      class="max-w-sm"
      oninput={handleSearch}
    />
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={handleView}
  />

  <div class="flex items-center justify-between text-sm text-muted-foreground">
    <span>显示 {Math.min(offset + 1, total)}-{Math.min(offset + limit, total)} / 共 {total} 条</span>
    <div class="flex gap-2">
      <Button
        variant="outline"
        size="sm"
        disabled={offset === 0}
        onclick={() => handlePageChange(Math.max(0, offset - limit))}
      >
        上一页
      </Button>
      <Button
        variant="outline"
        size="sm"
        disabled={offset + limit >= total}
        onclick={() => handlePageChange(offset + limit)}
      >
        下一页
      </Button>
    </div>
  </div>
</div>
