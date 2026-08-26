<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { domainsApi } from '$lib/api/domains';
  import type { CreateDomainRequest, DomainResponse } from '$lib/types/domain';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import DomainForm from '$lib/components/forms/DomainForm.svelte';
  import { onMount } from 'svelte';

  let domain = $state<DomainResponse | null>(null);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/domains'); return; }
    try {
      domain = await domainsApi.getById(id);
    } catch (err) {
      console.error('Failed to load domain:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(d: DomainResponse): CreateDomainRequest {
    return {
      domain_name: d.domain_name,
      business_unit_id: d.business_unit_id ?? '',
      company_id: d.company_id ?? '',
      expiry_date: d.expiry_date ?? '',
      renewal_amount: d.renewal_amount ?? '',
      renewal_currency: d.renewal_currency ?? 'CNY',
      provider_id: d.provider_id ?? '',
      account_credential_id: d.account_credential_id ?? '',
      platform_phone: d.platform_phone ?? '',
      domain_email: d.domain_email ?? '',
      privacy_status: d.privacy_status ?? '',
      is_enabled: d.is_enabled ?? true,
      remarks: d.remarks ?? '',
    };
  }

  async function handleUpdate(data: CreateDomainRequest) {
    const id = $page.params.id;
    if (!id) return;
    await domainsApi.update(id, data);
    goto(`/domains/${id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '域名', href: '/domains' },
    { label: domain?.domain_name || '详情', href: domain ? `/domains/${domain.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑域名</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !domain}
    <p class="text-sm text-muted-foreground">加载失败，域名可能不存在。</p>
  {:else}
    <DomainForm initial={toForm(domain)} submitLabel="保存" onSubmit={handleUpdate} />
  {/if}
</div>
