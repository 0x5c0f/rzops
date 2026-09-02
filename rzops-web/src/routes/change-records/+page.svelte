<script lang="ts">
  import { goto } from '$app/navigation';
  import { changeRecordsApi } from '$lib/api/change-records';
  import type { ChangeRecordResponse, ListChangeRecordsQuery } from '$lib/types/change_record';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import * as Select from '$lib/ui/select';
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
  ];

  let data = $state<ChangeRecordResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let page = $state(1);
  const perPage = 20;
  let offset = $derived((page - 1) * perPage);
  let changeType = $state('');
  let resourceType = $state('');
  let dateFrom = $state('');
  let dateTo = $state('');
  // rowId -> 资源名称（快照解析不到的再异步查详情）
  let resourceNames = $state<Record<string, string>>({});

  let changeTypeMap = $derived(Object.fromEntries(changeTypeOptions.map(o => [o.value, o.label])));
  let changeTypeLabel = $derived(changeTypeMap[changeType] || '变更类型');

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
        const display = label || `${g.id.slice(0, 8)}…`;
        for (const rid of g.rowIds) {
          resourceNames[rid] = display;
        }
      })
    );
  }

  async function loadData() {
    loading = true;
    try {
      const res = await changeRecordsApi.list({
        change_type: changeType || undefined,
        resource_type: resourceType || undefined,
        created_from: dateFrom ? `${dateFrom}T00:00:00Z` : undefined,
        created_to: dateTo ? `${dateTo}T23:59:59Z` : undefined,
        page,
        per_page: perPage,
      });
      data = res.data;
      total = res.count;
      await resolveNames(res.data);
    } catch (err) {
      console.error('Failed to load change records:', err);
    } finally {
      loading = false;
    }
  }

  function resetAndLoad() {
    page = 1;
    loadData();
  }

  onMount(loadData);

  function handleView(item: ChangeRecordResponse) {
    goto(`/change-records/${item.id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '变更记录' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">变更记录</h1>
  </div>

  <div class="flex flex-wrap items-center gap-2">
    <Select.Root type="single" bind:value={changeType} onValueChange={resetAndLoad}>
      <Select.Trigger class="w-36">{changeTypeLabel}</Select.Trigger>
      <Select.Content>
        <Select.Item value="">全部变更类型</Select.Item>
        {#each changeTypeOptions as o}
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
    <Button variant="outline" size="sm" onclick={() => { changeType = ''; resourceType = ''; dateFrom = ''; dateTo = ''; resetAndLoad(); }}>
      重置
    </Button>
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={handleView}
    editLabel="详情" storageKey="change-records" />

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
