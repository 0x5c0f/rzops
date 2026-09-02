<script lang="ts">
  import { goto } from '$app/navigation';
  import { serverIpsApi } from '$lib/api/server-ips';
  import type { ServerIpResponse, ListServerIpsQuery } from '$lib/types/server_ip';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
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
    { key: 'ip_address', label: 'IP地址' , link: (item: ServerIpResponse) => `/server-ips/${item.id}`, lockVisible: true },
    { key: 'server_id', label: '服务器', valueMap: serverMap },
    { key: 'ip_type', label: '类型', valueMap: ipTypeMap },
    { key: 'status', label: '状态', valueMap: ipStatusMap },
    { key: 'nic_name', label: '网卡', render: (v: unknown) => v || '-', hideInTable: true },
    { key: 'is_primary', label: '主IP', render: (v: unknown) => v ? '是' : '否', hideInTable: true },
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

  function handlePerPageChange(newPerPage: number) {
    query = { ...query, per_page: newPerPage, page: 1 };
    loadData();
  }

  function handleEdit(item: ServerIpResponse) {
    goto(`/server-ips/${item.id}/edit`);
  }

  async function handleDelete(item: ServerIpResponse) {
    try {
      await serverIpsApi.delete(item.id);
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
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
    onDelete={handleDelete} storageKey="server-ips" />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
