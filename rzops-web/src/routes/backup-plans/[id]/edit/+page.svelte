<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { backupPlansApi } from '$lib/api/backup-plans';
  import type { BackupPlanResponse, CreateBackupPlanRequest } from '$lib/types/backup_plan';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import BackupPlanForm from '$lib/components/forms/BackupPlanForm.svelte';
  import { onMount } from 'svelte';

  let plan = $state<BackupPlanResponse | null>(null);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/backup-plans'); return; }
    try {
      plan = await backupPlansApi.getById(id);
    } catch (err) {
      console.error('Failed to load backup plan:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(p: BackupPlanResponse): CreateBackupPlanRequest {
    return {
      name: p.name,
      target_type: p.target_type ?? '',
      target_id: p.target_id ?? '',
      schedule: p.schedule ?? '',
      retention_days: p.retention_days ?? undefined,
      status: p.status ?? 'active',
      remarks: p.remarks ?? '',
    };
  }

  async function handleUpdate(data: CreateBackupPlanRequest) {
    const id = $page.params.id;
    if (!id) return;
    await backupPlansApi.update(id, data);
    goto(`/backup-plans/${id}`);
    return id;
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '备份计划', href: '/backup-plans' },
    { label: plan?.name || '详情', href: plan ? `/backup-plans/${plan.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑备份计划</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !plan}
    <p class="text-sm text-muted-foreground">加载失败，备份计划可能不存在。</p>
  {:else}
    <BackupPlanForm
      initial={toForm(plan)}
      entityId={plan.id}
      submitLabel="保存"
      onSubmit={handleUpdate}
    />
  {/if}
</div>
