<script lang="ts">
  import { goto } from '$app/navigation';
  import { credentialsApi } from '$lib/api/credentials';
  import type { CreateCredentialRequest } from '$lib/types/credential';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import CredentialForm from '$lib/components/forms/CredentialForm.svelte';

  async function handleCreate(data: CreateCredentialRequest) {
    await credentialsApi.create(data);
    goto('/credentials');
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '凭据', href: '/credentials' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建凭据</h1>
  </div>

  <CredentialForm submitLabel="创建" onSubmit={handleCreate} />
</div>
