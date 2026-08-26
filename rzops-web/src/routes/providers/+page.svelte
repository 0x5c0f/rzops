<script lang="ts">
  import { goto } from '$app/navigation';
  import { providersApi } from '$lib/api/providers';
  import type { ProviderResponse, ListProvidersQuery } from '$lib/types/provider';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import * as Select from '$lib/ui/select';
  import DataTable from '$lib/components/shared/DataTable.svelte';
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

  let providerTypeMap = $derived(Object.fromEntries($providerTypeOptions.map(o => [o.value, o.label])));
  let commonStatusMap = $derived(Object.fromEntries($commonStatusOptions.map(o => [o.value, o.label])));

  const columns = $derived([
    { key: 'name', label: '名称' , link: (item: ProviderResponse) => `/providers/${item.id}` },
    {
      key: 'provider_types', label: '类型',
      render: (v: unknown) => {
        const arr = v as string[];
        return (arr || []).map(t => providerTypeMap[t] || t).join(', ') || '-';
      }
    },
    { key: 'contact_name', label: '联系人' },
    { key: 'contact_phone', label: '电话' },
    { key: 'status', label: '状态', valueMap: commonStatusMap },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string) },
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

  function handlePageChange(newPage: number) {
    query = { ...query, page: newPage };
    loadData();
  }

  function handleEdit(item: ProviderResponse) {
    goto(`/providers/${item.id}/edit`);
  }

  async function handleDelete(item: ProviderResponse) {
    if (!confirm(`确定要删除供应商 "${item.name}" 吗？`)) return;
    try {
      await providersApi.delete(item.id);
      loadData();
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

  <div class="flex gap-2">
    <Input
      placeholder="搜索供应商..."
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
    <span>显示 {Math.min((page - 1) * perPage + 1, total)}-{Math.min(page * perPage, total)} / 共 {total} 条</span>
    <div class="flex gap-2">
      <Button
        variant="outline"
        size="sm"
        disabled={page <= 1}
        onclick={() => handlePageChange(page - 1)}
      >
        上一页
      </Button>
      <Button
        variant="outline"
        size="sm"
        disabled={page * perPage >= total}
        onclick={() => handlePageChange(page + 1)}
      >
        下一页
      </Button>
    </div>
  </div>
</div>
