<script lang="ts">
  import { goto } from '$app/navigation';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import type { CreateDatabaseInstanceRequest } from '$lib/types/database_instance';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import DatabaseInstanceForm from '$lib/components/forms/DatabaseInstanceForm.svelte';

  async function handleCreate(data: CreateDatabaseInstanceRequest) {
    const res = await databaseInstancesApi.create(data);
    goto('/database-instances');
    return res.id;
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '数据库实例', href: '/database-instances' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建数据库实例</h1>
  </div>

  <DatabaseInstanceForm submitLabel="创建" onSubmit={handleCreate} />
</div>
