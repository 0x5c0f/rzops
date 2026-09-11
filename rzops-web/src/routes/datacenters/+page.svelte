<script lang="ts">
  import { goto } from '$app/navigation';
  import { datacentersApi } from '$lib/api/datacenters';
  import { providersApi } from '$lib/api/providers';
  import type { DataCenterResponse, ListDataCentersQuery } from '$lib/types/datacenter';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { commonStatusOptions, countryOptions, getOptionColor } from '$lib/utils/enum-options';
  import { formatResourceWithStatus } from '$lib/utils/resource-status';
  import { onMount } from 'svelte';
import { canCreate, canUpdate, canDelete } from '$lib/utils/permissions';

  let data = $state<DataCenterResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListDataCentersQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let providerMap = $state<Record<string, string>>({});
  let providerStatusMap = $state<Record<string, string>>({});
  let showAdvancedFilter = $state(false);
  let countryMap = $derived(Object.fromEntries($countryOptions.map(o => [o.value, o.label])));

  let commonStatusMap = $derived(Object.fromEntries($commonStatusOptions.map(o => [o.value, o.label])));

  let activeFilterCount = $derived.by(() => {
    let count = 0;
    if (query.status) count++;
    if (query.country) count++;
    return count;
  });

  const columns = $derived([
    { key: 'name', label: '名称' , link: (item: DataCenterResponse) => `/datacenters/${item.id}`, lockVisible: true },
    { key: 'country', label: '国家', hideBelow: 'md', valueMap: countryMap },
    { key: 'status', label: '状态', valueMap: commonStatusMap, statusBadge: (item: DataCenterResponse) => ({ status: item.status, color: getOptionColor($commonStatusOptions, item.status), label: commonStatusMap[item.status] }) },
    { key: 'provider_id', label: '供应商', hideBelow: 'lg', render: (v: unknown, item: DataCenterResponse) => {
      if (!item.provider_id) return '-';
      if (!providerMap[item.provider_id]) return '已删除';
      return formatResourceWithStatus(providerMap[item.provider_id], providerStatusMap[item.provider_id], 'provider');
    }, hideInTable: true },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string), hideInTable: true },
  ]);

  function getRowClass(item: DataCenterResponse): string {
    if (item.status && item.status !== 'active') {
      return 'text-slate-400';
    }
    return '';
  }

  async function loadData() {
    loading = true;
    try {
      const res = await datacentersApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load datacenters:', err);
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    const [provOptions, providerList] = await Promise.all([
      getProviderOptions(),
      providersApi.list({ per_page: 200 }),
    ]);
    providerMap = Object.fromEntries(provOptions.map(o => [o.value, o.label]));
    providerStatusMap = Object.fromEntries(providerList.data.map((p: {id: string, status?: string}) => [p.id, p.status || 'active']));
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

  function clearFilter(key: keyof ListDataCentersQuery) {
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

  function handleEdit(item: DataCenterResponse) {
    goto(`/datacenters/${item.id}/edit`);
  }

  async function handleDelete(item: DataCenterResponse) {
    try {
      await datacentersApi.delete(item.id);
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
    } catch (err) {
      console.error('Failed to delete datacenter:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '数据中心' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">数据中心管理</h1>
    {#if canCreate('datacenter')}
      <Button onclick={() => goto('/datacenters/new')}>新建数据中心</Button>
    {/if}
  </div>

  <div class="space-y-3">
    <div class="flex flex-wrap items-center gap-2">
      <Input
        placeholder="搜索名称 / 地址..."
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
            状态: {commonStatusMap[query.status] ?? query.status}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('status')}>×</button>
          </span>
        {/if}
        {#if query.country}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            国家: {countryMap[query.country] ?? query.country}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('country')}>×</button>
          </span>
        {/if}
      </div>
    {/if}

    {#if showAdvancedFilter}
      <div class="grid gap-3 rounded-lg border p-4 md:grid-cols-2">
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
            {#each $commonStatusOptions as opt}
              <option value={opt.value}>{opt.label}</option>
            {/each}
          </select>
        </div>
        <div class="space-y-1">
          <label class="text-xs font-medium text-muted-foreground">国家</label>
          <select
            class="w-full rounded-md border px-3 py-2 text-sm"
            value={query.country ?? ''}
            onchange={(e) => {
              const val = (e.target as HTMLSelectElement).value;
              query = { ...query, country: val || undefined, page: 1 };
              loadData();
            }}
          >
            <option value="">全部</option>
            {#each $countryOptions as opt}
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
    onEdit={canUpdate('datacenter') ? handleEdit : undefined}
    onDelete={canDelete('datacenter') ? handleDelete : undefined}
    {getRowClass}
    storageKey="datacenters" />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
