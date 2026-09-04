<script lang="ts">
  import { usersApi } from '$lib/api/users';
  import { rolesApi } from '$lib/api/roles';
  import type { UserResponse, CreateUserRequest, UpdateUserRequest } from '$lib/types/user';
  import type { RoleResponse } from '$lib/types/role';
  import { canCreate, canUpdate, canDelete } from '$lib/utils/permissions';
  import { formatDate } from '$lib/utils/format';
  import { validate } from '$lib/utils/validation';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Dialog from '$lib/ui/dialog';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';

  let data = $state<UserResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let q = $state('');
  let page = $state(1);
  let perPage = $state(20);
  let roles = $state<RoleResponse[]>([]);

  // 新建/编辑对话框
  let dialogOpen = $state(false);
  let editing = $state<UserResponse | null>(null);
  let form = $state<{
    email: string;
    password: string;
    full_name: string;
    is_active: boolean;
    is_superuser: boolean;
    role_ids: string[];
  }>({ email: '', password: '', full_name: '', is_active: true, is_superuser: false, role_ids: [] });
  let saving = $state(false);
  let error = $state('');

  // 重置密码对话框
  let pwdOpen = $state(false);
  let pwdTarget = $state<UserResponse | null>(null);
  let pwdValue = $state('');
  let pwdSaving = $state(false);
  let pwdError = $state('');

  const columns = [
    { key: 'email', label: '邮箱' },
    { key: 'full_name', label: '姓名', render: (v: unknown) => (v ? String(v) : '-') },
    {
      key: 'roles', label: '角色',
      render: (v: unknown) => {
        const arr = v as { name: string }[];
        return (arr || []).map((r) => r.name).join(', ') || '-';
      }
    },
    {
      key: 'is_superuser', label: '类型',
      render: (v: unknown) => (v ? '超级管理员' : '普通用户'),
    },
    {
      key: 'is_active', label: '状态',
      display: (item: UserResponse) => (item.is_active ? '启用' : '停用'),
    },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string) },
  ];

  async function loadData() {
    loading = true;
    try {
      const res = await usersApi.list({ q: q || undefined, page, per_page: perPage });
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load users:', err);
    } finally {
      loading = false;
    }
  }

  async function loadRoles() {
    try {
      const res = await rolesApi.list();
      roles = res.data;
    } catch (err) {
      console.error('Failed to load roles:', err);
    }
  }

  onMount(() => {
    loadData();
    loadRoles();
  });

  function openCreate() {
    editing = null;
    form = { email: '', password: '', full_name: '', is_active: true, is_superuser: false, role_ids: [] };
    error = '';
    dialogOpen = true;
  }

  function openEdit(item: UserResponse) {
    editing = item;
    form = {
      email: item.email,
      password: '',
      full_name: item.full_name ?? '',
      is_active: item.is_active,
      is_superuser: item.is_superuser,
      role_ids: item.roles.map((r) => r.id),
    };
    error = '';
    dialogOpen = true;
  }

  function toggleRole(id: string) {
    form.role_ids = form.role_ids.includes(id)
      ? form.role_ids.filter((r) => r !== id)
      : [...form.role_ids, id];
  }

  async function handleSave() {
    error = validate([
      { value: form.email, label: '邮箱', required: true, email: true, maxLength: 100 },
      ...(editing ? [] : [{ value: form.password, label: '密码', required: true, minLength: 6, maxLength: 64 }]),
      { value: form.full_name, label: '姓名', maxLength: 50 },
    ]) ?? '';
    if (error) return;
    saving = true;
    error = '';
    try {
      if (editing) {
        const payload: UpdateUserRequest = {
          full_name: form.full_name.trim() || null,
          is_active: form.is_active,
          is_superuser: form.is_superuser,
          role_ids: form.role_ids,
        };
        await usersApi.update(editing.id, payload);
      } else {
        const payload: CreateUserRequest = {
          email: form.email.trim(),
          password: form.password,
          full_name: form.full_name.trim() || null,
          is_active: form.is_active,
          is_superuser: form.is_superuser,
          role_ids: form.role_ids,
        };
        await usersApi.create(payload);
      }
      dialogOpen = false;
      await loadData();
    } catch (err) {
      error = err instanceof Error ? err.message : '保存失败';
    } finally {
      saving = false;
    }
  }

  function openPwd(item: UserResponse) {
    pwdTarget = item;
    pwdValue = '';
    pwdError = '';
    pwdOpen = true;
  }

  async function handleResetPwd() {
    if (!pwdTarget) return;
    pwdError = validate([{ value: pwdValue, label: '新密码', required: true, minLength: 6, maxLength: 64 }]) ?? '';
    if (pwdError) return;
    pwdSaving = true;
    try {
      await usersApi.resetPassword(pwdTarget.id, { new_password: pwdValue });
      pwdOpen = false;
    } catch (err) {
      pwdError = err instanceof Error ? err.message : '重置失败';
    } finally {
      pwdSaving = false;
    }
  }

  async function handleDelete(item: UserResponse) {
    try {
      await usersApi.delete(item.id);
      await loadData();
    } catch (err) {
      console.error('Failed to delete user:', err);
    }
  }

  function getDeleteLabel(item: UserResponse): string {
    return `${item.full_name || item.email} (${item.email})`;
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '用户管理' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">用户管理</h1>
    {#if canCreate('user')}
      <Button onclick={openCreate}>新建用户</Button>
    {/if}
  </div>

  <div class="flex flex-wrap items-center gap-2">
    <Input
      placeholder="搜索邮箱 / 姓名..."
      class="max-w-sm"
      value={q}
      oninput={(e) => { q = (e.target as HTMLInputElement).value; page = 1; loadData(); }}
    />
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={canUpdate('user') ? openEdit : undefined}
    onDelete={canDelete('user') ? handleDelete : undefined}
    getDeleteLabel={getDeleteLabel}
    storageKey="users"
  >
    {#snippet extraActions(item)}
      <Button variant="ghost" size="sm" onclick={() => openPwd(item)}>重置密码</Button>
    {/snippet}
  </DataTable>

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={(p) => { page = p; loadData(); }}
    onPerPageChange={(s) => { perPage = s; page = 1; loadData(); }}
  />

  <!-- 新建 / 编辑对话框 -->
  <Dialog.Root bind:open={dialogOpen}>
    <Dialog.Content class="sm:max-w-lg">
      <Dialog.Header>
        <Dialog.Title>{editing ? '编辑用户' : '新建用户'}</Dialog.Title>
        <Dialog.Description>
          {editing ? `修改用户 ${editing.email} 的信息与角色` : '创建新的平台用户并分配角色'}
        </Dialog.Description>
      </Dialog.Header>
      <div class="space-y-4 py-2">
        <div class="space-y-2">
          <Label for="email">邮箱 *</Label>
          <Input id="email" type="email" bind:value={form.email} placeholder="user@example.com" disabled={!!editing} />
        </div>
        <div class="grid grid-cols-2 gap-3">
          <div class="space-y-2">
            <Label for="full_name">姓名</Label>
            <Input id="full_name" bind:value={form.full_name} placeholder="可选" />
          </div>
          {#if !editing}
            <div class="space-y-2">
              <Label for="password">初始密码 *</Label>
              <Input id="password" type="password" bind:value={form.password} placeholder="至少 6 位" />
            </div>
          {/if}
        </div>

        <div class="space-y-2">
          <Label>角色分配（可多选）</Label>
          <div class="flex flex-wrap gap-2 rounded-md border p-3">
            {#each roles as r}
              <label
                class="flex cursor-pointer items-center gap-1.5 rounded-full border px-3 py-1 text-sm transition-colors"
                class:border-primary={form.role_ids.includes(r.id)}
                style={form.role_ids.includes(r.id) ? 'background-color: hsl(var(--primary) / 0.1)' : ''}
              >
                <input
                  type="checkbox"
                  class="h-3.5 w-3.5"
                  checked={form.role_ids.includes(r.id)}
                  onchange={() => toggleRole(r.id)}
                />
                {r.name}
              </label>
            {/each}
            {#if roles.length === 0}
              <span class="text-sm text-muted-foreground">暂无角色，请先在角色管理中创建</span>
            {/if}
          </div>
        </div>

        <div class="flex items-center gap-4">
          <label class="flex items-center gap-2">
            <input type="checkbox" class="h-4 w-4" bind:checked={form.is_active} />
            <span class="text-sm">启用</span>
          </label>
          <label class="flex items-center gap-2">
            <input type="checkbox" class="h-4 w-4" bind:checked={form.is_superuser} />
            <span class="text-sm">超级管理员</span>
          </label>
        </div>

        {#if error}
          <p class="text-sm text-destructive">{error}</p>
        {/if}
      </div>
      <Dialog.Footer>
        <Dialog.Close asChild>
          <Button variant="outline">取消</Button>
        </Dialog.Close>
        <Button onclick={handleSave} disabled={saving}>
          {saving ? '保存中...' : '保存'}
        </Button>
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>

  <!-- 重置密码对话框 -->
  <Dialog.Root bind:open={pwdOpen}>
    <Dialog.Content class="sm:max-w-md">
      <Dialog.Header>
        <Dialog.Title>重置密码</Dialog.Title>
        <Dialog.Description>
          {pwdTarget ? `为用户 ${pwdTarget.email} 设置新密码` : ''}
        </Dialog.Description>
      </Dialog.Header>
      <div class="space-y-4 py-2">
        <div class="space-y-2">
          <Label for="pwd">新密码 *</Label>
          <Input id="pwd" type="password" bind:value={pwdValue} placeholder="至少 6 位" />
        </div>
        {#if pwdError}
          <p class="text-sm text-destructive">{pwdError}</p>
        {/if}
      </div>
      <Dialog.Footer>
        <Dialog.Close asChild>
          <Button variant="outline">取消</Button>
        </Dialog.Close>
        <Button onclick={handleResetPwd} disabled={pwdSaving}>
          {pwdSaving ? '重置中...' : '确认重置'}
        </Button>
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>
</div>
