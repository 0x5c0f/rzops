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
  import { formatResourceWithStatus } from '$lib/utils/resource-status';
  import { onMount } from 'svelte';

  let data = $state<DomainResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListDomainsQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let providerMap = $state<Record<string, string>>({});
  let providerStatusMap = $state<Record<string, string>>({});
  let providerOptions = $state<{ value: string; label: string }[]>([]);

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
    if (!item.is_enabled) {
      return 'bg-gray-100';
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
    providerOptions = provOptions;
    providerMap = Object.fromEntries(provOptions.map(o => [o.value, o.label]));
    providerStatusMap = Object.fromEntries(providerList.data.map((p: {id: string, status?: string}) => [p.id, p.status || 'active']));
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

  <div class="flex flex-wrap items-center gap-2">
    <Input
      placeholder="搜索域名..."
      class="max-w-sm"
      value={query.q ?? ''}
      oninput={handleSearch}
    />
    <select
      class="w-44 rounded-md border px-3 py-2 text-sm"
      value={query.provider_id ?? ''}
      onchange={(e) => {
        const val = (e.target as HTMLSelectElement).value;
        query = { ...query, provider_id: val || undefined, page: 1 };
        loadData();
      }}
    >
      <option value="">全部注册商</option>
      {#each providerOptions as opt}
        <option value={opt.value}>{opt.label}</option>
      {/each}
    </select>
    <select
      class="w-32 rounded-md border px-3 py-2 text-sm"
      value={query.is_enabled === undefined ? '' : String(query.is_enabled)}
      onchange={(e) => {
        const val = (e.target as HTMLSelectElement).value;
        query = { ...query, is_enabled: val === '' ? undefined : val === 'true', page: 1 };
        loadData();
      }}
    >
      <option value="">全部状态</option>
      <option value="true">启用</option>
      <option value="false">停用</option>
    </select>
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
