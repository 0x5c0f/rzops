<script lang="ts">
  import AttachmentSection from '$lib/components/shared/AttachmentSection.svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import type { DatabaseInstanceResponse } from '$lib/types/database_instance';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { databaseStatusOptions, databaseTypeOptions, importanceOptions, getOptionLabel } from '$lib/utils/enum-options';
  import {
    getServerOptions,
    getBackupPlanOptions,
    getMonitorTargetOptions,
  } from '$lib/utils/entity-options';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';

  let instance = $state<DatabaseInstanceResponse | null>(null);
  let loading = $state(true);
  let serverMap = $state<Record<string, string>>({});
  let backupPlanMap = $state<Record<string, string>>({});
  let monitorTargetMap = $state<Record<string, string>>({});

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/database-instances'); return; }
    try {
      const [inst, servers, backups, monitors] = await Promise.all([
        databaseInstancesApi.getById(id),
        getServerOptions(),
        getBackupPlanOptions(),
        getMonitorTargetOptions(),
      ]);
      instance = inst;
      serverMap = Object.fromEntries(servers.map(o => [o.value, o.label]));
      backupPlanMap = Object.fromEntries(backups.map(o => [o.value, o.label]));
      monitorTargetMap = Object.fromEntries(monitors.map(o => [o.value, o.label]));
    } catch (err) {
      console.error('Failed to load database instance:', err);
      goto('/database-instances');
    } finally {
      loading = false;
    }
  });

  async function handleDelete() {
    if (!instance) return;
    if (!confirm(`确定要删除数据库实例 "${instance.name}" 吗？`)) return;
    try {
      await databaseInstancesApi.delete(instance.id);
      goto('/database-instances');
    } catch (err) {
      console.error('Failed to delete database instance:', err);
    }
  }
</script>

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '数据库实例', href: '/database-instances' },
    { label: instance?.name || '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if instance}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold">{instance.name}</h1>
        <StatusBadge status={instance.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/database-instances')}>返回列表</Button>
        <Button onclick={() => goto(`/database-instances/${instance?.id}/edit`)}>编辑</Button>
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
      </div>
    </div>

    <div class="grid gap-6 lg:grid-cols-2">
      <Card.Root>
        <Card.Header>
          <Card.Title>基本信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">数据库类型</dt>
              <dd>{getOptionLabel($databaseTypeOptions, instance.db_type)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">服务器</dt>
              <dd>
                {#if instance.server_id}
                  <a href="/servers/{instance.server_id}" class="text-primary hover:underline">
                    {serverMap[instance.server_id] || instance.server_id}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">实例名称</dt>
              <dd>{instance.instance_name || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">端口</dt>
              <dd class="font-mono">{instance.port ?? '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">重要性</dt>
              <dd>{getOptionLabel($importanceOptions, instance.importance)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">自建</dt>
              <dd>{instance.is_self_installed ? '是' : '否'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">运维托管</dt>
              <dd>{instance.is_ops_managed ? '是' : '否'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>

      <Card.Root>
        <Card.Header>
          <Card.Title>关联信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">备份计划</dt>
              <dd>
                {#if instance.backup_plan_id}
                  <a href="/backup-plans/{instance.backup_plan_id}" class="text-primary hover:underline">
                    {backupPlanMap[instance.backup_plan_id] || instance.backup_plan_id}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">监控目标</dt>
              <dd>
                {#if instance.monitor_target_id}
                  <a href="/monitor-targets/{instance.monitor_target_id}" class="text-primary hover:underline">
                    {monitorTargetMap[instance.monitor_target_id] || instance.monitor_target_id}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">状态</dt>
              <dd>{getOptionLabel($databaseStatusOptions, instance.status)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">下线时间</dt>
              <dd>{formatDate(instance.offline_time)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">描述</dt>
              <dd>{instance.description || '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>
    </div>
    <AttachmentSection targetType="database" targetId={instance.id} />
  {/if}
</div>
