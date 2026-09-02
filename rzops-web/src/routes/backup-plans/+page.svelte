<script lang="ts">
  import { goto } from '$app/navigation';
  import { backupPlansApi } from '$lib/api/backup-plans';
  import { serversApi } from '$lib/api/servers';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import type { BackupPlanResponse, ListBackupPlansQuery } from '$lib/types/backup_plan';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { formatResourceWithStatus, getResourceStatusClass } from '$lib/utils/resource-status';
  import { onMount } from 'svelte';
  import { commonStatusOptions, backupTargetTypeOptions } from '$lib/utils/enum-options';

  let data = $state<BackupPlanResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListBackupPlansQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);

  let statusMap = $derived(Object.fromEntries($commonStatusOptions.map(o => [o.value, o.label])));
  let targetTypeMap = $derived(Object.fromEntries($backupTargetTypeOptions.map(o => [o.value, o.label])));

  // 关联目标状态映射
  let serverStatusMap = $state<Record<string, string>>({});
  let databaseStatusMap = $state<Record<string, string>>({});
  let siteStatusMap = $state<Record<string, string>>({});

  const targetRoute: Record<string, string> = {
    server: '/servers/',
    database: '/database-instances/',
    site: '/ops-sites/',
  };
  function targetHref(item: BackupPlanResponse): string | null {
    if (!item.target_type || !item.target_id) return null;
    const prefix = targetRoute[item.target_type];
    return prefix ? `${prefix}${item.target_id}` : null;
  }

  function getTargetStatus(item: BackupPlanResponse): string | undefined {
    if (!item.target_type || !item.target_id) return undefined;
    if (item.target_type === 'server') return serverStatusMap[item.target_id];
    if (item.target_type === 'database') return databaseStatusMap[item.target_id];
    if (item.target_type === 'site') return siteStatusMap[item.target_id];
    return undefined;
  }

  function getTargetResourceType(item: BackupPlanResponse): string {
    if (item.target_type === 'server') return 'server';
    if (item.target_type === 'database') return 'database';
    if (item.target_type === 'site') return 'site';
    return '';
  }

  const columns = [
    { key: 'name', label: '名称' , link: (item: BackupPlanResponse) => `/backup-plans/${item.id}`, lockVisible: true },
    { key: 'target_type', label: '目标类型', valueMap: targetTypeMap },
    { key: 'target_name', label: '关联目标', link: targetHref, render: (v: unknown, item: BackupPlanResponse) => {
      const status = getTargetStatus(item);
      const resType = getTargetResourceType(item);
      const name = item.target_name || '-';
      if (status && resType) {
        return formatResourceWithStatus(name, status, resType);
      }
      return name;
    }},
    { key: 'status', label: '状态', valueMap: statusMap },
    { key: 'schedule', label: '调度计划', hideInTable: true },
    { key: 'retention_days', label: '保留天数', hideInTable: true },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string), hideInTable: true },
  ];

  async function loadData() {
    loading = true;
    try {
      const [res, servers, databases, sites] = await Promise.all([
        backupPlansApi.list(query),
        serversApi.list({ per_page: 200 }),
        databaseInstancesApi.list({ per_page: 200 }),
        opsSitesApi.list({ per_page: 200 }),
      ]);
      data = res.data;
      total = res.count;
      serverStatusMap = Object.fromEntries(servers.data.map((s: {id: string, status: string}) => [s.id, s.status]));
      databaseStatusMap = Object.fromEntries(databases.data.map((d: {id: string, status: string}) => [d.id, d.status]));
      siteStatusMap = Object.fromEntries(sites.data.map((s: {id: string, status: string}) => [s.id, s.status]));
    } catch (err) {
      console.error('Failed to load backup-plans:', err);
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

  function handlePerPageChange(newPerPage: number) {
    query = { ...query, per_page: newPerPage, page: 1 };
    loadData();
  }

  function handleEdit(item: BackupPlanResponse) {
    goto(`/backup-plans/${item.id}/edit`);
  }

  async function handleDelete(item: BackupPlanResponse) {
    try {
      await backupPlansApi.delete(item.id);
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
    } catch (err) {
      console.error('Failed to delete backup-plan:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '备份计划' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">备份计划管理</h1>
    <Button onclick={() => goto('/backup-plans/new')}>新建备份计划</Button>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索备份计划..."
      class="max-w-sm"
      oninput={handleSearch}
    />
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={handleEdit}
    onDelete={handleDelete} storageKey="backup-plans" />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
