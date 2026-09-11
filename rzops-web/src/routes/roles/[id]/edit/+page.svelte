<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { rolesApi } from '$lib/api/roles';
  import type { RoleResponse } from '$lib/types/role';
  import PermissionMatrix from '$lib/components/shared/PermissionMatrix.svelte';
  import { validate } from '$lib/utils/validation';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';

  let role = $state<RoleResponse | null>(null);
  let loading = $state(true);
  let loadError = $state(false);
  let name = $state('');
  let description = $state('');
  let isActive = $state(true);
  let permissions = $state<string[]>([]);
  let saving = $state(false);
  let error = $state('');

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/roles'); return; }
    try {
      const detail = await rolesApi.get(id);
      role = detail.role;
      name = detail.role.name;
      description = detail.role.description ?? '';
      isActive = detail.role.is_active;
      permissions = detail.permissions;
    } catch (err) {
      console.error('Failed to load role:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    error = validate([
      { value: name, label: '角色名称', required: true, maxLength: 50 },
      { value: description, label: '描述', maxLength: 200 },
    ]) ?? '';
    if (error) return;
    if (!role) return;
    saving = true;
    error = '';
    try {
      await rolesApi.update(role.id, {
        name: name.trim(),
        description: description.trim() || null,
        is_active: isActive,
        permissions,
      });
      goto('/roles');
    } catch (err) {
      error = err instanceof Error ? err.message : '保存失败';
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '角色管理', href: '/roles' },
    { label: role?.name || '角色', href: role ? `/roles/${role.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">{role ? `编辑角色 ${role.name}` : '编辑角色'}</h1>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => goto('/roles')}>取消</Button>
      <Button onclick={handleSave} disabled={saving || loading}>{saving ? '保存中...' : '保存'}</Button>
    </div>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !role}
    <p class="text-sm text-muted-foreground">加载失败，角色可能不存在。</p>
  {:else}
    <div class="grid grid-cols-1 gap-3 md:grid-cols-3">
      <div class="space-y-2">
        <Label for="rc">角色编码</Label>
        <Input id="rc" value={role.code} disabled />
      </div>
      <div class="space-y-2">
        <Label for="rn">角色名称 *</Label>
        <Input id="rn" bind:value={name} />
      </div>
      <div class="space-y-2">
        <Label for="rd">描述</Label>
        <Input id="rd" bind:value={description} placeholder="可选" />
      </div>
    </div>

    <label class="flex items-center gap-2">
      <input type="checkbox" class="h-4 w-4" bind:checked={isActive} />
      <span class="text-sm">启用</span>
    </label>

    <div>
      <h2 class="mb-2 text-sm font-medium text-muted-foreground">权限配置</h2>
      <p class="mb-3 text-xs text-muted-foreground">
        业务资源采用递进权限：勾选「编辑」自动包含「新增 + 查看」，勾选「删除」自动包含全部。
      </p>
      <PermissionMatrix {permissions} onChange={(p) => (permissions = p)} />
    </div>

    {#if error}
      <p class="text-sm text-destructive">{error}</p>
    {/if}
  {/if}
</div>
