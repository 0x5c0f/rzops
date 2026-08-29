<script lang="ts">
  import type { CreateDatabaseInstanceRequest } from '$lib/types/database_instance';
import AttachmentFormSection from '$lib/components/shared/AttachmentFormSection.svelte';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { databaseStatusOptions, databaseTypeOptions, importanceOptions, monitorTypeOptions, commonStatusOptions } from '$lib/utils/enum-options';
  import { getServerOptions } from '$lib/utils/entity-options';
  import { backupPlansApi } from '$lib/api/backup-plans';
  import { monitorTargetsApi } from '$lib/api/monitor-targets';
  import { onMount } from 'svelte';

  interface BackupDraft {
    id?: string;
    name: string;
    schedule: string;
    retention_days: string;
    status: string;
  }
  interface MonitorDraft {
    id?: string;
    name: string;
    monitor_type: string;
    endpoint: string;
    interval_seconds: string;
    status: string;
  }

  let {
    initial = {} as CreateDatabaseInstanceRequest,
    entityId = '',
    initialBackupPlans = [] as BackupDraft[],
    initialMonitorTargets = [] as MonitorDraft[],
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateDatabaseInstanceRequest;
    entityId?: string;
    initialBackupPlans?: BackupDraft[];
    initialMonitorTargets?: MonitorDraft[];
    submitLabel?: string;
    onSubmit: (data: CreateDatabaseInstanceRequest) => Promise<string | void>;
  } = $props();

  let saving = $state(false);
  let attachmentRef = $state<{ uploadAll: (id: string) => Promise<void> } | null>(null);
  let serverOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateDatabaseInstanceRequest>(createInitial(initial));
  let enableBackup = $state(initialBackupPlans.length > 0);
  let enableMonitor = $state(initialMonitorTargets.length > 0);
  let backupPlans = $state<BackupDraft[]>(structuredClone(initialBackupPlans));
  let monitorTargets = $state<MonitorDraft[]>(structuredClone(initialMonitorTargets));

  function createInitial(initial?: CreateDatabaseInstanceRequest): CreateDatabaseInstanceRequest {
    return {
      name: '',
      db_type: '',
      server_id: '',
      is_self_installed: false,
      is_ops_managed: false,
      status: 'active',
      ...structuredClone(initial ?? {}),
    };
  }

  onMount(async () => {
    serverOptions = await getServerOptions();
  });

  function emptyBackup(): BackupDraft {
    return { name: '', schedule: '', retention_days: '', status: 'active' };
  }
  function emptyMonitor(): MonitorDraft {
    return { name: '', monitor_type: '', endpoint: '', interval_seconds: '', status: 'active' };
  }
  function addBackupRow() {
    backupPlans = [...backupPlans, emptyBackup()];
  }
  function removeBackupRow(index: number) {
    backupPlans = backupPlans.filter((_, i) => i !== index);
  }
  function addMonitorRow() {
    monitorTargets = [...monitorTargets, emptyMonitor()];
  }
  function removeMonitorRow(index: number) {
    monitorTargets = monitorTargets.filter((_, i) => i !== index);
  }

  // 备份计划增量同步（target_type 固定为 database）
  async function syncBackupPlans(dbId: string) {
    for (const bp of initialBackupPlans) {
      if (bp.id && !backupPlans.some(r => r.id === bp.id)) {
        await backupPlansApi.delete(bp.id);
      }
    }
    for (const row of backupPlans) {
      const payload = {
        name: row.name.trim(),
        target_type: 'database',
        target_id: dbId,
        schedule: row.schedule || undefined,
        retention_days: row.retention_days ? Number(row.retention_days) : undefined,
        status: row.status,
      };
      if (row.id) {
        await backupPlansApi.update(row.id, payload);
      } else {
        await backupPlansApi.create(payload);
      }
    }
  }

  // 监控目标增量同步（target_type 固定为 database）
  async function syncMonitorTargets(dbId: string) {
    for (const mt of initialMonitorTargets) {
      if (mt.id && !monitorTargets.some(r => r.id === mt.id)) {
        await monitorTargetsApi.delete(mt.id);
      }
    }
    for (const row of monitorTargets) {
      const payload = {
        name: row.name.trim(),
        target_type: 'database',
        target_id: dbId,
        monitor_type: row.monitor_type || undefined,
        endpoint: row.endpoint || undefined,
        interval_seconds: row.interval_seconds ? Number(row.interval_seconds) : undefined,
        status: row.status,
      };
      if (row.id) {
        await monitorTargetsApi.update(row.id, payload);
      } else {
        await monitorTargetsApi.create(payload);
      }
    }
  }

  async function handleSave() {
    if (enableBackup && backupPlans.some(r => !r.name.trim())) {
      alert('备份计划的名称必填，请填写完整或删除空行');
      return;
    }
    if (enableMonitor && monitorTargets.some(r => !r.name.trim())) {
      alert('监控目标的名称必填，请填写完整或删除空行');
      return;
    }
    saving = true;
    try {
      const id = await onSubmit(form);
      if (id) {
        if (enableBackup) await syncBackupPlans(id);
        if (enableMonitor) await syncMonitorTargets(id);
        await attachmentRef?.uploadAll(id);
      }
    } catch (err) {
      console.error('Failed to save database instance:', err);
      alert('保存失败，请重试');
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Card.Root>
    <Card.Header>
      <Card.Title>基本信息</Card.Title>
    </Card.Header>
    <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <div class="space-y-2">
        <Label for="name">名称 <span class="text-destructive">*</span></Label>
        <Input id="name" bind:value={form.name} required />
      </div>

      <FormSelect
        label="数据库类型 *"
        bind:value={form.db_type}
        options={$databaseTypeOptions}
        placeholder="选择数据库类型"
        required
      />

      <FormSelect
        label="服务器"
        bind:value={form.server_id}
        options={serverOptions}
        placeholder="选择服务器"
      />

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={$databaseStatusOptions}
      />

      <FormSelect
        label="重要性"
        bind:value={form.importance}
        options={$importanceOptions}
        placeholder="选择重要性"
      />

      <div class="space-y-2">
        <Label for="instance_name">实例名称</Label>
        <Input id="instance_name" bind:value={form.instance_name} />
      </div>

      <div class="space-y-2">
        <Label for="port">端口</Label>
        <Input id="port" type="number" bind:value={form.port} />
      </div>

      <div class="flex items-center gap-4 pt-6">
        <div class="flex items-center gap-2">
          <input id="is_self_installed" type="checkbox" bind:checked={form.is_self_installed} class="h-4 w-4 rounded border-gray-300" />
          <Label for="is_self_installed">自建</Label>
        </div>
        <div class="flex items-center gap-2">
          <input id="is_ops_managed" type="checkbox" bind:checked={form.is_ops_managed} class="h-4 w-4 rounded border-gray-300" />
          <Label for="is_ops_managed">运维托管</Label>
        </div>
      </div>

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="description">描述</Label>
        <TextArea id="description" bind:value={form.description} rows={3} />
      </div>
    </Card.Content>
  </Card.Root>

  <!-- 备份计划：勾选后在此内联新建 -->
  <Card.Root>
    <Card.Header>
      <Card.Title>备份计划</Card.Title>
      <Card.Description>数据库实例通常需要定期备份，勾选后直接在此维护备份计划（也可在"备份计划"菜单维护）</Card.Description>
    </Card.Header>
    <Card.Content class="space-y-3">
      <div class="flex items-center gap-2">
        <input id="enable_backup" type="checkbox" bind:checked={enableBackup} class="h-4 w-4 rounded border-gray-300" />
        <Label for="enable_backup">配置备份计划</Label>
      </div>
      {#if enableBackup}
        {#each backupPlans as bp, i (i)}
          <div class="grid gap-3 rounded-lg border p-3 md:grid-cols-2 lg:grid-cols-5">
            <div class="space-y-1">
              <Label>名称 <span class="text-destructive">*</span></Label>
              <Input bind:value={bp.name} placeholder="如：主库每日备份" />
            </div>
            <div class="space-y-1">
              <Label>调度计划</Label>
              <Input bind:value={bp.schedule} placeholder="cron 表达式" />
            </div>
            <div class="space-y-1">
              <Label>保留天数</Label>
              <Input type="number" bind:value={bp.retention_days} />
            </div>
            <FormSelect label="状态" bind:value={bp.status} options={$commonStatusOptions} />
            <div class="flex items-end">
              <Button variant="outline" size="sm" onclick={() => removeBackupRow(i)}>移除</Button>
            </div>
          </div>
        {/each}
        <Button variant="outline" size="sm" onclick={addBackupRow}>+ 添加备份计划</Button>
      {/if}
    </Card.Content>
  </Card.Root>

  <!-- 监控目标：勾选后在此内联新建 -->
  <Card.Root>
    <Card.Header>
      <Card.Title>监控目标</Card.Title>
      <Card.Description>为数据库实例配置监控（TCP 端口探测等），勾选后直接在此维护（也可在"监控目标"菜单维护）</Card.Description>
    </Card.Header>
    <Card.Content class="space-y-3">
      <div class="flex items-center gap-2">
        <input id="enable_monitor" type="checkbox" bind:checked={enableMonitor} class="h-4 w-4 rounded border-gray-300" />
        <Label for="enable_monitor">配置监控目标</Label>
      </div>
      {#if enableMonitor}
        {#each monitorTargets as mt, i (i)}
          <div class="grid gap-3 rounded-lg border p-3 md:grid-cols-2 lg:grid-cols-5">
            <div class="space-y-1">
              <Label>名称 <span class="text-destructive">*</span></Label>
              <Input bind:value={mt.name} placeholder="如：主库 TCP 监控" />
            </div>
            <FormSelect label="监控类型" bind:value={mt.monitor_type} options={$monitorTypeOptions} placeholder="选择类型" />
            <div class="space-y-1">
              <Label>端点</Label>
              <Input bind:value={mt.endpoint} placeholder="URL / IP:Port" />
            </div>
            <div class="space-y-1">
              <Label>间隔(秒)</Label>
              <Input type="number" bind:value={mt.interval_seconds} />
            </div>
            <div class="flex items-end">
              <Button variant="outline" size="sm" onclick={() => removeMonitorRow(i)}>移除</Button>
            </div>
          </div>
        {/each}
        <Button variant="outline" size="sm" onclick={addMonitorRow}>+ 添加监控目标</Button>
      {/if}
    </Card.Content>
  </Card.Root>

    <AttachmentFormSection bind:this={attachmentRef} targetType="database" targetId={entityId} />

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
