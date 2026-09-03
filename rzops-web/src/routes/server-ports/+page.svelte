<script lang="ts">
  import { goto } from '$app/navigation';
  import { serverPortsApi } from '$lib/api/server-ports';
  import { serversApi } from '$lib/api/servers';
  import type { ServerPortResponse, ListServerPortsQuery } from '$lib/types/server_port';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';
  import { protocolOptions } from '$lib/utils/enum-options';
  import { formatResourceWithStatus, isResourceOffline } from '$lib/utils/resource-status';

  let data = $state<ServerPortResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListServerPortsQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let serverStatusMap = $state<Record<string, string>>({});
  let protocolMap = $derived(Object.fromEntries($protocolOptions.map(o => [o.value, o.label])));

  const columns = $derived([
    { key: 'service_name', label: '服务名称' , link: (item: ServerPortResponse) => `/server-ports/${item.id}`, lockVisible: true },
    { key: 'protocol', label: '协议', valueMap: protocolMap },
    { key: 'port', label: '端口' },
    {
      key: 'servers',
      label: '服务器',
      render: (v: unknown) => {
        const list = v as { id: string; name: string }[] | null | undefined;
        if (!list || list.length === 0) return '-';
        const shown = list.slice(0, 3).map(s => formatResourceWithStatus(s.name, serverStatusMap[s.id], 'server')).join(', ');
        return list.length > 3 ? `${shown}, … +${list.length - 3}` : shown;
      },
    },
    { key: 'is_enabled', label: '启用', render: (v: unknown) => v ? '是' : '否', hideInTable: true },
  ]);

  function getRowClass(item: ServerPortResponse): string {
    if (!item.is_enabled) {
      return 'opacity-60 bg-gray-50';
    }
    // 多绑定：仅当绑定的服务器非空且全部退役时才标灰（任一在线即保持正常显示）
    const servers = item.servers ?? [];
    if (servers.length > 0 && servers.every(s => isResourceOffline('server', serverStatusMap[s.id]))) {
      return 'opacity-60 bg-gray-50';
    }
    return '';
  }

  async function loadData() {
    loading = true;
    try {
      const res = await serverPortsApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load server ports:', err);
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    const serverList = await serversApi.list({ per_page: 200 });
    serverStatusMap = Object.fromEntries(serverList.data.map((s: {id: string, status: string}) => [s.id, s.status]));
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

  function handleEdit(item: ServerPortResponse) {
    goto(`/server-ports/${item.id}/edit`);
  }

  async function handleDelete(item: ServerPortResponse) {
    try {
      await serverPortsApi.delete(item.id);
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
    } catch (err) {
      console.error('Failed to delete server port:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '服务器端口' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">服务器端口管理</h1>
    <Button onclick={() => goto('/server-ports/new')}>新建服务器端口</Button>
  </div>

  <div class="flex flex-wrap items-center gap-2">
    <Input
      placeholder="搜索服务名 / 端口..."
      class="max-w-sm"
      value={query.q ?? ''}
      oninput={handleSearch}
    />
    <select
      class="w-32 rounded-md border px-3 py-2 text-sm"
      value={query.protocol ?? ''}
      onchange={(e) => {
        const val = (e.target as HTMLSelectElement).value;
        query = { ...query, protocol: val || undefined, page: 1 };
        loadData();
      }}
    >
      <option value="">全部协议</option>
      {#each $protocolOptions as opt}
        <option value={opt.value}>{opt.label}</option>
      {/each}
    </select>
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={handleEdit}
    onDelete={handleDelete}
    {getRowClass}
    storageKey="server-ports" />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
