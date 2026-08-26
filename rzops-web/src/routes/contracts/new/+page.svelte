<script lang="ts">
  import { goto } from '$app/navigation';
  import { contractsApi } from '$lib/api/contracts';
  import type { CreateContractRequest } from '$lib/types/contract';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import ContractForm from '$lib/components/forms/ContractForm.svelte';

  async function handleCreate(data: CreateContractRequest) {
    await contractsApi.create(data);
    goto('/contracts');
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '合同', href: '/contracts' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建合同</h1>
  </div>

  <ContractForm submitLabel="创建" onSubmit={handleCreate} />
</div>
