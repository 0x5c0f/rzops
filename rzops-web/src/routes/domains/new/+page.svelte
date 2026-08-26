<script lang="ts">
  import { goto } from '$app/navigation';
  import { domainsApi } from '$lib/api/domains';
  import type { CreateDomainRequest } from '$lib/types/domain';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import DomainForm from '$lib/components/forms/DomainForm.svelte';

  async function handleCreate(data: CreateDomainRequest) {
    await domainsApi.create(data);
    goto('/domains');
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '域名', href: '/domains' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建域名</h1>
  </div>

  <DomainForm submitLabel="创建" onSubmit={handleCreate} />
</div>
