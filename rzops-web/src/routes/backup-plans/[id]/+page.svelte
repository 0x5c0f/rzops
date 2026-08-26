<script lang="ts">
  import AttachmentSection from '$lib/components/shared/AttachmentSection.svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { backupPlansApi } from '$lib/api/backup-plans';
  import type { BackupPlanResponse } from '$lib/types/backup_plan';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { onMount } from 'svelte';

  let plan = $state<BackupPlanResponse | null>(null);
  let loading = $state(true);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/backup-plans'); return; }

    try {
      plan = await backupPlansApi.getById(id);
    } catch (err) {
      console.error('Failed to load backup-plan:', err);
      goto('/backup-plans');
    } finally {
      loading = false;
    }
  });

  async function handleDelete() {
    if (!plan) return;
    if (!confirm(`确定要删除备份计划 "${plan.name}" 吗？`)) return;
    try {
      await backupPlansApi.delete(plan.id);
      goto('/backup-plans');
    } catch (err) {
      console.error('Failed to delete backup-plan:', err);
    }
  }
</script>

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '备份计划', href: '/backup-plans' },
    { label: plan?.name || '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if plan}
    <!-- 页面标题 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold">{plan.name}</h1>
        <StatusBadge status={plan.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/backup-plans')}>返回列表</Button>
        <Button onclick={() => goto(`/backup-plans/${plan?.id}/edit`)}>编辑</Button>
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
      </div>
    </div>

    <div class="grid gap-6 lg:grid-cols-2">
      <!-- 基本信息 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>基本信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">名称</dt>
              <dd>{plan.name}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">目标类型</dt>
              <dd>{plan.target_type || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">目标ID</dt>
              <dd class="font-mono">{plan.target_id || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">调度计划</dt>
              <dd class="font-mono">{plan.schedule || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">保留天数</dt>
              <dd>{plan.retention_days ?? '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>
    </div>
    <AttachmentSection targetType="backup_plan" targetId={plan.id} />
  {/if}
</div>
