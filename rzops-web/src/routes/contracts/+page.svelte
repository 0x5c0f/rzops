<script lang="ts">
  import { goto } from '$app/navigation';
  import { contractsApi } from '$lib/api/contracts';
  import { providersApi } from '$lib/api/providers';
  import type { ContractResponse, ListContractsQuery } from '$lib/types/contract';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';
  import { contractStatusOptions, getOptionColor } from '$lib/utils/enum-options';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { formatResourceWithStatus } from '$lib/utils/resource-status';

  let data = $state<ContractResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListContractsQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let providerMap = $state<Record<string, string>>({});
  let providerStatusMap = $state<Record<string, string>>({});

  let statusMap = $derived(Object.fromEntries($contractStatusOptions.map(o => [o.value, o.label])));

  const columns = $derived([
    { key: 'name', label: '名称' },
    { key: 'provider_id', label: '供应商', render: (v: unknown, item: ContractResponse) => {
      if (!item.provider_id) return '-';
      if (!providerMap[item.provider_id]) return '已删除';
      return formatResourceWithStatus(providerMap[item.provider_id], providerStatusMap[item.provider_id], 'provider');
    }},
    { key: 'contract_no', label: '合同编号' },
    { key: 'start_date', label: '开始日期', render: (v: unknown) => formatDate(v as string) },
    { key: 'end_date', label: '结束日期', render: (v: unknown) => formatDate(v as string) },
    { key: 'status', label: '状态', valueMap: statusMap, statusBadge: (item: ContractResponse) => ({ status: item.status, color: getOptionColor($contractStatusOptions, item.status), label: statusMap[item.status] }) },
    { key: 'amount', label: '金额' },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string) },
  ]);

  function getRowClass(item: ContractResponse): string {
    if (item.status && item.status !== 'active') {
      return 'text-slate-400';
    }
    return '';
  }

  async function loadData() {
    loading = true;
    try {
      const res = await contractsApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load contracts:', err);
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

  function handlePageChange(newPage: number) {
    query = { ...query, page: newPage };
    loadData();
  }

  function handlePerPageChange(newPerPage: number) {
    query = { ...query, per_page: newPerPage, page: 1 };
    loadData();
  }

  function handleEdit(item: ContractResponse) {
    goto(`/contracts/${item.id}`);
  }

  async function handleDelete(item: ContractResponse) {
    try {
      await contractsApi.delete(item.id);
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
    } catch (err) {
      console.error('Failed to delete contract:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '合同' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">合同管理</h1>
    <Button onclick={() => goto('/contracts/new')}>新建合同</Button>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索合同..."
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
    {getRowClass}
    storageKey="contracts" />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
