<script lang="ts">
  import { goto } from '$app/navigation';
  import { domainsApi } from '$lib/api/domains';
  import type { DomainResponse, ListDomainsQuery } from '$lib/types/domain';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { onMount } from 'svelte';

  let data = $state<DomainResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListDomainsQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let providerMap = $state<Record<string, string>>({});

  const columns = $derived([
    { key: 'domain_name', label: '域名' , link: (item: DomainResponse) => `/domains/${item.id}`, lockVisible: true },
    { key: 'provider_id', label: '注册商', valueMap: providerMap },
    { key: 'expiry_date', label: '到期日期', render: (v: unknown) => formatDate(v as string) },
    { key: 'is_enabled', label: '启用状态', render: (v: unknown) => (v ? '启用' : '停用') },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string), hideInTable: true },
  ]);

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
    const provOptions = await getProviderOptions();
    providerMap = Object.fromEntries(provOptions.map(o => [o.value, o.label]));
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

  <div class="flex gap-2">
    <Input
      placeholder="搜索域名..."
      class="max-w-sm"
      oninput={handleSearch}
    />
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={handleEdit}
    onDelete={handleDelete} storageKey="domains" />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
