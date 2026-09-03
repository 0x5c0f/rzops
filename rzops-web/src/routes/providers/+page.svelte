<script lang="ts">
  import { goto } from '$app/navigation';
  import { providersApi } from '$lib/api/providers';
  import type { ProviderResponse, ListProvidersQuery } from '$lib/types/provider';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import * as Select from '$lib/ui/select';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { providerTypeOptions, commonStatusOptions } from '$lib/utils/enum-options';
  import { onMount } from 'svelte';

  let data = $state<ProviderResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListProvidersQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let showAdvancedFilter = $state(false);

  let providerTypeMap = $derived(Object.fromEntries($providerTypeOptions.map(o => [o.value, o.label])));
  let commonStatusMap = $derived(Object.fromEntries($commonStatusOptions.map(o => [o.value, o.label])));

  let activeFilterCount = $derived.by(() => {
    let count = 0;
    if (query.status) count++;
    return count;
  });

  const columns = $derived([
    { key: 'name', label: '名称' , link: (item: ProviderResponse) => `/providers/${item.id}`, lockVisible: true },
    {
      key: 'provider_types', label: '类型',
      render: (v: unknown) => {
        const arr = v as string[];
        return (arr || []).map(t => providerTypeMap[t] || t).join(', ') || '-';
      }
    },
    { key: 'status', label: '状态', valueMap: commonStatusMap },
    { key: 'contact_name', label: '联系人', hideInTable: true },
    { key: 'contact_phone', label: '电话', hideInTable: true },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string), hideInTable: true },
  ]);

  async function loadData() {
    loading = true;
    try {
      const res = await providersApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load providers:', err);
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

  function clearFilter(key: keyof ListProvidersQuery) {
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

  function handleEdit(item: ProviderResponse) {
    goto(`/providers/${item.id}/edit`);
  }

  async function handleDelete(item: ProviderResponse) {
    try {
      await providersApi.delete(item.id);
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
    } catch (err) {
      console.error('Failed to delete provider:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '供应商' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">供应商管理</h1>
    <Button onclick={() => goto('/providers/new')}>新建供应商</Button>
  </div>

  <div class="space-y-3">
    <div class="flex flex-wrap items-center gap-2">
      <Input
        placeholder="搜索名称 / 联系人..."
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
      </div>
    {/if}
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={handleEdit}
    onDelete={handleDelete} storageKey="providers" />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
