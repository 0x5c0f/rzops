<script lang="ts">
  import { goto } from '$app/navigation';
  import { backupPlansApi } from '$lib/api/backup-plans';
  import type { CreateBackupPlanRequest } from '$lib/types/backup_plan';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import BackupPlanForm from '$lib/components/forms/BackupPlanForm.svelte';

  async function handleCreate(data: CreateBackupPlanRequest) {
    await backupPlansApi.create(data);
    goto('/backup-plans');
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '备份计划', href: '/backup-plans' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建备份计划</h1>
  </div>

  <BackupPlanForm submitLabel="创建" onSubmit={handleCreate} />
</div>
