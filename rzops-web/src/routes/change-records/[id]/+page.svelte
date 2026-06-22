<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { changeRecordsApi } from '$lib/api/change-records';
  import type { ChangeRecordResponse } from '$lib/types/change_record';
  import { Button } from '$lib/ui/button';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';

  let changeRecord = $state<ChangeRecordResponse | null>(null);
  let loading = $state(true);

  onMount(async () => {
    try {
      changeRecord = await changeRecordsApi.getById($page.params.id ?? "");
    } catch (err) {
      console.error('Failed to load change record:', err);
      goto('/change-records');
    } finally {
      loading = false;
    }
  });
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '变更记录', href: '/change-records' },
    { label: changeRecord?.change_type || '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if changeRecord}
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-semibold">{changeRecord.change_type}</h1>
      <Button variant="outline" onclick={() => goto('/change-records')}>返回列表</Button>
    </div>

    <Card.Root>
      <Card.Header>
        <Card.Title>详细信息</Card.Title>
      </Card.Header>
      <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <div class="space-y-2">
          <Label>变更类型</Label>
          <div class="text-sm">{changeRecord.change_type}</div>
        </div>
        <div class="space-y-2">
          <Label>资源类型</Label>
          <div class="text-sm">{changeRecord.resource_type ?? '-'}</div>
        </div>
        <div class="space-y-2">
          <Label>资源ID</Label>
          <div class="text-sm">{changeRecord.resource_id ?? '-'}</div>
        </div>
        <div class="space-y-2">
          <Label>操作者ID</Label>
          <div class="text-sm">{changeRecord.actor_id ?? '-'}</div>
        </div>
        <div class="space-y-2">
          <Label>创建时间</Label>
          <div class="text-sm">{formatDate(changeRecord.created_at)}</div>
        </div>
        <div class="space-y-2">
          <Label>备注</Label>
          <div class="text-sm">{changeRecord.remarks ?? '-'}</div>
        </div>
        <div class="space-y-2 md:col-span-2 lg:col-span-3">
          <Label>变更前数据</Label>
          <pre class="text-sm bg-muted p-2 rounded overflow-auto">{changeRecord.before_data ? JSON.stringify(changeRecord.before_data, null, 2) : '-'}</pre>
        </div>
        <div class="space-y-2 md:col-span-2 lg:col-span-3">
          <Label>变更后数据</Label>
          <pre class="text-sm bg-muted p-2 rounded overflow-auto">{changeRecord.after_data ? JSON.stringify(changeRecord.after_data, null, 2) : '-'}</pre>
        </div>
      </Card.Content>
    </Card.Root>
  {/if}
</div>
