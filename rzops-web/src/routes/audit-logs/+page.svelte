<script lang="ts">
  import { goto } from '$app/navigation';
  import { auditLogsApi } from '$lib/api/audit-logs';
  import type { AuditLogResponse, ListAuditLogsQuery } from '$lib/types/audit_log';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { resolveResourceLabel, getResourceLink, resourceTypeZh } from '$lib/utils/resource-label';
  import { onMount } from 'svelte';

  let data = $state<AuditLogResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListAuditLogsQuery>({ limit: 20, offset: 0 });
  let offset = $derived(query.offset ?? 0);
  let limit = $derived(query.limit ?? 20);
  // rowId -> 资源名称
  let resourceNames = $state<Record<string, string>>({});

  const columns = [
    { key: 'action', label: '操作' },
    {
      key: 'resource_type',
      label: '资源类型',
      display: (item: AuditLogResponse) => resourceTypeZh(item.resource_type),
    },
    {
      key: 'resource_id',
      label: '资源',
      display: (item: AuditLogResponse) =>
        item.resource_id ? resourceNames[item.id] || '加载中…' : '-',
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

  async function resolveNames(rows: AuditLogResponse[]) {
    // 按 (type,id) 去重并发解析，避免同一资源重复请求
    const grouped = new Map<string, { type: string; id: string; rowIds: string[] }>();
    for (const row of rows) {
      if (!row.resource_id || !row.resource_type) continue;
      const key = `${row.resource_type}:${row.resource_id}`;
      const entry = grouped.get(key);
      if (entry) {
        entry.rowIds.push(row.id);
      } else {
        grouped.set(key, { type: row.resource_type, id: row.resource_id, rowIds: [row.id] });
      }
    }
    await Promise.allSettled(
      Array.from(grouped.values()).map(async (g) => {
        const label = await resolveResourceLabel(g.type, g.id);
        if (label) {
          for (const rid of g.rowIds) {
            resourceNames[rid] = label;
          }
        }
      })
    );
  }

  async function loadData() {
    loading = true;
    try {
      const res = await auditLogsApi.list(query);
      data = res.data;
      total = res.count;
      await resolveNames(res.data);
    } catch (err) {
      console.error('Failed to load audit logs:', err);
    } finally {
      loading = false;
    }
  }

  onMount(loadData);

  function handleSearch(e: Event) {
    const input = e.target as HTMLInputElement;
    query = { ...query, q: input.value, offset: 0 };
    loadData();
  }

  function handlePageChange(newOffset: number) {
    query = { ...query, offset: newOffset };
    loadData();
  }

  function handleView(item: AuditLogResponse) {
    goto(`/audit-logs/${item.id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '审计日志' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">审计日志</h1>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索审计日志..."
      class="max-w-sm"
      oninput={handleSearch}
    />
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={handleView}
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
