<script lang="ts">
  import { goto } from '$app/navigation';
  import { auditLogsApi } from '$lib/api/audit-logs';
  import type { AuditLogResponse, ListAuditLogsQuery } from '$lib/types/audit_log';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';

  let data = $state<AuditLogResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListAuditLogsQuery>({ limit: 20, offset: 0 });
  let offset = $derived(query.offset ?? 0);
  let limit = $derived(query.limit ?? 20);

  const columns = [
    { key: 'action', label: '操作' },
    { key: 'resource_type', label: '资源类型' },
    { key: 'resource_id', label: '资源ID' },
    { key: 'actor_id', label: '操作者' },
    { key: 'created_at', label: '创建时间' },
  ];

  async function loadData() {
    loading = true;
    try {
      const res = await auditLogsApi.list(query);
      data = res.data;
      total = res.count;
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
