<script lang="ts">
  import { goto } from '$app/navigation';
  import { rolesApi } from '$lib/api/roles';
  import type { RoleResponse } from '$lib/types/role';
  import { canCreate, canUpdate, canDelete } from '$lib/utils/permissions';
  import { Button } from '$lib/ui/button';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';

  let data = $state<RoleResponse[]>([]);
  let loading = $state(true);

  async function loadData() {
    loading = true;
    try {
      const res = await rolesApi.list();
      data = res.data;
    } catch (err) {
      console.error('Failed to load roles:', err);
    } finally {
      loading = false;
    }
  }

  onMount(loadData);

  function openCreate() {
    goto('/roles/new');
  }

  function openEdit(item: RoleResponse) {
    goto(`/roles/${item.id}/edit`);
  }

  async function handleDelete(item: RoleResponse) {
    try {
      await rolesApi.delete(item.id);
      await loadData();
    } catch (err) {
      console.error('Failed to delete role:', err);
    }
  }

  function getDeleteLabel(item: RoleResponse): string {
    return `${item.name} (${item.code})`;
  }

  const columns = [
    { key: 'name', label: '角色名称', link: (item: RoleResponse) => `/roles/${item.id}` },
    { key: 'code', label: '编码' },
    { key: 'description', label: '描述', render: (v: unknown) => (v ? String(v) : '-') },
    { key: 'is_builtin', label: '类型', render: (v: unknown) => (v ? '内置' : '自定义') },
    { key: 'is_active', label: '状态', display: (item: RoleResponse) => (item.is_active ? '启用' : '停用') },
  ];
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '角色管理' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">角色管理</h1>
    {#if canCreate('role')}
      <Button onclick={openCreate}>新建角色</Button>
    {/if}
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={canUpdate('role') ? openEdit : undefined}
    onDelete={canDelete('role') ? handleDelete : undefined}
    getDeleteLabel={getDeleteLabel}
    storageKey="roles"
  />
</div>
