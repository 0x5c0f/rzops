<script lang="ts">
  import { goto } from '$app/navigation';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import type { OpsSiteResponse, ListOpsSitesQuery } from '$lib/types/ops_site';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';
  import { siteStatusOptions, serviceTargetOptions, importanceOptions, environmentOptions } from '$lib/utils/enum-options';

  let data = $state<OpsSiteResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListOpsSitesQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let showAdvancedFilter = $state(false);
  let siteStatusMap = $derived(Object.fromEntries($siteStatusOptions.map(o => [o.value, o.label])));
  let serviceTargetMap = $derived(Object.fromEntries($serviceTargetOptions.map(o => [o.value, o.label])));
  let importanceMap = $derived(Object.fromEntries($importanceOptions.map(o => [o.value, o.label])));
  let environmentMap = $derived(Object.fromEntries($environmentOptions.map(o => [o.value, o.label])));

  let activeFilterCount = $derived.by(() => {
    let count = 0;
    if (query.status) count++;
    if (query.environment) count++;
    if (query.importance) count++;
    return count;
  });

  const columns = $derived([
    { key: 'name', label: '名称' , link: (item: OpsSiteResponse) => `/ops-sites/${item.id}`, lockVisible: true },
    { key: 'url', label: 'URL' },
    { key: 'environment', label: '环境', valueMap: environmentMap },
    { key: 'status', label: '状态', valueMap: siteStatusMap },
    { key: 'service_target', label: '服务目标', valueMap: serviceTargetMap, hideInTable: true },
    { key: 'importance', label: '重要性', valueMap: importanceMap, hideInTable: true },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string), hideInTable: true },
  ]);

  async function loadData() {
    loading = true;
    try {
      const res = await opsSitesApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load ops-sites:', err);
    } finally {
      loading = false;
    }
  }

  onMount(loadData);

  function handleSearch(e: Event) {
    const input = e.target as HTMLInputElement;
    query = { ...query, q: input.value, page: 1 };
    loadData();
  }

  function resetFilters() {
    query = { page: 1, per_page: perPage };
    loadData();
  }

  function clearFilter(key: keyof ListOpsSitesQuery) {
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

  function handleEdit(item: OpsSiteResponse) {
    goto(`/ops-sites/${item.id}/edit`);
  }

  async function handleDelete(item: OpsSiteResponse) {
    try {
      await opsSitesApi.delete(item.id);
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
    } catch (err) {
      console.error('Failed to delete ops-site:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '站点' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">站点管理</h1>
    <Button onclick={() => goto('/ops-sites/new')}>新建站点</Button>
  </div>

  <div class="space-y-3">
    <div class="flex flex-wrap items-center gap-2">
      <Input
        placeholder="搜索名称 / URL..."
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

    {#if activeFilterCount > 0}
      <div class="flex flex-wrap items-center gap-2">
        <span class="text-sm text-muted-foreground">已选条件：</span>
        {#if query.status}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            状态: {siteStatusMap[query.status] ?? query.status}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('status')}>×</button>
          </span>
        {/if}
        {#if query.environment}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            环境: {environmentMap[query.environment] ?? query.environment}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('environment')}>×</button>
          </span>
        {/if}
        {#if query.importance}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            重要性: {importanceMap[query.importance] ?? query.importance}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('importance')}>×</button>
          </span>
        {/if}
      </div>
    {/if}

    {#if showAdvancedFilter}
      <div class="grid gap-3 rounded-lg border p-4 md:grid-cols-3">
        <div class="space-y-1">
          <label class="text-xs font-medium text-muted-foreground">状态</label>
          <select
            class="w-full rounded-md border px-3 py-2 text-sm"
            value={query.status ?? ''}
            onchange={(e) => {
              const val = (e.target as HTMLSelectElement).value;
              query = { ...query, status: val || undefined, page: 1 };
              loadData();
            }}
          >
            <option value="">全部</option>
            {#each $siteStatusOptions as opt}
              <option value={opt.value}>{opt.label}</option>
            {/each}
          </select>
        </div>
        <div class="space-y-1">
          <label class="text-xs font-medium text-muted-foreground">环境</label>
          <select
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
          <label class="text-xs font-medium text-muted-foreground">重要性</label>
          <select
            class="w-full rounded-md border px-3 py-2 text-sm"
            value={query.importance ?? ''}
            onchange={(e) => {
              const val = (e.target as HTMLSelectElement).value;
              query = { ...query, importance: val || undefined, page: 1 };
              loadData();
            }}
          >
            <option value="">全部</option>
            {#each $importanceOptions as opt}
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
    onEdit={handleEdit}
    onDelete={handleDelete}
    storageKey="ops-sites"
  />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
