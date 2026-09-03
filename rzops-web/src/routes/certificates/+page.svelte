<script lang="ts">
  import { goto } from '$app/navigation';
  import { certificatesApi } from '$lib/api/certificates';
  import { providersApi } from '$lib/api/providers';
  import type { CertificateResponse, ListCertificatesQuery } from '$lib/types/certificate';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { certificateStatusOptions, certificateTypeOptions } from '$lib/utils/enum-options';
  import { formatResourceWithStatus, isResourceOffline } from '$lib/utils/resource-status';

  let data = $state<CertificateResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListCertificatesQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let providerMap = $state<Record<string, string>>({});
  let providerStatusMap = $state<Record<string, string>>({});
  let showAdvancedFilter = $state(false);

  let certificateTypeMap = $derived(Object.fromEntries($certificateTypeOptions.map(o => [o.value, o.label])));
  let certificateStatusMap = $derived(Object.fromEntries($certificateStatusOptions.map(o => [o.value, o.label])));

  let activeFilterCount = $derived.by(() => {
    let count = 0;
    if (query.status) count++;
    return count;
  });

  const columns = $derived([
    { key: 'name', label: '名称' , link: (item: CertificateResponse) => `/certificates/${item.id}`, lockVisible: true },
    { key: 'certificate_type', label: '类型', valueMap: certificateTypeMap },
    { key: 'status', label: '状态', valueMap: certificateStatusMap },
    { key: 'lease_end_date', label: '到期日期' },
    { key: 'provider_id', label: '供应商', render: (v: unknown, item: CertificateResponse) => {
      if (!item.provider_id) return '-';
      if (!providerMap[item.provider_id]) return '已删除';
      return formatResourceWithStatus(providerMap[item.provider_id], providerStatusMap[item.provider_id], 'provider');
    }, hideInTable: true },
  ]);

  function getRowClass(item: CertificateResponse): string {
    if (item.provider_id && isResourceOffline('provider', providerStatusMap[item.provider_id])) {
      return 'opacity-60 bg-gray-50';
    }
    return '';
  }

  async function loadData() {
    loading = true;
    try {
      const res = await certificatesApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load certificates:', err);
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    const [provOptions, providerList] = await Promise.all([
      getProviderOptions(),
      providersApi.list({ per_page: 200 }),
    ]);
    Object.assign(providerMap, Object.fromEntries(provOptions.map(o => [o.value, o.label])));
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

  function clearFilter(key: keyof ListCertificatesQuery) {
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

  function handleEdit(item: CertificateResponse) {
    goto(`/certificates/${item.id}/edit`);
  }

  async function handleDelete(item: CertificateResponse) {
    try {
      await certificatesApi.delete(item.id);
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
    } catch (err) {
      console.error('Failed to delete certificate:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '证书' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">证书管理</h1>
    <Button onclick={() => goto('/certificates/new')}>新建证书</Button>
  </div>

  <div class="space-y-3">
    <div class="flex flex-wrap items-center gap-2">
      <Input
        placeholder="搜索名称 / 域名..."
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
            状态: {certificateStatusMap[query.status] ?? query.status}
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
            {#each $certificateStatusOptions as opt}
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
    {getRowClass}
    storageKey="certificates" />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
