<script lang="ts">
  import { goto } from '$app/navigation';
  import { certificatesApi } from '$lib/api/certificates';
  import type { CreateCertificateRequest } from '$lib/types/certificate';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import CertificateForm from '$lib/components/forms/CertificateForm.svelte';

  async function handleCreate(data: CreateCertificateRequest) {
    await certificatesApi.create(data);
    goto('/certificates');
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '证书', href: '/certificates' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建证书</h1>
  </div>

  <CertificateForm submitLabel="创建" onSubmit={handleCreate} />
</div>
