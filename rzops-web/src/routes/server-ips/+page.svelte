<script lang="ts">
  import { goto } from '$app/navigation';
  import { serverIpsApi } from '$lib/api/server-ips';
  import { serversApi } from '$lib/api/servers';
  import type { ServerIpResponse, ListServerIpsQuery } from '$lib/types/server_ip';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';
  import { getServerOptions, searchServerOptions } from '$lib/utils/entity-options';
  import { ipStatusOptions, ipTypeOptions } from '$lib/utils/enum-options';
  import { formatResourceWithStatus, isResourceOffline } from '$lib/utils/resource-status';
  import { getOptionColor } from '$lib/utils/enum-options';
  import RemoteSearchSelect from '$lib/components/shared/RemoteSearchSelect.svelte';

  let data = $state<ServerIpResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListServerIpsQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let serverMap = $state<Record<string, string>>({});
  let serverStatusMap = $state<Record<string, string>>({});
  let serverOptions = $state<{ value: string; label: string }[]>([]);
  let showAdvancedFilter = $state(false);
  let ipTypeMap = $derived(Object.fromEntries($ipTypeOptions.map(o => [o.value, o.label])));
  let ipStatusMap = $derived(Object.fromEntries($ipStatusOptions.map(o => [o.value, o.label])));

  let activeFilterCount = $derived.by(() => {
    let count = 0;
    if (query.status) count++;
    if (query.server_id) count++;
    if (query.ip_type) count++;
    return count;
  });

  const columns = $derived([
    { key: 'ip_address', label: 'IP地址' , link: (item: ServerIpResponse) => `/server-ips/${item.id}`, lockVisible: true },
    { key: 'server_id', label: '服务器', render: (v: unknown, item: ServerIpResponse) => {
      if (!item.server_id) return '-';
      if (!serverMap[item.server_id]) return '已删除';
      const name = serverMap[item.server_id];
      const status = serverStatusMap[item.server_id];
      return formatResourceWithStatus(name, status, 'server');
    }},
    { key: 'ip_type', label: '类型', valueMap: ipTypeMap },
    { key: 'status', label: '状态', valueMap: ipStatusMap, statusBadge: (item: ServerIpResponse) => ({ status: item.status, color: getOptionColor($ipStatusOptions, item.status), label: ipStatusMap[item.status] }) },
    { key: 'nic_name', label: '网卡', render: (v: unknown) => v || '-', hideInTable: true },
    { key: 'is_primary', label: '主IP', render: (v: unknown) => v ? '是' : '否', hideInTable: true },
  ]);

  function getRowClass(item: ServerIpResponse): string {
    if (item.status !== 'enabled') {
      return 'text-slate-400';
    }
    if (item.server_id) {
      const status = serverStatusMap[item.server_id];
      if (status === undefined) {
        return 'text-red-500'; // 关联服务器已删除
      }
      if (isResourceOffline('server', status)) {
        return 'text-amber-600'; // 关联服务器已退役
      }
    }
    return '';
  }

  async function loadData() {
    loading = true;
    try {
      // RemoteSearchSelect 清除时 value 为 ''，转换为 undefined
      const queryToSend = { ...query };
      if (queryToSend.server_id === '') queryToSend.server_id = undefined;
      const res = await serverIpsApi.list(queryToSend);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load server IPs:', err);
    } finally {
      loading = false;
    }
  }

  // 监听服务器筛选变化（RemoteSearchSelect 双向绑定，不触发 onchange）
  let prevServerId = $state(query.server_id);
  $effect(() => {
    if (query.server_id !== prevServerId) {
      prevServerId = query.server_id;
      query = { ...query, page: 1 };
      loadData();
    }
  });

  // RemoteSearchSelect 的 displayOptions 只包含已选中的服务器，避免合并后显示全部
  let serverFilterDisplayOptions = $derived(
    query.server_id ? serverOptions.filter(o => o.value === query.server_id) : []
  );

  onMount(async () => {
    const [srvOptions, serverList] = await Promise.all([
      getServerOptions(),
      serversApi.list({ per_page: 200 }),
    ]);
    serverOptions = srvOptions;
    Object.assign(serverMap, Object.fromEntries(srvOptions.map(o => [o.value, o.label])));
    serverStatusMap = Object.fromEntries(serverList.data.map((s: {id: string, status: string}) => [s.id, s.status]));
    await loadData();
  });

  function handleSearch(e: Event) {
    const input = e.target as HTMLInputElement;
    query = { ...query, q: input.value, page: 1 };
    loadData();
  }

  function resetFilters() {
    query = { page: 1, per_page: perPage };
    loadData();
  }

  function clearFilter(key: keyof ListServerIpsQuery) {
    const newQuery = { ...query };
    delete newQuery[key];
    newQuery.page = 1;
    query = newQuery;
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

  <div class="space-y-3">
    <div class="flex flex-wrap items-center gap-2">
      <Input
        placeholder="搜索IP / 网卡..."
        class="max-w-sm"
        value={query.q ?? ''}
        oninput={handleSearch}
      />
      <Button
        variant={showAdvancedFilter ? 'default' : 'outline'}
        size="sm"
        onclick={() => (showAdvancedFilter = !showAdvancedFilter)}
      >
        高级筛选
        {#if activeFilterCount > 0}
          <span class="ml-1 rounded-full bg-primary px-1.5 text-xs text-primary-foreground">{activeFilterCount}</span>
        {/if}
      </Button>
      {#if activeFilterCount > 0}
        <Button variant="ghost" size="sm" onclick={resetFilters}>重置</Button>
      {/if}
    </div>

    {#if activeFilterCount > 0}
      <div class="flex flex-wrap items-center gap-2">
        <span class="text-sm text-muted-foreground">已选条件：</span>
        {#if query.status}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            状态: {ipStatusMap[query.status] ?? query.status}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('status')}>×</button>
          </span>
        {/if}
        {#if query.server_id}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            服务器: {serverMap[query.server_id] ?? query.server_id}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('server_id')}>×</button>
          </span>
        {/if}
        {#if query.ip_type}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            IP类型: {ipTypeMap[query.ip_type] ?? query.ip_type}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('ip_type')}>×</button>
          </span>
        {/if}
      </div>
    {/if}

    {#if showAdvancedFilter}
      <div class="grid gap-3 rounded-lg border p-4 md:grid-cols-3">
        <div class="space-y-1">
          <label class="text-xs font-medium text-muted-foreground">状态</label>
          <select
            class="w-full rounded-md border px-3 py-2 text-sm"
            value={query.status ?? ''}
            onchange={(e) => {
              const val = (e.target as HTMLSelectElement).value;
              query = { ...query, status: val || undefined, page: 1 };
              loadData();
            }}
          >
            <option value="">全部</option>
            {#each $ipStatusOptions as opt}
              <option value={opt.value}>{opt.label}</option>
            {/each}
          </select>
        </div>
        <div class="space-y-1">
          <label class="text-xs font-medium text-muted-foreground">IP类型</label>
          <select
            class="w-full rounded-md border px-3 py-2 text-sm"
            value={query.ip_type ?? ''}
            onchange={(e) => {
              const val = (e.target as HTMLSelectElement).value;
              query = { ...query, ip_type: val || undefined, page: 1 };
              loadData();
            }}
          >
            <option value="">全部</option>
            {#each $ipTypeOptions as opt}
              <option value={opt.value}>{opt.label}</option>
            {/each}
          </select>
        </div>
        <div class="space-y-1">
          <label class="text-xs font-medium text-muted-foreground">服务器</label>
          <RemoteSearchSelect
            bind:value={query.server_id}
            searchFn={searchServerOptions}
            displayOptions={serverFilterDisplayOptions}
            placeholder="全部服务器"
            searchPlaceholder="输入服务器名称搜索..."
          />
        </div>
      </div>
    {/if}
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={handleEdit}
    onDelete={handleDelete}
    {getRowClass}
    storageKey="server-ips" />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
