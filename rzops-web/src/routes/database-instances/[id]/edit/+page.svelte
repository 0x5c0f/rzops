<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import type { CreateDatabaseInstanceRequest, DatabaseInstanceResponse } from '$lib/types/database_instance';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import DatabaseInstanceForm from '$lib/components/forms/DatabaseInstanceForm.svelte';
  import { onMount } from 'svelte';

  let instance = $state<DatabaseInstanceResponse | null>(null);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/database-instances'); return; }
    try {
      instance = await databaseInstancesApi.getById(id);
    } catch (err) {
      console.error('Failed to load database instance:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(d: DatabaseInstanceResponse): CreateDatabaseInstanceRequest {
    return {
      name: d.name,
      db_type: d.db_type,
      server_id: d.server_id ?? '',
      description: d.description ?? '',
      status: d.status ?? 'active',
      is_self_installed: d.is_self_installed ?? false,
      importance: d.importance ?? '',
      is_ops_managed: d.is_ops_managed ?? false,
      management_credential_id: d.management_credential_id ?? '',
      backup_plan_id: d.backup_plan_id ?? '',
      monitor_target_id: d.monitor_target_id ?? '',
      port: d.port ?? undefined,
      instance_name: d.instance_name ?? '',
    };
  }

  async function handleUpdate(data: CreateDatabaseInstanceRequest) {
    const id = $page.params.id;
    if (!id) return;
    await databaseInstancesApi.update(id, data);
    goto(`/database-instances/${id}`);
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '数据库实例', href: '/database-instances' },
    { label: instance?.name || '详情', href: instance ? `/database-instances/${instance.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑数据库实例</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !instance}
    <p class="text-sm text-muted-foreground">加载失败，数据库实例可能不存在。</p>
  {:else}
    <DatabaseInstanceForm initial={toForm(instance)} submitLabel="保存" onSubmit={handleUpdate} />
  {/if}
</div>
