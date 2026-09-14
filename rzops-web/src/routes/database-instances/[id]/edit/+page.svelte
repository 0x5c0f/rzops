<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import { backupPlansApi } from '$lib/api/backup-plans';
  import { monitorTargetsApi } from '$lib/api/monitor-targets';
  import type { CreateDatabaseInstanceRequest, DatabaseInstanceResponse } from '$lib/types/database_instance';
  import type { BackupPlanResponse } from '$lib/types/backup_plan';
  import type { MonitorTargetResponse } from '$lib/types/monitor_target';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import DatabaseInstanceForm from '$lib/components/forms/DatabaseInstanceForm.svelte';
  import { onMount } from 'svelte';

  let instance = $state<DatabaseInstanceResponse | null>(null);
  let backupPlans = $state<BackupPlanResponse[]>([]);
  let monitorTargets = $state<MonitorTargetResponse[]>([]);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/database-instances'); return; }
    try {
      const [instData, bpData, mtData] = await Promise.all([
        databaseInstancesApi.getById(id),
        backupPlansApi.list({ target_type: 'database', target_id: id, per_page: 100 }),
        monitorTargetsApi.list({ target_type: 'database', target_id: id, per_page: 100 }),
      ]);
      instance = instData;
      backupPlans = bpData.data;
      monitorTargets = mtData.data;
    } catch (err) {
      console.error('Failed to load database instance:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(d: DatabaseInstanceResponse): CreateDatabaseInstanceRequest {
    return {
      name: d.name,
      db_type: d.db_type,
      server_id: d.server_id ?? '',
      description: d.description ?? '',
      status: d.status || 'active',
      environment: d.environment ?? '',
      is_self_installed: d.is_self_installed ?? false,
      importance: d.importance ?? '',
      is_ops_managed: d.is_ops_managed ?? false,
      port: d.port ?? undefined,
      instance_name: d.instance_name ?? '',
    };
  }

  function toBackupDraft(b: BackupPlanResponse) {
    return {
      id: b.id,
      name: b.name,
      schedule: b.schedule ?? '',
      retention_days: b.retention_days != null ? String(b.retention_days) : '',
      status: b.status,
    };
  }

  function toMonitorDraft(m: MonitorTargetResponse) {
    return {
      id: m.id,
      name: m.name,
      monitor_type: m.monitor_type ?? '',
      endpoint: m.endpoint ?? '',
      interval_seconds: m.interval_seconds != null ? String(m.interval_seconds) : '',
      status: m.status,
    };
  }

  async function handleUpdate(data: CreateDatabaseInstanceRequest) {
    const id = $page.params.id;
    if (!id) return;
    await databaseInstancesApi.update(id, data);
    goto(`/database-instances/${id}`);
    return id;
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '数据库实例', href: '/database-instances' },
    { label: instance?.name || '详情', href: instance ? `/database-instances/${instance.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑数据库实例</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !instance}
    <p class="text-sm text-muted-foreground">加载失败，数据库实例可能不存在。</p>
  {:else}
    <DatabaseInstanceForm
      initial={toForm(instance)}
      initialServerName={instance.server_name}
      initialBackupPlans={backupPlans.map(toBackupDraft)}
      initialMonitorTargets={monitorTargets.map(toMonitorDraft)}
      entityId={instance.id}
      submitLabel="保存"
      onSubmit={handleUpdate}
    />
  {/if}
</div>
