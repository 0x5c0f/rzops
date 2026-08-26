<script lang="ts">
  import { goto } from '$app/navigation';
  import { serverIpsApi } from '$lib/api/server-ips';
  import type { ServerIpResponse, ListServerIpsQuery } from '$lib/types/server_ip';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';
  import { getServerOptions } from '$lib/utils/entity-options';
  import { ipStatusOptions, ipTypeOptions } from '$lib/utils/enum-options';

  let data = $state<ServerIpResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListServerIpsQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let serverMap = $state<Record<string, string>>({});
  let ipTypeMap = $derived(Object.fromEntries($ipTypeOptions.map(o => [o.value, o.label])));
  let ipStatusMap = $derived(Object.fromEntries($ipStatusOptions.map(o => [o.value, o.label])));

  const columns = $derived([
    { key: 'ip_address', label: 'IP地址' , link: (item: ServerIpResponse) => `/server-ips/${item.id}` },
    { key: 'ip_type', label: '类型', valueMap: ipTypeMap },
    { key: 'server_id', label: '服务器', valueMap: serverMap },
    { key: 'is_primary', label: '主IP', render: (v: unknown) => v ? '是' : '否' },
    { key: 'status', label: '状态', valueMap: ipStatusMap },
  ]);

  async function loadData() {
    loading = true;
    try {
      const res = await serverIpsApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load server IPs:', err);
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    const srvOptions = await getServerOptions();
    Object.assign(serverMap, Object.fromEntries(srvOptions.map(o => [o.value, o.label])));
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

  function handleEdit(item: ServerIpResponse) {
    goto(`/server-ips/${item.id}/edit`);
  }

  async function handleDelete(item: ServerIpResponse) {
    if (!confirm(`确定要删除服务器IP "${item.ip_address}" 吗？`)) return;
    try {
      await serverIpsApi.delete(item.id);
      loadData();
    } catch (err) {
      console.error('Failed to delete server IP:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '服务器IP' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">服务器IP管理</h1>
    <Button onclick={() => goto('/server-ips/new')}>新建服务器IP</Button>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索服务器IP..."
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
