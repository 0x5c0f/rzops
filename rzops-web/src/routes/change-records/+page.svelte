<script lang="ts">
  import { goto } from '$app/navigation';
  import { changeRecordsApi } from '$lib/api/change-records';
  import type { ChangeRecordResponse, ListChangeRecordsQuery } from '$lib/types/change_record';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import {
    resolveResourceLabel,
    getResourceLink,
    resourceTypeZh,
    labelFromSnapshot,
  } from '$lib/utils/resource-label';
  import { onMount } from 'svelte';
  import { changeTypeOptions } from '$lib/utils/enum-options';

  let data = $state<ChangeRecordResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListChangeRecordsQuery>({ limit: 20, offset: 0 });
  let offset = $derived(query.offset ?? 0);
  let limit = $derived(query.limit ?? 20);
  // rowId -> 资源名称（快照解析不到的再异步查详情）
  let resourceNames = $state<Record<string, string>>({});

  let changeTypeMap = $derived(Object.fromEntries(changeTypeOptions.map(o => [o.value, o.label])));

  const columns = [
    { key: 'change_type', label: '变更类型', valueMap: changeTypeMap },
    {
      key: 'resource_type',
      label: '资源类型',
      display: (item: ChangeRecordResponse) => resourceTypeZh(item.resource_type),
    },
    {
      key: 'resource_id',
      label: '资源',
      display: (item: ChangeRecordResponse) =>
        item.resource_id ? resourceNames[item.id] || '加载中…' : '-',
      link: (item: ChangeRecordResponse) => getResourceLink(item.resource_type, item.resource_id),
    },
    {
      key: 'actor_id',
      label: '操作者',
      display: (item: ChangeRecordResponse) =>
        item.actor_email || (item.actor_id ? item.actor_id.slice(0, 8) : '-'),
    },
    {
      key: 'created_at',
      label: '创建时间',
      display: (item: ChangeRecordResponse) => formatDate(item.created_at),
    },
  ];

  async function resolveNames(rows: ChangeRecordResponse[]) {
    const grouped = new Map<string, { type: string; id: string; rowIds: string[] }>();
    for (const row of rows) {
      // 优先从 after_data 快照取展示名（零请求）
      const fromSnapshot = labelFromSnapshot(row.resource_type, row.after_data);
      if (fromSnapshot) {
        resourceNames[row.id] = fromSnapshot;
        continue;
      }
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
      const res = await changeRecordsApi.list(query);
      data = res.data;
      total = res.count;
      await resolveNames(res.data);
    } catch (err) {
      console.error('Failed to load change records:', err);
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

  function handleView(item: ChangeRecordResponse) {
    goto(`/change-records/${item.id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '变更记录' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">变更记录</h1>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索变更记录..."
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
