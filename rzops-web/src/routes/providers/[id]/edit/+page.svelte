<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { providersApi } from '$lib/api/providers';
  import type { CreateProviderRequest, ProviderResponse } from '$lib/types/provider';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import ProviderForm from '$lib/components/forms/ProviderForm.svelte';
  import { onMount } from 'svelte';

  let provider = $state<ProviderResponse | null>(null);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/providers'); return; }
    try {
      provider = await providersApi.getById(id);
    } catch (err) {
      console.error('Failed to load provider:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(p: ProviderResponse): CreateProviderRequest {
    return {
      name: p.name,
      provider_types: [...(p.provider_types ?? [])],
      contact_name: p.contact_name ?? '',
      contact_phone: p.contact_phone ?? '',
      contact_qq: p.contact_qq ?? '',
      fax: p.fax ?? '',
      address: p.address ?? '',
      website: p.website ?? '',
      country: p.country ?? '',
      description: p.description ?? '',
      status: p.status ?? 'active',
    };
  }

  async function handleUpdate(data: CreateProviderRequest) {
    const id = $page.params.id;
    if (!id) return;
    await providersApi.update(id, data);
    goto(`/providers/${id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '供应商', href: '/providers' },
    { label: provider?.name || '详情', href: provider ? `/providers/${provider.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑供应商</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !provider}
    <p class="text-sm text-muted-foreground">加载失败，供应商可能不存在。</p>
  {:else}
    <ProviderForm initial={toForm(provider)} submitLabel="保存" onSubmit={handleUpdate} />
  {/if}
</div>
