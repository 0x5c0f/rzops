<script lang="ts">
  import { goto } from '$app/navigation';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import type { DatabaseInstanceResponse, ListDatabaseInstancesQuery } from '$lib/types/database_instance';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';
  import { getServerOptions } from '$lib/utils/entity-options';
  import { databaseTypeOptions, databaseStatusOptions, importanceOptions } from '$lib/utils/enum-options';

  let data = $state<DatabaseInstanceResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListDatabaseInstancesQuery>({ limit: 20, offset: 0 });
  let offset = $derived(query.offset ?? 0);
  let limit = $derived(query.limit ?? 20);
  let serverMap = $state<Record<string, string>>({});
  let dbTypeMap = $derived(Object.fromEntries(databaseTypeOptions.map(o => [o.value, o.label])));
  let dbStatusMap = $derived(Object.fromEntries(databaseStatusOptions.map(o => [o.value, o.label])));
  let importanceMap = $derived(Object.fromEntries(importanceOptions.map(o => [o.value, o.label])));

  const columns = $derived([
    { key: 'name', label: '名称' },
    { key: 'db_type', label: '数据库类型', valueMap: dbTypeMap },
    { key: 'server_id', label: '服务器', valueMap: serverMap },
    { key: 'status', label: '状态', valueMap: dbStatusMap },
    { key: 'importance', label: '重要性', valueMap: importanceMap },
  ]);

  async function loadData() {
    loading = true;
    try {
      const res = await databaseInstancesApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load database instances:', err);
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    const srvOptions = await getServerOptions();
    Object.assign(serverMap, Object.fromEntries(srvOptions.map(o => [o.value, o.label])));
    await loadData();
  });

  function handleSearch(e: Event) {
    const input = e.target as HTMLInputElement;
    query = { ...query, q: input.value, offset: 0 };
    loadData();
  }

  function handlePageChange(newOffset: number) {
    query = { ...query, offset: newOffset };
    loadData();
  }

  function handleEdit(item: DatabaseInstanceResponse) {
    goto(`/database-instances/${item.id}`);
  }

  async function handleDelete(item: DatabaseInstanceResponse) {
    if (!confirm(`确定要删除数据库实例 "${item.name}" 吗？`)) return;
    try {
      await databaseInstancesApi.delete(item.id);
      loadData();
    } catch (err) {
      console.error('Failed to delete database instance:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '数据库实例' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">数据库实例管理</h1>
    <Button onclick={() => goto('/database-instances/new')}>新建数据库实例</Button>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索数据库实例..."
      class="max-w-sm"
      oninput={handleSearch}
    />
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={handleEdit}
    onDelete={handleDelete}
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
