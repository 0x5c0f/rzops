<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { certificatesApi } from '$lib/api/certificates';
  import type { CertificateResponse, CreateCertificateRequest } from '$lib/types/certificate';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import CertificateForm from '$lib/components/forms/CertificateForm.svelte';
  import { onMount } from 'svelte';

  let cert = $state<CertificateResponse | null>(null);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/certificates'); return; }
    try {
      cert = await certificatesApi.getById(id);
    } catch (err) {
      console.error('Failed to load certificate:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(c: CertificateResponse): CreateCertificateRequest {
    return {
      name: c.name,
      provider_id: c.provider_id ?? '',
      lease_start_date: c.lease_start_date ?? '',
      lease_end_date: c.lease_end_date ?? '',
      certificate_type: c.certificate_type ?? '',
      status: c.status ?? 'active',
      private_key_credential_id: c.private_key_credential_id ?? '',
      remarks: c.remarks ?? '',
    };
  }

  async function handleUpdate(data: CreateCertificateRequest) {
    const id = $page.params.id;
    if (!id) return;
    await certificatesApi.update(id, data);
    goto(`/certificates/${id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '证书', href: '/certificates' },
    { label: cert?.name || '详情', href: cert ? `/certificates/${cert.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑证书</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !cert}
    <p class="text-sm text-muted-foreground">加载失败，证书可能不存在。</p>
  {:else}
    <CertificateForm initial={toForm(cert)} submitLabel="保存" onSubmit={handleUpdate} />
  {/if}
</div>
