<script lang="ts">
  import { goto } from '$app/navigation';
  import { rolesApi } from '$lib/api/roles';
  import type { CreateRoleRequest } from '$lib/types/role';
  import PermissionMatrix from '$lib/components/shared/PermissionMatrix.svelte';
  import { validate } from '$lib/utils/validation';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';

  let code = $state('');
  let name = $state('');
  let description = $state('');
  let permissions = $state<string[]>([]);
  let saving = $state(false);
  let error = $state('');

  async function handleSave() {
    error = validate([
      { value: code, label: '角色编码', required: true, maxLength: 50, pattern: /^[a-zA-Z0-9_-]+$/ },
      { value: name, label: '角色名称', required: true, maxLength: 50 },
      { value: description, label: '描述', maxLength: 200 },
    ]) ?? '';
    if (error) return;
    saving = true;
    error = '';
    try {
      const payload: CreateRoleRequest = {
        code: code.trim(),
        name: name.trim(),
        description: description.trim() || null,
        permissions,
      };
      await rolesApi.create(payload);
      goto('/roles');
    } catch (err) {
      error = err instanceof Error ? err.message : '保存失败';
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '角色管理', href: '/roles' }, { label: '新建' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建角色</h1>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => goto('/roles')}>取消</Button>
      <Button onclick={handleSave} disabled={saving}>{saving ? '保存中...' : '保存'}</Button>
    </div>
  </div>

  <div class="grid grid-cols-1 gap-3 md:grid-cols-3">
    <div class="space-y-2">
      <Label for="rc">角色编码 *</Label>
      <Input id="rc" bind:value={code} placeholder="如 dba_ops" />
    </div>
    <div class="space-y-2">
      <Label for="rn">角色名称 *</Label>
      <Input id="rn" bind:value={name} placeholder="如 DBA 运维" />
    </div>
    <div class="space-y-2">
      <Label for="rd">描述</Label>
      <Input id="rd" bind:value={description} placeholder="可选" />
    </div>
  </div>

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
</div>
