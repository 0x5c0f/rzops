<script lang="ts">
  import type { CreateCertificateRequest } from '$lib/types/certificate';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import DateField from '$lib/components/shared/DateField.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { certificateStatusOptions, certificateTypeOptions } from '$lib/utils/enum-options';
  import { getProviderOptions, getCredentialOptions } from '$lib/utils/entity-options';
  import { onMount } from 'svelte';

  let {
    initial = {} as CreateCertificateRequest,
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateCertificateRequest;
    submitLabel?: string;
    onSubmit: (data: CreateCertificateRequest) => Promise<void>;
  } = $props();

  let saving = $state(false);
  let providerOptions = $state<{ label: string; value: string }[]>([]);
  let credentialOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateCertificateRequest>(createInitial(initial));

  function createInitial(initial?: CreateCertificateRequest): CreateCertificateRequest {
    return {
      name: '',
      certificate_type: '',
      status: 'active',
      ...structuredClone(initial ?? {}),
    };
  }

  onMount(async () => {
    const [providers, credentials] = await Promise.all([
      getProviderOptions(),
      getCredentialOptions(),
    ]);
    providerOptions = providers;
    credentialOptions = credentials;
  });

  async function handleSave() {
    if (
      form.lease_start_date &&
      form.lease_end_date &&
      form.lease_end_date < form.lease_start_date
    ) {
      alert('到期日期不能早于起始日期');
      return;
    }
    saving = true;
    try {
      await onSubmit(form);
    } catch (err) {
      console.error('Failed to save certificate:', err);
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
        label="证书类型"
        bind:value={form.certificate_type}
        options={certificateTypeOptions}
        placeholder="选择证书类型"
      />

      <FormSelect
        label="供应商"
        bind:value={form.provider_id}
        options={providerOptions}
        placeholder="选择供应商"
      />

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={certificateStatusOptions}
      />

      <DateField
        id="lease_start_date"
        label="起始日期"
        bind:value={form.lease_start_date}
        max={form.lease_end_date || undefined}
      />

      <DateField
        id="lease_end_date"
        label="到期日期"
        bind:value={form.lease_end_date}
        min={form.lease_start_date || undefined}
      />

      <FormSelect
        label="密钥凭证"
        bind:value={form.private_key_credential_id}
        options={credentialOptions}
        placeholder="选择密钥凭证"
      />

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
