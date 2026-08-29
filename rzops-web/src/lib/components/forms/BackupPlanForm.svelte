<script lang="ts">
  import type { CreateBackupPlanRequest } from '$lib/types/backup_plan';
import AttachmentFormSection from '$lib/components/shared/AttachmentFormSection.svelte';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { commonStatusOptions, backupTargetTypeOptions } from '$lib/utils/enum-options';
  import {
    getDatabaseInstanceOptions,
    getServerOptions,
    getOpsSiteOptions,
  } from '$lib/utils/entity-options';

  let {
    initial = {} as CreateBackupPlanRequest,
    entityId = '',
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateBackupPlanRequest;
    submitLabel?: string;
    onSubmit: (data: CreateBackupPlanRequest) => Promise<string | void>;
  } = $props();

  let saving = $state(false);
  let attachmentRef = $state<{ uploadAll: (id: string) => Promise<void> } | null>(null);
  let targetOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateBackupPlanRequest>(createInitial(initial));

  function createInitial(initial?: CreateBackupPlanRequest): CreateBackupPlanRequest {
    return {
      name: '',
      target_type: '',
      schedule: '',
      status: 'active',
      ...structuredClone(initial ?? {}),
    };
  }

  // 根据目标类型联动加载可选项
  $effect(() => {
    const t = form.target_type;
    if (!t) { targetOptions = []; return; }
    const loader: Record<string, () => Promise<{ label: string; value: string }[]>> = {
      server: getServerOptions,
      database: getDatabaseInstanceOptions,
      site: getOpsSiteOptions,
    };
    const fn = loader[t];
    if (fn) {
      fn().then((opts) => { targetOptions = opts; });
    } else {
      targetOptions = [];
    }
  });

  // 编辑时确保已选值可见（选项未加载/已被删除时兜底显示当前值）
  let targetIdOptions = $derived(
    form.target_id && !targetOptions.some(o => o.value === form.target_id)
      ? [...targetOptions, { label: form.target_id, value: form.target_id }]
      : targetOptions
  );

  async function handleSave() {
    saving = true;
    try {
      const id = await onSubmit({
        ...form,
        target_id: form.target_id || undefined,
        target_type: form.target_type || undefined,
      });
      if (id) await attachmentRef?.uploadAll(id);
    } catch (err) {
      console.error('Failed to save backup plan:', err);
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

      <div class="space-y-2">
        <Label for="schedule">调度计划</Label>
        <Input id="schedule" bind:value={form.schedule} placeholder="cron 表达式" />
      </div>

      <div class="space-y-2">
        <Label for="retention_days">保留天数</Label>
        <Input id="retention_days" type="number" bind:value={form.retention_days} />
      </div>

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={$commonStatusOptions}
      />

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="remarks">备注</Label>
        <TextArea id="remarks" bind:value={form.remarks} rows={3} />
      </div>
    </Card.Content>
  </Card.Root>

  <Card.Root>
    <Card.Header>
      <Card.Title>关联信息</Card.Title>
      <Card.Description>该备份计划的备份对象（可选）</Card.Description>
    </Card.Header>
    <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <FormSelect
        label="目标类型"
        bind:value={form.target_type}
        options={$backupTargetTypeOptions}
        placeholder="选择目标类型"
      />
      <div class="space-y-2">
        <Label for="target_id">目标对象</Label>
        <FormSelect
          label=""
          bind:value={form.target_id}
          options={targetIdOptions}
          placeholder="选择目标对象"
        />
      </div>
    </Card.Content>
  </Card.Root>

    <AttachmentFormSection bind:this={attachmentRef} targetType="backup_plan" targetId={entityId} />

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
