<script lang="ts">
  import { goto } from '$app/navigation';
  import { providersApi } from '$lib/api/providers';
  import type { CreateProviderRequest } from '$lib/types/provider';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import ProviderForm from '$lib/components/forms/ProviderForm.svelte';

  async function handleCreate(data: CreateProviderRequest) {
    const res = await providersApi.create(data);
    goto('/providers');
    return res.id;
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '供应商', href: '/providers' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建供应商</h1>
  </div>

  <ProviderForm submitLabel="创建" onSubmit={handleCreate} />
</div>
