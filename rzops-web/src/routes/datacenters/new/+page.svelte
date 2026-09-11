<script lang="ts">
  import { goto } from '$app/navigation';
  import { datacentersApi } from '$lib/api/datacenters';
  import type { CreateDataCenterRequest } from '$lib/types/datacenter';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import DataCenterForm from '$lib/components/forms/DataCenterForm.svelte';

  async function handleCreate(data: CreateDataCenterRequest) {
    const res = await datacentersApi.create(data);
    goto('/datacenters');
    return res.id;
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '数据中心', href: '/datacenters' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建数据中心</h1>
  </div>

  <DataCenterForm submitLabel="创建" onSubmit={handleCreate} />
</div>
