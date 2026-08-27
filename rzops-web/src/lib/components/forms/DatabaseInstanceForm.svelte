<script lang="ts">
  import type { CreateDatabaseInstanceRequest } from '$lib/types/database_instance';
import AttachmentFormSection from '$lib/components/shared/AttachmentFormSection.svelte';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { databaseStatusOptions, databaseTypeOptions, importanceOptions } from '$lib/utils/enum-options';
  import {
    getServerOptions,
    getBackupPlanOptions,
    getMonitorTargetOptions,
  } from '$lib/utils/entity-options';
  import { onMount } from 'svelte';

  let {
    initial = {} as CreateDatabaseInstanceRequest,
    entityId = '',
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateDatabaseInstanceRequest;
    submitLabel?: string;
    onSubmit: (data: CreateDatabaseInstanceRequest) => Promise<string | void>;
  } = $props();

  let saving = $state(false);
  let attachmentRef = $state<{ uploadAll: (id: string) => Promise<void> } | null>(null);
  let serverOptions = $state<{ label: string; value: string }[]>([]);
  let backupPlanOptions = $state<{ label: string; value: string }[]>([]);
  let monitorTargetOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateDatabaseInstanceRequest>(createInitial(initial));

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
    const [servers, backups, monitors] = await Promise.all([
      getServerOptions(),
      getBackupPlanOptions(),
      getMonitorTargetOptions(),
    ]);
    serverOptions = servers;
    backupPlanOptions = backups;
    monitorTargetOptions = monitors;
  });

  async function handleSave() {
    saving = true;
    try {
      const id = await onSubmit(form);
      if (id) await attachmentRef?.uploadAll(id);
    } catch (err) {
      console.error('Failed to save database instance:', err);
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

      <FormSelect
        label="备份计划"
        bind:value={form.backup_plan_id}
        options={backupPlanOptions}
        placeholder="选择备份计划"
      />

      <FormSelect
        label="监控目标"
        bind:value={form.monitor_target_id}
        options={monitorTargetOptions}
        placeholder="选择监控目标"
      />

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

    <AttachmentFormSection bind:this={attachmentRef} targetType="database" targetId={entityId} />

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
