<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { contractsApi } from '$lib/api/contracts';
  import type { ContractResponse, CreateContractRequest } from '$lib/types/contract';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import ContractForm from '$lib/components/forms/ContractForm.svelte';
  import { onMount } from 'svelte';

  let contract = $state<ContractResponse | null>(null);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/contracts'); return; }
    try {
      contract = await contractsApi.getById(id);
    } catch (err) {
      console.error('Failed to load contract:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(c: ContractResponse): CreateContractRequest {
    return {
      name: c.name,
      provider_id: c.provider_id ?? '',
      subject_type: c.subject_type ?? '',
      subject_id: c.subject_id ?? '',
      contract_no: c.contract_no ?? '',
      start_date: c.start_date ?? '',
      end_date: c.end_date ?? '',
      amount: c.amount ?? '',
      currency: c.currency ?? 'CNY',
      status: c.status ?? 'active',
      remarks: c.remarks ?? '',
    };
  }

  async function handleUpdate(data: CreateContractRequest) {
    const id = $page.params.id;
    if (!id) return;
    await contractsApi.update(id, data);
    goto(`/contracts/${id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '合同', href: '/contracts' },
    { label: contract?.name || '详情', href: contract ? `/contracts/${contract.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑合同</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !contract}
    <p class="text-sm text-muted-foreground">加载失败，合同可能不存在。</p>
  {:else}
    <ContractForm initial={toForm(contract)} submitLabel="保存" onSubmit={handleUpdate} />
  {/if}
</div>
