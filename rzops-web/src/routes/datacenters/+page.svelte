<script lang="ts">
  import { goto } from '$app/navigation';
  import { datacentersApi } from '$lib/api/datacenters';
  import type { DataCenterResponse, ListDataCentersQuery } from '$lib/types/datacenter';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { commonStatusOptions, countryOptions } from '$lib/utils/enum-options';
  import { onMount } from 'svelte';

  let data = $state<DataCenterResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListDataCentersQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let providerMap = $state<Record<string, string>>({});
  let countryMap = $derived(Object.fromEntries($countryOptions.map(o => [o.value, o.label])));

  let commonStatusMap = $derived(Object.fromEntries($commonStatusOptions.map(o => [o.value, o.label])));

  const columns = $derived([
    { key: 'name', label: '名称' , link: (item: DataCenterResponse) => `/datacenters/${item.id}` },
    { key: 'country', label: '国家', valueMap: countryMap },
    { key: 'status', label: '状态', valueMap: commonStatusMap },
    { key: 'provider_id', label: '供应商', valueMap: providerMap },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string) },
  ]);

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

  function handleEdit(item: DataCenterResponse) {
    goto(`/datacenters/${item.id}/edit`);
  }

  async function handleDelete(item: DataCenterResponse) {
    if (!confirm(`确定要删除数据中心 "${item.name}" 吗？`)) return;
    try {
      await datacentersApi.delete(item.id);
      loadData();
    } catch (err) {
      console.error('Failed to delete datacenter:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '数据中心' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">数据中心管理</h1>
    <Button onclick={() => goto('/datacenters/new')}>新建数据中心</Button>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索数据中心..."
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
