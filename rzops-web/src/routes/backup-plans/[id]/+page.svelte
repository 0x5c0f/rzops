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
  import { backupTargetTypeOptions } from '$lib/utils/enum-options';
  import ConfirmDialog from '$lib/components/shared/ConfirmDialog.svelte';
import { canUpdate, canDelete } from '$lib/utils/permissions';

  let plan = $state<BackupPlanResponse | null>(null);

  let confirmOpen = $state(false);
  let loading = $state(true);
  let targetTypeMap = $derived(Object.fromEntries($backupTargetTypeOptions.map(o => [o.value, o.label])));

  // 目标类型 → 详情页路由前缀
  const targetRoute: Record<string, string> = {
    server: '/servers/',
    database: '/database-instances/',
    site: '/ops-sites/',
  };

  function targetHref(t: string | null, id: string | null): string | null {
    if (!t || !id) return null;
    const prefix = targetRoute[t];
    return prefix ? `${prefix}${id}` : null;
  }

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

  function handleDelete() {
    if (!plan) return;
    confirmOpen = true;
  }

  async function doDelete() {
    if (!plan) return;
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
        {#if canUpdate('backup_plan')}
        <Button onclick={() => goto(`/backup-plans/${plan?.id}/edit`)}>编辑</Button>
      {/if}
        {#if canDelete('backup_plan')}
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
      {/if}
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
              <dd>{plan.target_type ? (targetTypeMap[plan.target_type] ?? plan.target_type) : '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">关联目标</dt>
              <dd>
                {#if plan.target_name && plan.target_id}
                  <a href={targetHref(plan.target_type, plan.target_id)} class="font-medium text-primary hover:underline">
                    {plan.target_name}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
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

    <ConfirmDialog
      bind:open={confirmOpen}
      title="确认删除"
      description={`确定要删除备份计划「${plan?.name}」吗？此操作可在回收站恢复。`}
      confirmLabel="删除"
      onConfirm={doDelete}
    />
</div>
