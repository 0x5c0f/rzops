<script lang="ts">
  import { goto } from '$app/navigation';
  import { serversApi } from '$lib/api/servers';
  import type { ServerResponse, ListServersQuery } from '$lib/types/server';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { getDataCenterOptions, getProviderOptions } from '$lib/utils/entity-options';
  import { serverStatusOptions, serverTypeOptions } from '$lib/utils/enum-options';
  import { onMount } from 'svelte';

  let data = $state<ServerResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListServersQuery>({ limit: 20, offset: 0 });
  let offset = $derived(query.offset ?? 0);
  let limit = $derived(query.limit ?? 20);
  let dataCenterMap = $state<Record<string, string>>({});
  let providerMap = $state<Record<string, string>>({});

  let serverTypeMap = $derived(Object.fromEntries($serverTypeOptions.map(o => [o.value, o.label])));
  let serverStatusMap = $derived(Object.fromEntries($serverStatusOptions.map(o => [o.value, o.label])));

  const columns = $derived([
    { key: 'name', label: '名称' },
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
    query = { ...query, q: input.value, offset: 0 };
    loadData();
  }

  function handlePageChange(newOffset: number) {
    query = { ...query, offset: newOffset };
    loadData();
  }

  function handleEdit(item: ServerResponse) {
    goto(`/servers/${item.id}`);
  }

  async function handleDelete(item: ServerResponse) {
    if (!confirm(`确定要删除服务器 "${item.name}" 吗？`)) return;
    try {
      await serversApi.delete(item.id);
      loadData();
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
