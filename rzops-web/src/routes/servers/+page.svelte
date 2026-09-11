<script lang="ts">
  import { goto } from '$app/navigation';
  import { serversApi } from '$lib/api/servers';
  import type { ServerResponse, ListServersQuery } from '$lib/types/server';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { getDataCenterOptions, getProviderOptions } from '$lib/utils/entity-options';
  import { serverStatusOptions, serverTypeOptions, environmentOptions } from '$lib/utils/enum-options';
  import { getOptionColor } from '$lib/utils/enum-options';
  import { onMount } from 'svelte';
import { canCreate, canUpdate, canDelete } from '$lib/utils/permissions';

  let data = $state<ServerResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListServersQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let dataCenterMap = $state<Record<string, string>>({});
  let providerMap = $state<Record<string, string>>({});
  let dataCenterOptions = $state<{ value: string; label: string }[]>([]);
  let showAdvancedFilter = $state(false);

  let serverTypeMap = $derived(Object.fromEntries($serverTypeOptions.map(o => [o.value, o.label])));
  let serverStatusMap = $derived(Object.fromEntries($serverStatusOptions.map(o => [o.value, o.label])));
  let environmentMap = $derived(Object.fromEntries($environmentOptions.map(o => [o.value, o.label])));

  // 已激活的筛选条件数量
  let activeFilterCount = $derived.by(() => {
    let count = 0;
    if (query.server_type) count++;
    if (query.environment) count++;
    if (query.status) count++;
    if (query.data_center_id) count++;
    if (query.is_database_server !== undefined && query.is_database_server !== null) count++;
    return count;
  });

  const columns = $derived([
    { key: 'name', label: '名称' , link: (item: ServerResponse) => `/servers/${item.id}`, lockVisible: true },
    { key: 'primary_ip', label: '主IP' },
    { key: 'server_type', label: '类型', hideBelow: 'lg', valueMap: serverTypeMap },
    { key: 'environment', label: '环境', hideBelow: 'xl', valueMap: environmentMap },
    { key: 'status', label: '状态', valueMap: serverStatusMap, statusBadge: (item: ServerResponse) => ({ status: item.status, color: getOptionColor($serverStatusOptions, item.status), label: serverStatusMap[item.status] }) },
    { key: 'data_center_id', label: '数据中心', hideBelow: 'xl', valueMap: dataCenterMap, hideInTable: true },
    { key: 'isp_provider_id', label: 'ISP供应商', valueMap: providerMap, hideInTable: true },
    { key: 'server_provider_id', label: '服务器供应商', valueMap: providerMap, hideInTable: true },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string), hideInTable: true },
  ]);

  async function loadData() {
    loading = true;
    try {
      const res = await serversApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load servers:', err);
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    const [dcOptions, provOptions] = await Promise.all([
      getDataCenterOptions(),
      getProviderOptions(),
    ]);
    dataCenterOptions = dcOptions;
    dataCenterMap = Object.fromEntries(dcOptions.map(o => [o.value, o.label]));
    providerMap = Object.fromEntries(provOptions.map(o => [o.value, o.label]));
    await loadData();
  });

  function handleSearch(e: Event) {
    const input = e.target as HTMLInputElement;
    query = { ...query, q: input.value, page: 1 };
    loadData();
  }

  function resetFilters() {
    query = { page: 1, per_page: perPage };
    loadData();
  }

  function clearFilter(key: keyof ListServersQuery) {
    const newQuery = { ...query };
    delete newQuery[key];
    newQuery.page = 1;
    query = newQuery;
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

  function handleEdit(item: ServerResponse) {
    goto(`/servers/${item.id}/edit`);
  }

  async function handleDelete(item: ServerResponse) {
    try {
      await serversApi.delete(item.id);
      // 本地移除，避免重新请求整个列表
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      // 如果当前页为空且不是第一页，回到上一页
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
    } catch (err) {
      console.error('Failed to delete server:', err);
    }
  }

  function getRowClass(item: ServerResponse): string {
    if (item.status === 'retired') {
      return 'text-slate-400';
    }
    return '';
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '服务器' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">服务器管理</h1>
    {#if canCreate('server')}
      <Button onclick={() => goto('/servers/new')}>新建服务器</Button>
    {/if}
  </div>

  <div class="space-y-3">
    <!-- 默认筛选栏 -->
    <div class="flex flex-wrap items-center gap-2">
      <Input
        placeholder="搜索名称 / IP / 资产编号..."
        class="max-w-sm"
        value={query.q ?? ''}
        oninput={handleSearch}
      />
      <Button
        variant={showAdvancedFilter ? 'default' : 'outline'}
        size="sm"
        onclick={() => (showAdvancedFilter = !showAdvancedFilter)}
      >
        高级筛选
        {#if activeFilterCount > 0}
          <span class="ml-1 rounded-full bg-primary px-1.5 text-xs text-primary-foreground">{activeFilterCount}</span>
        {/if}
      </Button>
      {#if activeFilterCount > 0}
        <Button variant="ghost" size="sm" onclick={resetFilters}>重置</Button>
      {/if}
    </div>

    <!-- 已激活筛选条件标签 -->
    {#if activeFilterCount > 0}
      <div class="flex flex-wrap items-center gap-2">
        <span class="text-sm text-muted-foreground">已选条件：</span>
        {#if query.server_type}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            类型: {serverTypeMap[query.server_type] ?? query.server_type}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('server_type')}>×</button>
          </span>
        {/if}
        {#if query.environment}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            环境: {environmentMap[query.environment] ?? query.environment}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('environment')}>×</button>
          </span>
        {/if}
        {#if query.status}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            状态: {serverStatusMap[query.status] ?? query.status}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('status')}>×</button>
          </span>
        {/if}
        {#if query.data_center_id}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            数据中心: {dataCenterMap[query.data_center_id] ?? query.data_center_id}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('data_center_id')}>×</button>
          </span>
        {/if}
        {#if query.is_database_server !== undefined && query.is_database_server !== null}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            数据库服务器: {query.is_database_server ? '是' : '否'}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('is_database_server')}>×</button>
          </span>
        {/if}
      </div>
    {/if}

    <!-- 高级筛选面板 -->
    {#if showAdvancedFilter}
      <div class="grid gap-3 rounded-lg border p-4 md:grid-cols-3 lg:grid-cols-5">
        <div class="space-y-1">
          <label for="f-1" class="text-xs font-medium text-muted-foreground">服务器类型</label>
          <select id="f-1"
            class="w-full rounded-md border px-3 py-2 text-sm"
            value={query.server_type ?? ''}
            onchange={(e) => {
              const val = (e.target as HTMLSelectElement).value;
              query = { ...query, server_type: val || undefined, page: 1 };
              loadData();
            }}
          >
            <option value="">全部</option>
            {#each $serverTypeOptions as opt}
              <option value={opt.value}>{opt.label}</option>
            {/each}
          </select>
        </div>
        <div class="space-y-1">
          <label for="f-2" class="text-xs font-medium text-muted-foreground">环境</label>
          <select id="f-2"
            class="w-full rounded-md border px-3 py-2 text-sm"
            value={query.environment ?? ''}
            onchange={(e) => {
              const val = (e.target as HTMLSelectElement).value;
              query = { ...query, environment: val || undefined, page: 1 };
              loadData();
            }}
          >
            <option value="">全部</option>
            {#each $environmentOptions as opt}
              <option value={opt.value}>{opt.label}</option>
            {/each}
          </select>
        </div>
        <div class="space-y-1">
          <label for="f-3" class="text-xs font-medium text-muted-foreground">状态</label>
          <select id="f-3"
            class="w-full rounded-md border px-3 py-2 text-sm"
            value={query.status ?? ''}
            onchange={(e) => {
              const val = (e.target as HTMLSelectElement).value;
              query = { ...query, status: val || undefined, page: 1 };
              loadData();
            }}
          >
            <option value="">全部</option>
            {#each $serverStatusOptions as opt}
              <option value={opt.value}>{opt.label}</option>
            {/each}
          </select>
        </div>
        <div class="space-y-1">
          <label for="f-4" class="text-xs font-medium text-muted-foreground">数据库服务器</label>
          <select id="f-4"
            class="w-full rounded-md border px-3 py-2 text-sm"
            value={query.is_database_server === undefined ? '' : String(query.is_database_server)}
            onchange={(e) => {
              const val = (e.target as HTMLSelectElement).value;
              query = { ...query, is_database_server: val === '' ? undefined : val === 'true', page: 1 };
              loadData();
            }}
          >
            <option value="">全部</option>
            <option value="true">是</option>
            <option value="false">否</option>
          </select>
        </div>
        <div class="space-y-1">
          <label for="f-5" class="text-xs font-medium text-muted-foreground">数据中心</label>
          <select id="f-5"
            class="w-full rounded-md border px-3 py-2 text-sm"
            value={query.data_center_id ?? ''}
            onchange={(e) => {
              const val = (e.target as HTMLSelectElement).value;
              query = { ...query, data_center_id: val || undefined, page: 1 };
              loadData();
            }}
          >
            <option value="">全部</option>
            {#each dataCenterOptions as opt}
              <option value={opt.value}>{opt.label}</option>
            {/each}
          </select>
        </div>
      </div>
    {/if}
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={canUpdate('server') ? handleEdit : undefined}
    onDelete={canDelete('server') ? handleDelete : undefined}
    {getRowClass}
    storageKey="servers"
  />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
