<script lang="ts">
  import { goto } from '$app/navigation';
  import { certificatesApi } from '$lib/api/certificates';
  import type { CreateCertificateRequest } from '$lib/types/certificate';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { certificateStatusOptions, certificateTypeOptions } from '$lib/utils/enum-options';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { onMount } from 'svelte';

  let providerOptions = $state<{ label: string; value: string }[]>([]);

  onMount(async () => {
    providerOptions = await getProviderOptions();
  });

  let saving = $state(false);
  let form = $state<CreateCertificateRequest>({
    name: '',
    certificate_type: '',
    status: 'active',
    provider_id: '',
    lease_start_date: '',
    lease_end_date: '',
    private_key_credential_id: '',
    domain_id: '',
    domain_pattern: '',
    certificate_id: '',
    is_primary: false,
    remarks: '',
  });

  async function handleSave() {
    saving = true;
    try {
      await certificatesApi.create(form);
      goto('/certificates');
    } catch (err) {
      console.error('Failed to create certificate:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '证书', href: '/certificates' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建证书</h1>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => goto('/certificates')}>取消</Button>
      <Button onclick={handleSave} disabled={saving}>
        {saving ? '创建中...' : '创建'}
      </Button>
    </div>
  </div>

  <Card.Root>
    <Card.Header>
      <Card.Title>基本信息</Card.Title>
    </Card.Header>
    <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <div class="space-y-2">
        <Label for="name">名称 *</Label>
        <Input id="name" bind:value={form.name} required />
      </div>
      <FormSelect label="证书类型" bind:value={form.certificate_type} options={certificateTypeOptions} />
      <FormSelect label="供应商" bind:value={form.provider_id} options={providerOptions} />
      <FormSelect label="状态" bind:value={form.status} options={certificateStatusOptions} />
      <div class="space-y-2">
        <Label for="lease_start_date">起始日期</Label>
        <Input id="lease_start_date" type="date" bind:value={form.lease_start_date} />
      </div>
      <div class="space-y-2">
        <Label for="lease_end_date">到期日期</Label>
        <Input id="lease_end_date" type="date" bind:value={form.lease_end_date} />
      </div>
      <div class="space-y-2">
        <Label for="private_key_credential_id">密钥凭证ID</Label>
        <Input id="private_key_credential_id" bind:value={form.private_key_credential_id} />
      </div>
      <div class="space-y-2">
        <Label for="domain_id">域名ID</Label>
        <Input id="domain_id" bind:value={form.domain_id} />
      </div>
      <div class="space-y-2">
        <Label for="domain_pattern">域名模式</Label>
        <Input id="domain_pattern" bind:value={form.domain_pattern} />
      </div>
      <div class="space-y-2">
        <Label for="certificate_id">证书标识</Label>
        <Input id="certificate_id" bind:value={form.certificate_id} />
      </div>
      <div class="flex items-center gap-2 pt-6">
        <input id="is_primary" type="checkbox" bind:checked={form.is_primary} class="h-4 w-4 rounded border-gray-300" />
        <Label for="is_primary">主证书</Label>
      </div>
      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="remarks">备注</Label>
        <Input id="remarks" bind:value={form.remarks} />
      </div>
    </Card.Content>
  </Card.Root>
</div>
