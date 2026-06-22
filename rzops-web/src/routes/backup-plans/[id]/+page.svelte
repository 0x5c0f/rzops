<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { backupPlansApi } from '$lib/api/backup-plans';
  import type { BackupPlanResponse, UpdateBackupPlanRequest } from '$lib/types/backup_plan';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { onMount } from 'svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { commonStatusOptions } from '$lib/utils/enum-options';

  let plan = $state<BackupPlanResponse | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let form = $state<UpdateBackupPlanRequest>({});

  onMount(async () => {
    try {
      plan = await backupPlansApi.getById($page.params.id ?? "");
      form = {
        name: plan.name,
        target_type: plan.target_type ?? undefined,
        schedule: plan.schedule ?? undefined,
        retention_days: plan.retention_days ?? undefined,
        remarks: plan.remarks ?? undefined,
        status: plan.status,
      };
    } catch (err) {
      console.error('Failed to load backup-plan:', err);
      goto('/backup-plans');
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    if (!plan) return;
    saving = true;
    try {
      await backupPlansApi.update(plan.id, form);
      goto('/backup-plans');
    } catch (err) {
      console.error('Failed to save backup-plan:', err);
    } finally {
      saving = false;
    }
  }

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

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '备份计划', href: '/backup-plans' },
    { label: plan?.name || '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if plan}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{plan.name}</h1>
        <StatusBadge status={plan.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
        <Button onclick={handleSave} disabled={saving}>
          {saving ? '保存中...' : '保存'}
        </Button>
      </div>
    </div>

    <Card.Root>
      <Card.Header>
        <Card.Title>基本信息</Card.Title>
      </Card.Header>
      <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <div class="space-y-2">
          <Label for="name">名称</Label>
          <Input id="name" bind:value={form.name} />
        </div>
        <div class="space-y-2">
          <Label for="target_type">目标类型</Label>
          <Input id="target_type" bind:value={form.target_type} placeholder="数据库 / 文件 / ..." />
        </div>
        <div class="space-y-2">
          <Label for="schedule">调度计划</Label>
          <Input id="schedule" bind:value={form.schedule} placeholder="cron 表达式" />
        </div>
        <div class="space-y-2">
          <Label for="retention_days">保留天数</Label>
          <Input id="retention_days" type="number" bind:value={form.retention_days} />
        </div>
        <FormSelect label="状态" bind:value={form.status} options={commonStatusOptions} />
        <div class="space-y-2 md:col-span-2 lg:col-span-3">
          <Label for="remarks">备注</Label>
          <Input id="remarks" bind:value={form.remarks} />
        </div>
      </Card.Content>
    </Card.Root>
  {/if}
</div>
