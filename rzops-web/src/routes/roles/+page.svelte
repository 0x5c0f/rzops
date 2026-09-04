<script lang="ts">
  import { rolesApi } from '$lib/api/roles';
  import type { RoleResponse, RoleDetailResponse, CreateRoleRequest, UpdateRoleRequest } from '$lib/types/role';
  import { canCreate, canUpdate, canDelete } from '$lib/utils/permissions';
  import { validate } from '$lib/utils/validation';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Dialog from '$lib/ui/dialog';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';

  let data = $state<RoleResponse[]>([]);
  let loading = $state(true);

  // 新建/编辑对话框
  let dialogOpen = $state(false);
  let editing = $state<RoleResponse | null>(null);
  let form = $state<{
    code: string;
    name: string;
    description: string;
    is_active: boolean;
    permissions: string[];
  }>({ code: '', name: '', description: '', is_active: true, permissions: [] });
  let saving = $state(false);
  let error = $state('');

  // ── 权限矩阵定义 ──
  const businessResources: { key: string; label: string }[] = [
    { key: 'server', label: '服务器' },
    { key: 'datacenter', label: '数据中心' },
    { key: 'provider', label: '供应商' },
    { key: 'domain', label: '域名' },
    { key: 'certificate', label: '证书' },
    { key: 'server_ip', label: '服务器IP' },
    { key: 'server_port', label: '服务器端口' },
    { key: 'server_port_template', label: '端口模板' },
    { key: 'ops_site', label: '站点' },
    { key: 'database_instance', label: '数据库实例' },
    { key: 'backup_plan', label: '备份计划' },
    { key: 'monitor_target', label: '监控目标' },
    { key: 'attachment', label: '附件' },
    { key: 'dict', label: '字典' },
  ];
  const actions = [
    { key: 'read', label: '查看' },
    { key: 'create', label: '新增' },
    { key: 'update', label: '编辑' },
    { key: 'delete', label: '删除' },
  ];
  const systemResources = [
    { key: 'user', label: '用户管理' },
    { key: 'role', label: '角色管理' },
    { key: 'recycle', label: '回收站' },
    { key: 'audit', label: '审计日志' },
    { key: 'change', label: '变更记录' },
  ];

  function hasPerm(perm: string): boolean {
    return form.permissions.includes(perm);
  }

  function togglePerm(perm: string) {
    form.permissions = hasPerm(perm)
      ? form.permissions.filter((p) => p !== perm)
      : [...form.permissions, perm];
  }

  function toggleRow(key: string, checked: boolean) {
    const perms = businessResources
      .filter((r) => r.key === key)
      .flatMap((r) => actions.map((a) => `${r.key}:${a.key}`));
    if (checked) {
      const set = new Set(form.permissions);
      perms.forEach((p) => set.add(p));
      form.permissions = [...set];
    } else {
      form.permissions = form.permissions.filter((p) => !perms.includes(p));
    }
  }

  function rowAllChecked(key: string): boolean {
    return actions.every((a) => hasPerm(`${key}:${a.key}`));
  }

  function rowSomeChecked(key: string): boolean {
    return actions.some((a) => hasPerm(`${key}:${a.key}`));
  }

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
    editing = null;
    form = { code: '', name: '', description: '', is_active: true, permissions: [] };
    error = '';
    dialogOpen = true;
  }

  async function openEdit(item: RoleResponse) {
    editing = item;
    try {
      const detail = await rolesApi.get(item.id);
      form = {
        code: detail.role.code,
        name: detail.role.name,
        description: detail.role.description ?? '',
        is_active: detail.role.is_active,
        permissions: detail.permissions,
      };
    } catch {
      form = { code: item.code, name: item.name, description: '', is_active: item.is_active, permissions: [] };
    }
    error = '';
    dialogOpen = true;
  }

  async function handleSave() {
    error = validate([
      { value: form.code, label: '角色编码', required: true, maxLength: 50, pattern: /^[a-zA-Z0-9_-]+$/ },
      { value: form.name, label: '角色名称', required: true, maxLength: 50 },
      { value: form.description, label: '描述', maxLength: 200 },
    ]) ?? '';
    if (error) return;
    saving = true;
    error = '';
    try {
      if (editing) {
        const payload: UpdateRoleRequest = {
          name: form.name.trim(),
          description: form.description.trim() || null,
          is_active: form.is_active,
          permissions: form.permissions,
        };
        await rolesApi.update(editing.id, payload);
      } else {
        const payload: CreateRoleRequest = {
          code: form.code.trim(),
          name: form.name.trim(),
          description: form.description.trim() || null,
          permissions: form.permissions,
        };
        await rolesApi.create(payload);
      }
      dialogOpen = false;
      await loadData();
    } catch (err) {
      error = err instanceof Error ? err.message : '保存失败';
    } finally {
      saving = false;
    }
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
    { key: 'name', label: '角色名称' },
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

  <!-- 新建 / 编辑对话框 -->
  <Dialog.Root bind:open={dialogOpen}>
    <Dialog.Content class="max-w-3xl">
      <Dialog.Header>
        <Dialog.Title>{editing ? '编辑角色' : '新建角色'}</Dialog.Title>
        <Dialog.Description>
          {editing ? `为角色 ${editing.name} 配置权限点` : '创建自定义角色并勾选权限点'}
        </Dialog.Description>
      </Dialog.Header>
      <div class="max-h-[60vh] space-y-4 overflow-y-auto py-2 pr-1">
        <div class="grid grid-cols-3 gap-3">
          <div class="space-y-2">
            <Label for="rc">角色编码 *</Label>
            <Input id="rc" bind:value={form.code} placeholder="如 dba_ops" disabled={!!editing} />
          </div>
          <div class="space-y-2">
            <Label for="rn">角色名称 *</Label>
            <Input id="rn" bind:value={form.name} placeholder="如 DBA 运维" />
          </div>
          <div class="space-y-2">
            <Label for="rd">描述</Label>
            <Input id="rd" bind:value={form.description} placeholder="可选" />
          </div>
        </div>

        {#if editing}
          <label class="flex items-center gap-2">
            <input type="checkbox" class="h-4 w-4" bind:checked={form.is_active} />
            <span class="text-sm">启用</span>
          </label>
        {/if}

        <!-- 业务资源权限矩阵 -->
        <div class="rounded-md border">
          <div class="border-b bg-muted/40 px-3 py-2 text-sm font-medium">业务资源权限</div>
          <div class="overflow-x-auto">
            <table class="w-full text-sm">
              <thead>
                <tr class="border-b text-left text-muted-foreground">
                  <th class="px-3 py-2 font-medium">资源</th>
                  <th class="px-3 py-2 text-center font-medium">全选</th>
                  {#each actions as a}
                    <th class="px-3 py-2 text-center font-medium">{a.label}</th>
                  {/each}
                </tr>
              </thead>
              <tbody>
                {#each businessResources as r}
                  <tr class="border-b last:border-0 hover:bg-muted/20">
                    <td class="px-3 py-1.5 font-medium">{r.label}</td>
                    <td class="px-3 py-1.5 text-center">
                      <input
                        type="checkbox"
                        class="h-4 w-4"
                        checked={rowAllChecked(r.key)}
                        indeterminate={rowSomeChecked(r.key) && !rowAllChecked(r.key)}
                        onchange={(e) => toggleRow(r.key, (e.target as HTMLInputElement).checked)}
                      />
                    </td>
                    {#each actions as a}
                      <td class="px-3 py-1.5 text-center">
                        <input
                          type="checkbox"
                          class="h-4 w-4"
                          checked={hasPerm(`${r.key}:${a.key}`)}
                          onchange={() => togglePerm(`${r.key}:${a.key}`)}
                        />
                      </td>
                    {/each}
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>

        <!-- 系统权限 -->
        <div class="rounded-md border">
          <div class="border-b bg-muted/40 px-3 py-2 text-sm font-medium">系统管理权限</div>
          <div class="flex flex-wrap gap-3 p-3">
            {#each systemResources as s}
              <label class="flex cursor-pointer items-center gap-1.5 rounded-full border px-3 py-1 text-sm transition-colors"
                class:border-primary={hasPerm(`system:${s.key}`)}
                style={hasPerm(`system:${s.key}`) ? 'background-color: hsl(var(--primary) / 0.1)' : ''}>
                <input
                  type="checkbox"
                  class="h-3.5 w-3.5"
                  checked={hasPerm(`system:${s.key}`)}
                  onchange={() => togglePerm(`system:${s.key}`)}
                />
                {s.label}
              </label>
            {/each}
          </div>
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
</div>
