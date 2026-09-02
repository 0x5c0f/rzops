<script lang="ts">
  import { goto } from '$app/navigation';
  import { serversApi } from '$lib/api/servers';
  import type { ServerResponse, ListServersQuery } from '$lib/types/server';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { getDataCenterOptions, getProviderOptions } from '$lib/utils/entity-options';
  import { serverStatusOptions, serverTypeOptions } from '$lib/utils/enum-options';
  import { onMount } from 'svelte';

  let data = $state<ServerResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListServersQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let dataCenterMap = $state<Record<string, string>>({});
  let providerMap = $state<Record<string, string>>({});

  let serverTypeMap = $derived(Object.fromEntries($serverTypeOptions.map(o => [o.value, o.label])));
  let serverStatusMap = $derived(Object.fromEntries($serverStatusOptions.map(o => [o.value, o.label])));

  const columns = $derived([
    { key: 'name', label: '名称' , link: (item: ServerResponse) => `/servers/${item.id}` },
    { key: 'primary_ip', label: '主IP' },
    { key: 'server_type', label: '类型', valueMap: serverTypeMap },
    { key: 'status', label: '状态', valueMap: serverStatusMap },
    { key: 'data_center_id', label: '数据中心', valueMap: dataCenterMap },
    { key: 'isp_provider_id', label: 'ISP供应商', valueMap: providerMap },
    { key: 'server_provider_id', label: '服务器供应商', valueMap: providerMap },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string) },
  ]);

  async function loadData() {
    loading = true;
    try {
      const res = await serversApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load servers:', err);
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    const [dcOptions, provOptions] = await Promise.all([
      getDataCenterOptions(),
      getProviderOptions(),
    ]);
    dataCenterMap = Object.fromEntries(dcOptions.map(o => [o.value, o.label]));
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

  function handleEdit(item: ServerResponse) {
    goto(`/servers/${item.id}/edit`);
  }

  async function handleDelete(item: ServerResponse) {
    try {
      await serversApi.delete(item.id);
      // 本地移除，避免重新请求整个列表
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      // 如果当前页为空且不是第一页，回到上一页
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
    } catch (err) {
      console.error('Failed to delete server:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '服务器' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">服务器管理</h1>
    <Button onclick={() => goto('/servers/new')}>新建服务器</Button>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索服务器..."
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

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
