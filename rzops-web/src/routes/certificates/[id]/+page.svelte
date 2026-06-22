<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { certificatesApi } from '$lib/api/certificates';
  import type { CertificateResponse, UpdateCertificateRequest } from '$lib/types/certificate';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { certificateStatusOptions, certificateTypeOptions } from '$lib/utils/enum-options';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { onMount } from 'svelte';

  let providerOptions = $state<{ label: string; value: string }[]>([]);

  let certificate = $state<CertificateResponse | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let form = $state<UpdateCertificateRequest>({});

  onMount(async () => {
    try {
      const [cert, providers] = await Promise.all([
        certificatesApi.getById($page.params.id ?? ""),
        getProviderOptions(),
      ]);
      certificate = cert;
      providerOptions = providers;
      form = {
        name: certificate.name,
        provider_id: certificate.provider_id ?? undefined,
        lease_start_date: certificate.lease_start_date ?? undefined,
        lease_end_date: certificate.lease_end_date ?? undefined,
        certificate_type: certificate.certificate_type ?? undefined,
        status: certificate.status,
        private_key_credential_id: certificate.private_key_credential_id ?? undefined,
        domain_id: certificate.domain_id ?? undefined,
        domain_pattern: certificate.domain_pattern ?? undefined,
        certificate_id: certificate.certificate_id ?? undefined,
        is_primary: certificate.is_primary ?? undefined,
        remarks: certificate.remarks ?? undefined,
      };
    } catch (err) {
      console.error('Failed to load certificate:', err);
      goto('/certificates');
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    if (!certificate) return;
    saving = true;
    try {
      await certificatesApi.update(certificate.id, form);
      goto('/certificates');
    } catch (err) {
      console.error('Failed to save certificate:', err);
    } finally {
      saving = false;
    }
  }

  async function handleDelete() {
    if (!certificate) return;
    if (!confirm(`确定要删除证书 "${certificate.name}" 吗？`)) return;
    try {
      await certificatesApi.delete(certificate.id);
      goto('/certificates');
    } catch (err) {
      console.error('Failed to delete certificate:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '证书', href: '/certificates' },
    { label: certificate?.name || '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if certificate}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{certificate.name}</h1>
        <StatusBadge status={certificate.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
        <Button onclick={handleSave} disabled={saving}>
          {saving ? '保存中...' : '保存'}
        </Button>
      </div>
    </div>

    <Card.Root>
      <Card.Header>
        <Card.Title>基本信息</Card.Title>
      </Card.Header>
      <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <div class="space-y-2">
          <Label for="name">名称</Label>
          <Input id="name" bind:value={form.name} />
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
  {/if}
</div>
