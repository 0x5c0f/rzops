<script lang="ts">
  import type { CreateProviderRequest } from '$lib/types/provider';
import AttachmentFormSection from '$lib/components/shared/AttachmentFormSection.svelte';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import FormMultiSelect from '$lib/components/shared/FormMultiSelect.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { providerTypeOptions, commonStatusOptions } from '$lib/utils/enum-options';

  let {
    initial = {} as CreateProviderRequest,
    entityId = '',
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateProviderRequest;
    submitLabel?: string;
    onSubmit: (data: CreateProviderRequest) => Promise<string | void>;
  } = $props();

  let saving = $state(false);
  let attachmentRef = $state<{ uploadAll: (id: string) => Promise<void> } | null>(null);

  let form = $state<CreateProviderRequest>(createInitial(initial));

  function createInitial(initial?: CreateProviderRequest): CreateProviderRequest {
    return {
      name: '',
      provider_types: [],
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
      console.error('Failed to save provider:', err);
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

      <FormMultiSelect
        label="供应商类型"
        bind:value={form.provider_types}
        options={$providerTypeOptions}
        placeholder="选择供应商类型"
      />

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={$commonStatusOptions}
      />

      <div class="space-y-2">
        <Label for="contact_name">联系人</Label>
        <Input id="contact_name" bind:value={form.contact_name} />
      </div>

      <div class="space-y-2">
        <Label for="contact_phone">电话</Label>
        <Input id="contact_phone" bind:value={form.contact_phone} />
      </div>

      <div class="space-y-2">
        <Label for="contact_qq">QQ</Label>
        <Input id="contact_qq" bind:value={form.contact_qq} />
      </div>

      <div class="space-y-2">
        <Label for="fax">传真</Label>
        <Input id="fax" bind:value={form.fax} />
      </div>

      <div class="space-y-2">
        <Label for="website">网站</Label>
        <Input id="website" bind:value={form.website} />
      </div>

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

    <AttachmentFormSection bind:this={attachmentRef} targetType="provider" targetId={entityId} />

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
