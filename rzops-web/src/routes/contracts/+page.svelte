<script lang="ts">
  import { goto } from '$app/navigation';
  import { contractsApi } from '$lib/api/contracts';
  import type { ContractResponse, ListContractsQuery } from '$lib/types/contract';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';
  import { contractStatusOptions } from '$lib/utils/enum-options';
  import { getProviderOptions } from '$lib/utils/entity-options';

  let data = $state<ContractResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListContractsQuery>({ limit: 20, offset: 0 });
  let offset = $derived(query.offset ?? 0);
  let limit = $derived(query.limit ?? 20);
  let providerMap = $state<Record<string, string>>({});

  let statusMap = $derived(Object.fromEntries($contractStatusOptions.map(o => [o.value, o.label])));

  const columns = $derived([
    { key: 'name', label: '名称' },
    { key: 'provider_id', label: '供应商', valueMap: providerMap },
    { key: 'contract_no', label: '合同编号' },
    { key: 'start_date', label: '开始日期', render: (v: unknown) => formatDate(v as string) },
    { key: 'end_date', label: '结束日期', render: (v: unknown) => formatDate(v as string) },
    { key: 'status', label: '状态', valueMap: statusMap },
    { key: 'amount', label: '金额' },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string) },
  ]);

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
    const provOptions = await getProviderOptions();
    providerMap = Object.fromEntries(provOptions.map(o => [o.value, o.label]));
    await loadData();
  });

  function handleSearch(e: Event) {
    const input = e.target as HTMLInputElement;
    query = { ...query, q: input.value, offset: 0 };
    loadData();
  }

  function handlePageChange(newOffset: number) {
    query = { ...query, offset: newOffset };
    loadData();
  }

  function handleEdit(item: ContractResponse) {
    goto(`/contracts/${item.id}`);
  }

  async function handleDelete(item: ContractResponse) {
    if (!confirm(`确定要删除合同 "${item.name}" 吗？`)) return;
    try {
      await contractsApi.delete(item.id);
      loadData();
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
  />

  <div class="flex items-center justify-between text-sm text-muted-foreground">
    <span>显示 {Math.min(offset + 1, total)}-{Math.min(offset + limit, total)} / 共 {total} 条</span>
    <div class="flex gap-2">
      <Button
        variant="outline"
        size="sm"
        disabled={offset === 0}
        onclick={() => handlePageChange(Math.max(0, offset - limit))}
      >
        上一页
      </Button>
      <Button
        variant="outline"
        size="sm"
        disabled={offset + limit >= total}
        onclick={() => handlePageChange(offset + limit)}
      >
        下一页
      </Button>
    </div>
  </div>
</div>
