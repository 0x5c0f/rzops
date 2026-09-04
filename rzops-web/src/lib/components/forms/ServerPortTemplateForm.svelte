<script lang="ts">
  import type { CreateServerPortTemplateRequest } from '$lib/types/server_port_template';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { protocolOptions } from '$lib/utils/enum-options';
  import { validate } from '$lib/utils/validation';

  let {
    initial = {} as CreateServerPortTemplateRequest,
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateServerPortTemplateRequest;
    submitLabel?: string;
    onSubmit: (data: CreateServerPortTemplateRequest) => Promise<void>;
  } = $props();

  let saving = $state(false);
  let formError = $state<string | null>(null);

  let form = $state<CreateServerPortTemplateRequest>(
    JSON.parse(JSON.stringify({
      name: '',
      protocol: '',
      port: 0,
      service_name: '',
      access_scope: '',
      is_enabled: true,
      description: '',
      ...(initial ?? {}),
    }))
  );

  async function handleSave() {
    formError = validate([
      { value: form.name, label: '模板名称', required: true, maxLength: 100 },
      { value: form.protocol, label: '协议', required: true },
      { value: form.port, label: '端口号', required: true, format: 'port' },
      { value: form.service_name, label: '服务名称', required: true, maxLength: 100 },
    ]);
    if (formError) return;
    saving = true;
    try {
      await onSubmit(form);
    } catch (err) {
      console.error('Failed to save port template:', err);
      formError = '保存失败，请重试';
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  {#if formError}
    <div class="rounded-md border border-destructive/50 bg-destructive/10 px-4 py-3 text-sm text-destructive">
      {formError}
    </div>
  {/if}
  <Card.Root>
    <Card.Header>
      <Card.Title>基本信息</Card.Title>
    </Card.Header>
    <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <div class="space-y-2">
        <Label for="name">模板名称 <span class="text-destructive">*</span></Label>
        <Input id="name" bind:value={form.name} placeholder="如：SSH 标准端口" required />
      </div>

      <FormSelect
        label="协议 *"
        bind:value={form.protocol}
        options={$protocolOptions}
        placeholder="选择协议"
        required
      />

      <div class="space-y-2">
        <Label for="port">端口 <span class="text-destructive">*</span></Label>
        <Input id="port" type="number" bind:value={form.port} placeholder="如 22 / 80 / 443" min={1} max={65535} required />
      </div>

      <div class="space-y-2">
        <Label for="service_name">服务名称 <span class="text-destructive">*</span></Label>
        <Input id="service_name" bind:value={form.service_name} placeholder="如 ssh / nginx / mysql" required />
      </div>

      <div class="space-y-2">
        <Label for="access_scope">访问范围</Label>
        <Input id="access_scope" bind:value={form.access_scope} placeholder="公网 / 内网 / ..." />
      </div>

      <div class="flex items-center gap-2 pt-6">
        <input id="is_enabled" type="checkbox" bind:checked={form.is_enabled} class="h-4 w-4 rounded border-gray-300" />
        <Label for="is_enabled">启用</Label>
      </div>

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="description">描述</Label>
        <TextArea id="description" bind:value={form.description} rows={3} />
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
