<script lang="ts">
  import type { CreateBackupPlanRequest } from '$lib/types/backup_plan';
import AttachmentFormSection from '$lib/components/shared/AttachmentFormSection.svelte';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { commonStatusOptions } from '$lib/utils/enum-options';

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

  async function handleSave() {
    saving = true;
    try {
      const id = await onSubmit(form);
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

    <AttachmentFormSection bind:this={attachmentRef} targetType="backup_plan" targetId={entityId} />

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
