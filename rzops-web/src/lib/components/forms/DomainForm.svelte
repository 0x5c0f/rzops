<script lang="ts">
  import type { CreateDomainRequest } from '$lib/types/domain';
import AttachmentFormSection from '$lib/components/shared/AttachmentFormSection.svelte';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import DateField from '$lib/components/shared/DateField.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { currencyOptions, domainPrivacyStatusOptions } from '$lib/utils/enum-options';
  import { onMount } from 'svelte';

  let {
    initial = {} as CreateDomainRequest,
    entityId = '',
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateDomainRequest;
    submitLabel?: string;
    onSubmit: (data: CreateDomainRequest) => Promise<string | void>;
  } = $props();

  let saving = $state(false);
  let attachmentRef = $state<{ uploadAll: (id: string) => Promise<void> } | null>(null);
  let providerOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateDomainRequest>(createInitial(initial));

  function createInitial(initial?: CreateDomainRequest): CreateDomainRequest {
    return {
      domain_name: '',
      is_enabled: true,
      renewal_currency: 'CNY',
      registered_date: '',
      ...structuredClone(initial ?? {}),
    };
  }

  onMount(async () => {
    providerOptions = await getProviderOptions();
  });

  async function handleSave() {
    saving = true;
    try {
      const id = await onSubmit(form);
      if (id) await attachmentRef?.uploadAll(id);
    } catch (err) {
      console.error('Failed to save domain:', err);
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
        <Label for="domain_name">域名 <span class="text-destructive">*</span></Label>
        <Input id="domain_name" bind:value={form.domain_name} required placeholder="example.com" />
      </div>

      <FormSelect
        label="注册商"
        bind:value={form.provider_id}
        options={providerOptions}
        placeholder="选择注册商"
      />

      <DateField
        id="registered_date"
        label="注册日期"
        bind:value={form.registered_date}
      />

      <DateField
        id="expiry_date"
        label="到期日期"
        bind:value={form.expiry_date}
        min={new Date().toISOString().slice(0, 10)}
      />

      <div class="space-y-2">
        <Label for="renewal_amount">续费金额</Label>
        <Input id="renewal_amount" bind:value={form.renewal_amount} />
      </div>

      <FormSelect
        label="续费币种"
        bind:value={form.renewal_currency}
        options={$currencyOptions}
        placeholder="选择币种"
      />

      <div class="flex items-center gap-2 pt-6">
        <input type="checkbox" id="is_enabled" bind:checked={form.is_enabled} class="h-4 w-4" />
        <Label for="is_enabled">启用</Label>
      </div>

      <div class="space-y-2">
        <Label for="platform_phone">平台电话</Label>
        <Input id="platform_phone" bind:value={form.platform_phone} />
      </div>

      <div class="space-y-2">
        <Label for="domain_email">域名邮箱</Label>
        <Input id="domain_email" bind:value={form.domain_email} />
      </div>

      <FormSelect
        label="隐私状态"
        bind:value={form.privacy_status}
        options={$domainPrivacyStatusOptions}
        placeholder="选择隐私状态"
      />
    </Card.Content>
  </Card.Root>

  <Card.Root>
    <Card.Header>
      <Card.Title>其他</Card.Title>
    </Card.Header>
    <Card.Content>
      <div class="space-y-2">
        <Label for="remarks">备注</Label>
        <TextArea id="remarks" bind:value={form.remarks} rows={3} />
      </div>
    </Card.Content>
  </Card.Root>

    <AttachmentFormSection bind:this={attachmentRef} targetType="domain" targetId={entityId} />

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
