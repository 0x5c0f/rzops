<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { rolesApi } from '$lib/api/roles';
  import type { RoleResponse } from '$lib/types/role';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { formatDate } from '$lib/utils/format';
  import { canSystem } from '$lib/utils/permissions';
  import { MATRIX_GROUPS, resourceLevel, LEVEL_ACTIONS } from '$lib/utils/perm-matrix';
  import { onMount } from 'svelte';

  let role = $state<RoleResponse | null>(null);
  let permissions = $state<string[]>([]);
  let loading = $state(true);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) {
      goto('/roles');
      return;
    }
    try {
      const res = await rolesApi.get(id);
      role = res.role;
      permissions = res.permissions ?? [];
    } catch (err) {
      console.error('Failed to load role:', err);
      goto('/roles');
    } finally {
      loading = false;
    }
  });

  const levelLabel = (level: number) => {
    if (level <= 0) return { text: '无', cls: 'text-gray-400' };
    const act = LEVEL_ACTIONS.find((a) => a.level === level);
    return {
      text: act?.label ?? String(level),
      cls: level >= 3 ? 'text-red-600' : level === 2 ? 'text-amber-600' : 'text-green-600',
    };
  };
</script>

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '角色管理', href: '/roles' },
    { label: role?.name || '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if role}
    <!-- 页面标题 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold">{role.name}</h1>
        <StatusBadge status={role.is_active ? 'active' : 'disabled'} label={role.is_active ? '启用' : '停用'} />
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/roles')}>返回列表</Button>
        {#if canSystem('role')}
          <Button onclick={() => goto(`/roles/${role.id}/edit`)}>编辑</Button>
        {/if}
      </div>
    </div>

    <!-- 基本信息 -->
    <Card.Root>
      <Card.Header>
        <Card.Title>基本信息</Card.Title>
      </Card.Header>
      <Card.Content>
        <dl class="grid grid-cols-1 gap-x-8 gap-y-4 sm:grid-cols-2 lg:grid-cols-3">
          <div>
            <dt class="text-sm text-gray-500">角色名称</dt>
            <dd class="mt-1 font-medium">{role.name}</dd>
          </div>
          <div>
            <dt class="text-sm text-gray-500">编码</dt>
            <dd class="mt-1 font-mono text-sm">{role.code}</dd>
          </div>
          <div>
            <dt class="text-sm text-gray-500">类型</dt>
            <dd class="mt-1">{role.is_builtin ? '内置' : '自定义'}</dd>
          </div>
          <div>
            <dt class="text-sm text-gray-500">状态</dt>
            <dd class="mt-1">{role.is_active ? '启用' : '停用'}</dd>
          </div>
          <div>
            <dt class="text-sm text-gray-500">创建时间</dt>
            <dd class="mt-1">{formatDate(role.created_at)}</dd>
          </div>
          <div>
            <dt class="text-sm text-gray-500">更新时间</dt>
            <dd class="mt-1">{formatDate(role.updated_at)}</dd>
          </div>
          <div class="sm:col-span-2 lg:col-span-3">
            <dt class="text-sm text-gray-500">描述</dt>
            <dd class="mt-1">{role.description || '-'}</dd>
          </div>
        </dl>
      </Card.Content>
    </Card.Root>

    <!-- 权限 -->
    <Card.Root>
      <Card.Header>
        <Card.Title>权限配置</Card.Title>
        <Card.Description>共 {permissions.length} 个权限点</Card.Description>
      </Card.Header>
      <Card.Content>
        <div class="space-y-6">
          {#each MATRIX_GROUPS as group}
            <div>
              <h3 class="mb-2 text-sm font-semibold text-gray-700">{group.title}</h3>
              <div class="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
                {#each group.resources as res}
                  <div class="flex items-center justify-between rounded-md border border-gray-100 bg-gray-50 px-3 py-2">
                    <span class="text-sm">{res.label}</span>
                    {#if res.system}
                      {#if permissions.includes(`system:${res.key}`)}
                        <span class="text-xs font-medium text-green-600">已授权</span>
                      {:else}
                        <span class="text-xs text-gray-400">未授权</span>
                      {/if}
                    {:else}
                      {@const lv = resourceLevel(res.key, permissions)}
                      <span class={`text-xs font-medium ${levelLabel(lv).cls}`}>{levelLabel(lv).text}</span>
                    {/if}
                  </div>
                {/each}
              </div>
            </div>
          {/each}
        </div>
      </Card.Content>
    </Card.Root>
  {/if}
</div>
