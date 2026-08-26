<script lang="ts">
  import type { CreateMonitorTargetRequest } from '$lib/types/monitor_target';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { monitorTypeOptions, commonStatusOptions } from '$lib/utils/enum-options';

  let {
    initial = {} as CreateMonitorTargetRequest,
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateMonitorTargetRequest;
    submitLabel?: string;
    onSubmit: (data: CreateMonitorTargetRequest) => Promise<void>;
  } = $props();

  let saving = $state(false);

  let form = $state<CreateMonitorTargetRequest>(createInitial(initial));

  function createInitial(initial?: CreateMonitorTargetRequest): CreateMonitorTargetRequest {
    return {
      name: '',
      monitor_type: '',
      endpoint: '',
      status: 'active',
      ...structuredClone(initial ?? {}),
    };
  }

  async function handleSave() {
    saving = true;
    try {
      await onSubmit(form);
    } catch (err) {
      console.error('Failed to save monitor target:', err);
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
        <Label for="name">名称 *</Label>
        <Input id="name" bind:value={form.name} required />
      </div>

      <FormSelect
        label="监控类型"
        bind:value={form.monitor_type}
        options={$monitorTypeOptions}
        placeholder="选择监控类型"
      />

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={$commonStatusOptions}
      />

      <div class="space-y-2">
        <Label for="endpoint">端点</Label>
        <Input id="endpoint" bind:value={form.endpoint} placeholder="URL / IP:Port / ..." />
      </div>

      <div class="space-y-2">
        <Label for="interval_seconds">间隔(秒)</Label>
        <Input id="interval_seconds" type="number" bind:value={form.interval_seconds} />
      </div>

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="remarks">备注</Label>
        <TextArea id="remarks" bind:value={form.remarks} rows={3} />
      </div>
    </Card.Content>
  </Card.Root>

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
