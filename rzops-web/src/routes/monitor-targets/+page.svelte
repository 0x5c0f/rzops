<script lang="ts">
  import { goto } from '$app/navigation';
  import { monitorTargetsApi } from '$lib/api/monitor-targets';
  import { serversApi } from '$lib/api/servers';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import { domainsApi } from '$lib/api/domains';
  import { certificatesApi } from '$lib/api/certificates';
  import type { MonitorTargetResponse, ListMonitorTargetsQuery } from '$lib/types/monitor_target';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { formatResourceWithStatus, getResourceStatusClass } from '$lib/utils/resource-status';
  import { onMount } from 'svelte';
  import { commonStatusOptions, monitorTypeOptions, assetTargetTypeOptions, getOptionColor } from '$lib/utils/enum-options';
import { canCreate, canUpdate, canDelete } from '$lib/utils/permissions';

  let data = $state<MonitorTargetResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListMonitorTargetsQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let showAdvancedFilter = $state(false);

  let statusMap = $derived(Object.fromEntries($commonStatusOptions.map(o => [o.value, o.label])));
  let monitorTypeMap = $derived(Object.fromEntries($monitorTypeOptions.map(o => [o.value, o.label])));
  let targetTypeMap = $derived(Object.fromEntries($assetTargetTypeOptions.map(o => [o.value, o.label])));

  let activeFilterCount = $derived.by(() => {
    let count = 0;
    if (query.status) count++;
    if (query.target_type) count++;
    return count;
  });

  // 关联目标状态映射
  let serverStatusMap = $state<Record<string, string>>({});
  let databaseStatusMap = $state<Record<string, string>>({});
  let siteStatusMap = $state<Record<string, string>>({});
  let domainStatusMap = $state<Record<string, string>>({});
  let certificateStatusMap = $state<Record<string, string>>({});

  const targetRoute: Record<string, string> = {
    server: '/servers/',
    database: '/database-instances/',
    site: '/ops-sites/',
    domain: '/domains/',
    certificate: '/certificates/',
  };
  function targetHref(item: MonitorTargetResponse): string | null {
    if (!item.target_type || !item.target_id) return null;
    const prefix = targetRoute[item.target_type];
    return prefix ? `${prefix}${item.target_id}` : null;
  }

  function getTargetStatus(item: MonitorTargetResponse): string | undefined {
    if (!item.target_type || !item.target_id) return undefined;
    if (item.target_type === 'server') return serverStatusMap[item.target_id];
    if (item.target_type === 'database') return databaseStatusMap[item.target_id];
    if (item.target_type === 'site') return siteStatusMap[item.target_id];
    if (item.target_type === 'domain') return domainStatusMap[item.target_id];
    if (item.target_type === 'certificate') return certificateStatusMap[item.target_id];
    return undefined;
  }

  function getTargetResourceType(item: MonitorTargetResponse): string {
    if (item.target_type === 'server') return 'server';
    if (item.target_type === 'database') return 'database';
    if (item.target_type === 'site') return 'site';
    if (item.target_type === 'domain') return 'ip';
    if (item.target_type === 'certificate') return 'provider';
    return '';
  }

  const columns = $derived([
    { key: 'name', label: '名称' , link: (item: MonitorTargetResponse) => `/monitor-targets/${item.id}`, lockVisible: true },
    { key: 'target_type', label: '目标类型', valueMap: targetTypeMap },
    { key: 'target_name', label: '关联目标', link: targetHref, render: (v: unknown, item: MonitorTargetResponse) => {
      const status = getTargetStatus(item);
      const resType = getTargetResourceType(item);
      const name = item.target_name || '-';
      if (status && resType) {
        return formatResourceWithStatus(name, status, resType);
      }
      return name;
    }},
    { key: 'monitor_type', label: '监控类型', hideBelow: 'md', valueMap: monitorTypeMap },
    { key: 'status', label: '状态', valueMap: statusMap, statusBadge: (item: MonitorTargetResponse) => ({ status: item.status, color: getOptionColor($commonStatusOptions, item.status), label: statusMap[item.status] }) },
    { key: 'endpoint', label: '端点', hideBelow: 'xl', hideInTable: true },
    { key: 'interval_seconds', label: '间隔(秒)', hideBelow: 'lg', hideInTable: true },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string), hideInTable: true },
  ]);

  async function loadData() {
    loading = true;
    try {
      const [res, servers, databases, sites, domains, certs] = await Promise.all([
        monitorTargetsApi.list(query),
        serversApi.list({ per_page: 200 }),
        databaseInstancesApi.list({ per_page: 200 }),
        opsSitesApi.list({ per_page: 200 }),
        domainsApi.list({ per_page: 200 }),
        certificatesApi.list({ per_page: 200 }),
      ]);
      data = res.data;
      total = res.count;
      serverStatusMap = Object.fromEntries(servers.data.map((s: {id: string, status: string}) => [s.id, s.status]));
      databaseStatusMap = Object.fromEntries(databases.data.map((d: {id: string, status: string}) => [d.id, d.status]));
      siteStatusMap = Object.fromEntries(sites.data.map((s: {id: string, status: string}) => [s.id, s.status]));
      domainStatusMap = Object.fromEntries(domains.data.map((d: {id: string, is_enabled: boolean}) => [d.id, d.is_enabled ? 'enabled' : 'disabled']));
      certificateStatusMap = Object.fromEntries(certs.data.map((c: {id: string, status: string}) => [c.id, c.status]));
    } catch (err) {
      console.error('Failed to load monitor-targets:', err);
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

  function resetFilters() {
    query = { page: 1, per_page: perPage };
    loadData();
  }

  function getRowClass(item: MonitorTargetResponse): string {
    if (item.status === 'disabled' || item.status === 'paused' || item.status === 'inactive') {
      return 'text-slate-400';
    }
    return ''; // archived 已归档不标色，仅排末尾
  }

  function clearFilter(key: keyof ListMonitorTargetsQuery) {
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

  function handleEdit(item: MonitorTargetResponse) {
    goto(`/monitor-targets/${item.id}/edit`);
  }

  async function handleDelete(item: MonitorTargetResponse) {
    try {
      await monitorTargetsApi.delete(item.id);
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
    } catch (err) {
      console.error('Failed to delete monitor-target:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '监控目标' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">监控目标管理</h1>
    {#if canCreate('monitor_target')}
      <Button onclick={() => goto('/monitor-targets/new')}>新建监控目标</Button>
    {/if}
  </div>

  <div class="space-y-3">
    <div class="flex flex-wrap items-center gap-2">
      <Input
        placeholder="搜索名称 / 目标..."
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
            状态: {statusMap[query.status] ?? query.status}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('status')}>×</button>
          </span>
        {/if}
        {#if query.target_type}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            目标类型: {targetTypeMap[query.target_type] ?? query.target_type}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('target_type')}>×</button>
          </span>
        {/if}
      </div>
    {/if}

    {#if showAdvancedFilter}
      <div class="grid gap-3 rounded-lg border p-4 md:grid-cols-2">
        <div class="space-y-1">
          <label for="f-1" class="text-xs font-medium text-muted-foreground">状态</label>
          <select id="f-1"
            class="w-full rounded-md border px-3 py-2 text-sm"
            value={query.status ?? ''}
            onchange={(e) => {
              const val = (e.target as HTMLSelectElement).value;
              query = { ...query, status: val || undefined, page: 1 };
              loadData();
            }}
          >
            <option value="">全部</option>
            {#each $commonStatusOptions as opt}
              <option value={opt.value}>{opt.label}</option>
            {/each}
          </select>
        </div>
        <div class="space-y-1">
          <label for="f-2" class="text-xs font-medium text-muted-foreground">目标类型</label>
          <select id="f-2"
            class="w-full rounded-md border px-3 py-2 text-sm"
            value={query.target_type ?? ''}
            onchange={(e) => {
              const val = (e.target as HTMLSelectElement).value;
              query = { ...query, target_type: val || undefined, page: 1 };
              loadData();
            }}
          >
            <option value="">全部</option>
            {#each $assetTargetTypeOptions as opt}
              <option value={opt.value}>{opt.label}</option>
            {/each}
          </select>
        </div>
      </div>
    {/if}
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={canUpdate('monitor_target') ? handleEdit : undefined}
    onDelete={canDelete('monitor_target') ? handleDelete : undefined}
    {getRowClass}
    storageKey="monitor-targets" />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
