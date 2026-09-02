<script lang="ts">
  import { goto } from '$app/navigation';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import { serversApi } from '$lib/api/servers';
  import type { DatabaseInstanceResponse, ListDatabaseInstancesQuery } from '$lib/types/database_instance';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';
  import { getServerOptions } from '$lib/utils/entity-options';
  import { databaseTypeOptions, databaseStatusOptions, importanceOptions, environmentOptions } from '$lib/utils/enum-options';
  import { formatResourceWithStatus, isResourceOffline } from '$lib/utils/resource-status';

  let data = $state<DatabaseInstanceResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListDatabaseInstancesQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let serverMap = $state<Record<string, string>>({});
  let serverStatusMap = $state<Record<string, string>>({});
  let dbTypeMap = $derived(Object.fromEntries($databaseTypeOptions.map(o => [o.value, o.label])));
  let dbStatusMap = $derived(Object.fromEntries($databaseStatusOptions.map(o => [o.value, o.label])));
  let importanceMap = $derived(Object.fromEntries($importanceOptions.map(o => [o.value, o.label])));
  let environmentMap = $derived(Object.fromEntries($environmentOptions.map(o => [o.value, o.label])));

  const columns = $derived([
    { key: 'name', label: '名称' , link: (item: DatabaseInstanceResponse) => `/database-instances/${item.id}`, lockVisible: true },
    { key: 'db_type', label: '数据库类型', valueMap: dbTypeMap },
    { key: 'environment', label: '环境', valueMap: environmentMap },
    { key: 'server_id', label: '服务器', render: (v: unknown, item: DatabaseInstanceResponse) => {
      if (!item.server_id) return '-';
      if (!serverMap[item.server_id]) return '已删除';
      return formatResourceWithStatus(serverMap[item.server_id], serverStatusMap[item.server_id], 'server');
    }},
    { key: 'status', label: '状态', valueMap: dbStatusMap },
    { key: 'importance', label: '重要性', valueMap: importanceMap, hideInTable: true },
  ]);

  function getRowClass(item: DatabaseInstanceResponse): string {
    if (item.server_id && isResourceOffline('server', serverStatusMap[item.server_id])) {
      return 'opacity-60 bg-gray-50';
    }
    return '';
  }

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
    const [srvOptions, serverList] = await Promise.all([
      getServerOptions(),
      serversApi.list({ per_page: 200 }),
    ]);
    Object.assign(serverMap, Object.fromEntries(srvOptions.map(o => [o.value, o.label])));
    serverStatusMap = Object.fromEntries(serverList.data.map((s: {id: string, status: string}) => [s.id, s.status]));
    await loadData();
  });

  function handleSearch(e: Event) {
    const input = e.target as HTMLInputElement;
    query = { ...query, q: input.value, page: 1 };
    loadData();
  }

  function handlePageChange(newPage: number) {
    query = { ...query, page: newPage };
    loadData();
  }

  function handlePerPageChange(newPerPage: number) {
    query = { ...query, per_page: newPerPage, page: 1 };
    loadData();
  }

  function handleEdit(item: DatabaseInstanceResponse) {
    goto(`/database-instances/${item.id}/edit`);
  }

  async function handleDelete(item: DatabaseInstanceResponse) {
    try {
      await databaseInstancesApi.delete(item.id);
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
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

  <div class="flex flex-wrap gap-2">
    <Input
      placeholder="搜索数据库实例..."
      class="max-w-sm"
      oninput={handleSearch}
    />
    <select
      class="w-32 rounded-md border px-3 py-2 text-sm"
      bind:value={query.environment}
      onchange={() => { query = { ...query, page: 1 }; loadData(); }}
    >
      <option value="">全部环境</option>
      {#each $environmentOptions as opt}
        <option value={opt.value}>{opt.label}</option>
      {/each}
    </select>
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={handleEdit}
    onDelete={handleDelete}
    {getRowClass}
    storageKey="database-instances"
  />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
