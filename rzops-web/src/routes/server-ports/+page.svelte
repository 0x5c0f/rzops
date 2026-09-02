<script lang="ts">
  import { goto } from '$app/navigation';
  import { serverPortsApi } from '$lib/api/server-ports';
  import type { ServerPortResponse, ListServerPortsQuery } from '$lib/types/server_port';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';
  import { protocolOptions } from '$lib/utils/enum-options';

  let data = $state<ServerPortResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListServerPortsQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let protocolMap = $derived(Object.fromEntries($protocolOptions.map(o => [o.value, o.label])));

  const columns = $derived([
    {
      key: 'servers',
      label: '服务器',
      render: (v: unknown) => {
        const list = v as { id: string; name: string }[] | null | undefined;
        if (!list || list.length === 0) return '-';
        const shown = list.slice(0, 3).map(s => s.name).join(', ');
        return list.length > 3 ? `${shown} +${list.length - 3}` : shown;
      },
    },
    { key: 'protocol', label: '协议', valueMap: protocolMap },
    { key: 'port', label: '端口' },
    { key: 'service_name', label: '服务名称' , link: (item: ServerPortResponse) => `/server-ports/${item.id}` },
    { key: 'is_enabled', label: '启用', render: (v: unknown) => v ? '是' : '否' },
  ]);

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

  function handleEdit(item: ServerPortResponse) {
    goto(`/server-ports/${item.id}/edit`);
  }

  async function handleDelete(item: ServerPortResponse) {
    try {
      await serverPortsApi.delete(item.id);
      loadData();
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

  <div class="flex gap-2">
    <Input
      placeholder="搜索服务器端口..."
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
