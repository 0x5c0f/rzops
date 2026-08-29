<script lang="ts">
  import { goto } from '$app/navigation';
  import { monitorTargetsApi } from '$lib/api/monitor-targets';
  import type { MonitorTargetResponse, ListMonitorTargetsQuery } from '$lib/types/monitor_target';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';
  import { commonStatusOptions, monitorTypeOptions, assetTargetTypeOptions } from '$lib/utils/enum-options';

  let data = $state<MonitorTargetResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListMonitorTargetsQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);

  let statusMap = $derived(Object.fromEntries($commonStatusOptions.map(o => [o.value, o.label])));
  let monitorTypeMap = $derived(Object.fromEntries($monitorTypeOptions.map(o => [o.value, o.label])));
  let targetTypeMap = $derived(Object.fromEntries($assetTargetTypeOptions.map(o => [o.value, o.label])));

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

  const columns = [
    { key: 'name', label: '名称' , link: (item: MonitorTargetResponse) => `/monitor-targets/${item.id}` },
    { key: 'target_type', label: '目标类型', valueMap: targetTypeMap },
    { key: 'target_name', label: '关联目标', link: targetHref, render: (v: unknown, item: MonitorTargetResponse) => (item.target_name || '-') },
    { key: 'monitor_type', label: '监控类型', valueMap: monitorTypeMap },
    { key: 'endpoint', label: '端点' },
    { key: 'interval_seconds', label: '间隔(秒)' },
    { key: 'status', label: '状态', valueMap: statusMap },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string) },
  ];

  async function loadData() {
    loading = true;
    try {
      const res = await monitorTargetsApi.list(query);
      data = res.data;
      total = res.count;
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

  function handlePageChange(newPage: number) {
    query = { ...query, page: newPage };
    loadData();
  }

  function handleEdit(item: MonitorTargetResponse) {
    goto(`/monitor-targets/${item.id}/edit`);
  }

  async function handleDelete(item: MonitorTargetResponse) {
    if (!confirm(`确定要删除监控目标 "${item.name}" 吗？`)) return;
    try {
      await monitorTargetsApi.delete(item.id);
      loadData();
    } catch (err) {
      console.error('Failed to delete monitor-target:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '监控目标' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">监控目标管理</h1>
    <Button onclick={() => goto('/monitor-targets/new')}>新建监控目标</Button>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索监控目标..."
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
