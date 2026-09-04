<script lang="ts">
  import type { CreateDataCenterRequest } from '$lib/types/datacenter';
import AttachmentFormSection from '$lib/components/shared/AttachmentFormSection.svelte';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import FormMultiSelect from '$lib/components/shared/FormMultiSelect.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { commonStatusOptions, countryOptions, lineTypeOptions } from '$lib/utils/enum-options';
  import { validate } from '$lib/utils/validation';
  import { onMount } from 'svelte';

  let {
    initial = {} as CreateDataCenterRequest,
    entityId = '',
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateDataCenterRequest;
    submitLabel?: string;
    onSubmit: (data: CreateDataCenterRequest) => Promise<string | void>;
  } = $props();

  let saving = $state(false);
  let formError = $state<string | null>(null);
  let attachmentRef = $state<{ uploadAll: (id: string) => Promise<void> } | null>(null);
  let providerOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateDataCenterRequest>(createInitial(initial));

  function createInitial(initial?: CreateDataCenterRequest): CreateDataCenterRequest {
    return {
      name: '',
      status: 'active',
      ...JSON.parse(JSON.stringify(initial ?? {})),
    };
  }

  onMount(async () => {
    providerOptions = await getProviderOptions();
  });

  async function handleSave() {
    formError = validate([
      { value: form.name, label: '数据中心名称', required: true, maxLength: 100 },
      { value: form.country, label: '国家', maxLength: 50 },
      { value: form.address, label: '地址', maxLength: 200 },
    ]);
    if (formError) return;
    saving = true;
    try {
      const id = await onSubmit(form);
      if (id) await attachmentRef?.uploadAll(id);
    } catch (err) {
      console.error('Failed to save datacenter:', err);
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

  <div class="flex items-center justify-between">
    <div></div>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => history.back()}>取消</Button>
      <Button onclick={handleSave} disabled={saving}>
        {saving ? '保存中...' : submitLabel}
      </Button>
    </div>
  </div>

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
        label="供应商"
        bind:value={form.provider_id}
        options={providerOptions}
        placeholder="选择供应商"
      />

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={$commonStatusOptions}
      />

      <div class="space-y-2">
        <Label for="phone">电话</Label>
        <Input id="phone" bind:value={form.phone} />
      </div>

      <FormSelect
        label="国家"
        bind:value={form.country}
        options={$countryOptions}
        placeholder="选择国家"
      />

      <FormMultiSelect
        label="线路类型"
        bind:value={form.line_type}
        options={$lineTypeOptions}
        placeholder="选择线路类型"
      />

      <div class="space-y-2">
        <Label for="address">地址</Label>
        <Input id="address" bind:value={form.address} />
      </div>

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="description">描述</Label>
        <TextArea id="description" bind:value={form.description} rows={3} />
      </div>
    </Card.Content>
  </Card.Root>

    <AttachmentFormSection bind:this={attachmentRef} targetType="data_center" targetId={entityId} />

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
