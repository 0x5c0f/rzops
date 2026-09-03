<script lang="ts">
  import { goto } from '$app/navigation';
  import { auditLogsApi } from '$lib/api/audit-logs';
  import type { AuditLogResponse, ListAuditLogsQuery } from '$lib/types/audit_log';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import * as Select from '$lib/ui/select';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { getResourceLink, resourceTypeZh } from '$lib/utils/resource-label';
  import { onMount } from 'svelte';

  const resourceTypeOptions = [
    { label: '供应商', value: 'provider' },
    { label: '数据中心', value: 'datacenter' },
    { label: '服务器', value: 'server' },
    { label: '服务器IP', value: 'server_ip' },
    { label: '服务器端口', value: 'server_port' },
    { label: '域名', value: 'domain' },
    { label: '证书', value: 'certificate' },
    { label: '证书域名绑定', value: 'certificate_domain' },
    { label: '数据库实例', value: 'database_instance' },
    { label: '站点', value: 'ops_site' },
    { label: '凭据', value: 'credential' },
    { label: '备份计划', value: 'backup_plan' },
    { label: '监控目标', value: 'monitor_target' },
    { label: '合同', value: 'contract' },
    { label: '附件', value: 'attachment' },
    { label: '站点关联', value: 'site_relation' },
    { label: '认证', value: 'auth' },
  ];

  const actionOptions = [
    { label: '创建', value: 'create' },
    { label: '更新', value: 'update' },
    { label: '删除', value: 'delete' },
    { label: '其他', value: 'other' },
  ];

  let data = $state<AuditLogResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let page = $state(1);
  const perPage = 20;
  let offset = $derived((page - 1) * perPage);
  let action = $state('');
  let resourceType = $state('');
  let dateFrom = $state('');
  let dateTo = $state('');

  const columns = [
    {
      key: 'action',
      label: '操作',
      display: (item: AuditLogResponse) =>
        ({ create: '创建', update: '更新', delete: '删除', other: '其他' } as Record<string, string>)[item.action] || item.action,
    },
    {
      key: 'resource_type',
      label: '资源类型',
      display: (item: AuditLogResponse) => resourceTypeZh(item.resource_type),
    },
    {
      key: 'resource_id',
      label: '资源',
      display: (item: AuditLogResponse) =>
        item.resource_id ? item.resource_name || `${item.resource_id.slice(0, 8)}…` : '-',
      link: (item: AuditLogResponse) => getResourceLink(item.resource_type, item.resource_id),
    },
    {
      key: 'actor_id',
      label: '操作者',
      display: (item: AuditLogResponse) =>
        item.actor_email || (item.actor_id ? item.actor_id.slice(0, 8) : '-'),
    },
    {
      key: 'created_at',
      label: '创建时间',
      display: (item: AuditLogResponse) => formatDate(item.created_at),
    },
  ];

  async function loadData() {
    loading = true;
    try {
      const res = await auditLogsApi.list({
        action: action || undefined,
        resource_type: resourceType || undefined,
        created_from: dateFrom ? `${dateFrom}T00:00:00Z` : undefined,
        created_to: dateTo ? `${dateTo}T23:59:59Z` : undefined,
        page,
        per_page: perPage,
      });
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load audit logs:', err);
    } finally {
      loading = false;
    }
  }

  function resetAndLoad() {
    page = 1;
    loadData();
  }

  onMount(loadData);

  function handleView(item: AuditLogResponse) {
    goto(`/audit-logs/${item.id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '审计日志' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">审计日志</h1>
  </div>

  <div class="flex flex-wrap items-center gap-2">
    <Select.Root type="single" bind:value={action} onValueChange={resetAndLoad}>
      <Select.Trigger class="w-32">{action ? actionOptions.find(o => o.value === action)?.label || action : '全部操作'}</Select.Trigger>
      <Select.Content>
        <Select.Item value="">全部操作</Select.Item>
        {#each actionOptions as o}
          <Select.Item value={o.value}>{o.label}</Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>

    <Select.Root type="single" bind:value={resourceType} onValueChange={resetAndLoad}>
      <Select.Trigger class="w-40">{resourceType ? resourceTypeOptions.find(o => o.value === resourceType)?.label || resourceType : '全部资源类型'}</Select.Trigger>
      <Select.Content>
        <Select.Item value="">全部资源类型</Select.Item>
        {#each resourceTypeOptions as o}
          <Select.Item value={o.value}>{o.label}</Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>

    <Input
      type="date"
      class="w-44"
      bind:value={dateFrom}
      onchange={resetAndLoad}
    />
    <span class="text-sm text-muted-foreground">至</span>
    <Input
      type="date"
      class="w-44"
      bind:value={dateTo}
      onchange={resetAndLoad}
    />
    <Button variant="outline" size="sm" onclick={() => { action = ''; resourceType = ''; dateFrom = ''; dateTo = ''; resetAndLoad(); }}>
      重置
    </Button>
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={handleView}
    editLabel="详情" storageKey="audit-logs"
    {page}
    {perPage} />

  <div class="flex items-center justify-between text-sm text-muted-foreground">
    <span>显示 {total === 0 ? 0 : offset + 1}-{Math.min(offset + perPage, total)} / 共 {total} 条</span>
    <div class="flex gap-2">
      <Button
        variant="outline"
        size="sm"
        disabled={page <= 1}
        onclick={() => { page -= 1; loadData(); }}
      >
        上一页
      </Button>
      <Button
        variant="outline"
        size="sm"
        disabled={offset + perPage >= total}
        onclick={() => { page += 1; loadData(); }}
      >
        下一页
      </Button>
    </div>
  </div>
</div>
