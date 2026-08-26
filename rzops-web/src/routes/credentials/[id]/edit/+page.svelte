<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { credentialsApi } from '$lib/api/credentials';
  import type { CreateCredentialRequest, CredentialResponse } from '$lib/types/credential';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import CredentialForm from '$lib/components/forms/CredentialForm.svelte';
  import { onMount } from 'svelte';

  let credential = $state<CredentialResponse | null>(null);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/credentials'); return; }
    try {
      credential = await credentialsApi.getById(id);
    } catch (err) {
      console.error('Failed to load credential:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(c: CredentialResponse): CreateCredentialRequest {
    return {
      name: c.name,
      credential_type: c.credential_type,
      username: c.username ?? '',
      secret_ref: c.secret_ref ?? '',
      owner_id: c.owner_id ?? '',
      status: c.status ?? 'active',
      remarks: c.remarks ?? '',
    };
  }

  async function handleUpdate(data: CreateCredentialRequest) {
    const id = $page.params.id;
    if (!id) return;
    await credentialsApi.update(id, data);
    goto(`/credentials/${id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '凭据', href: '/credentials' },
    { label: credential?.name || '详情', href: credential ? `/credentials/${credential.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑凭据</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !credential}
    <p class="text-sm text-muted-foreground">加载失败，凭据可能不存在。</p>
  {:else}
    <CredentialForm initial={toForm(credential)} submitLabel="保存" onSubmit={handleUpdate} />
  {/if}
</div>
