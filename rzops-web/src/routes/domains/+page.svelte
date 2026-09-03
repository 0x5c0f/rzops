<script lang="ts">
  import { goto } from '$app/navigation';
  import { domainsApi } from '$lib/api/domains';
  import { providersApi } from '$lib/api/providers';
  import type { DomainResponse, ListDomainsQuery } from '$lib/types/domain';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { formatResourceWithStatus, isResourceOffline } from '$lib/utils/resource-status';
  import { onMount } from 'svelte';

  let data = $state<DomainResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListDomainsQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let providerMap = $state<Record<string, string>>({});
  let providerStatusMap = $state<Record<string, string>>({});
  let showAdvancedFilter = $state(false);

  let activeFilterCount = $derived.by(() => {
    let count = 0;
    if (query.is_enabled !== undefined && query.is_enabled !== null) count++;
    return count;
  });

  const columns = $derived([
    { key: 'domain_name', label: '域名' , link: (item: DomainResponse) => `/domains/${item.id}`, lockVisible: true },
    { key: 'provider_id', label: '注册商', render: (v: unknown, item: DomainResponse) => {
      if (!item.provider_id) return '-';
      if (!providerMap[item.provider_id]) return '已删除';
      return formatResourceWithStatus(providerMap[item.provider_id], providerStatusMap[item.provider_id], 'provider');
    }},
    { key: 'expiry_date', label: '到期日期', render: (v: unknown) => formatDate(v as string) },
    { key: 'is_enabled', label: '启用状态', render: (v: unknown) => (v ? '启用' : '停用') },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string), hideInTable: true },
  ]);

  function getRowClass(item: DomainResponse): string {
    if (item.provider_id && isResourceOffline('provider', providerStatusMap[item.provider_id])) {
      return 'opacity-60 bg-gray-50';
    }
    return '';
  }

  async function loadData() {
    loading = true;
    try {
      const res = await domainsApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load domains:', err);
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

  function clearFilter(key: keyof ListDomainsQuery) {
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

  function handleEdit(item: DomainResponse) {
    goto(`/domains/${item.id}/edit`);
  }

  async function handleDelete(item: DomainResponse) {
    try {
      await domainsApi.delete(item.id);
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
    } catch (err) {
      console.error('Failed to delete domain:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '域名' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">域名管理</h1>
    <Button onclick={() => goto('/domains/new')}>新建域名</Button>
  </div>

  <div class="space-y-3">
    <div class="flex flex-wrap items-center gap-2">
      <Input
        placeholder="搜索域名..."
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
        {#if query.is_enabled !== undefined && query.is_enabled !== null}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            启用状态: {query.is_enabled ? '启用' : '停用'}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('is_enabled')}>×</button>
          </span>
        {/if}
      </div>
    {/if}

    {#if showAdvancedFilter}
      <div class="grid gap-3 rounded-lg border p-4 md:grid-cols-2">
        <div class="space-y-1">
          <label class="text-xs font-medium text-muted-foreground">启用状态</label>
          <select
            class="w-full rounded-md border px-3 py-2 text-sm"
            value={query.is_enabled === undefined ? '' : String(query.is_enabled)}
            onchange={(e) => {
              const val = (e.target as HTMLSelectElement).value;
              query = { ...query, is_enabled: val === '' ? undefined : val === 'true', page: 1 };
              loadData();
            }}
          >
            <option value="">全部</option>
            <option value="true">启用</option>
            <option value="false">停用</option>
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
    {getRowClass}
    storageKey="domains" />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
