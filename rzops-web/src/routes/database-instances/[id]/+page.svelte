<script lang="ts">
  import AttachmentSection from '$lib/components/shared/AttachmentSection.svelte';
  import { siteRelationsApi, type SiteRefByDatabase } from '$lib/api/site-relations';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import { backupPlansApi } from '$lib/api/backup-plans';
  import { monitorTargetsApi } from '$lib/api/monitor-targets';
  import type { DatabaseInstanceResponse } from '$lib/types/database_instance';
  import type { BackupPlanResponse } from '$lib/types/backup_plan';
  import type { MonitorTargetResponse } from '$lib/types/monitor_target';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import * as Table from '$lib/ui/table';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { databaseStatusOptions, databaseTypeOptions, importanceOptions, siteDatabaseUsageOptions, getOptionLabel } from '$lib/utils/enum-options';
  import { getServerOptions } from '$lib/utils/entity-options';
  import { formatResourceWithStatus, getResourceStatusClass } from '$lib/utils/resource-status';
  import { serversApi } from '$lib/api/servers';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';
  import ConfirmDialog from '$lib/components/shared/ConfirmDialog.svelte';

  let instance = $state<DatabaseInstanceResponse | null>(null);

  let confirmOpen = $state(false);
  let loading = $state(true);
  let serverMap = $state<Record<string, string>>({});
  let serverStatusMap = $state<Record<string, string>>({});
  let backupPlans = $state<BackupPlanResponse[]>([]);
  let monitorTargets = $state<MonitorTargetResponse[]>([]);
  let sites = $state<SiteRefByDatabase[]>([]);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/database-instances'); return; }
    try {
      const [inst, servers, serverList] = await Promise.all([
        databaseInstancesApi.getById(id),
        getServerOptions(),
        serversApi.list({ per_page: 200 }),
      ]);
      instance = inst;
      serverMap = Object.fromEntries(servers.map(o => [o.value, o.label]));
      serverStatusMap = Object.fromEntries(serverList.data.map((s: {id: string, status: string}) => [s.id, s.status]));
      backupPlansApi.list({ target_type: 'database', target_id: id, per_page: 100 }).then(r => { backupPlans = r.data; }).catch(() => {});
      monitorTargetsApi.list({ target_type: 'database', target_id: id, per_page: 100 }).then(r => { monitorTargets = r.data; }).catch(() => {});
      siteRelationsApi.listSitesByDatabase(id).then(s => { sites = s; }).catch(() => {});
    } catch (err) {
      console.error('Failed to load database instance:', err);
      goto('/database-instances');
    } finally {
      loading = false;
    }
  });

  function handleDelete() {
    if (!instance) return;
    confirmOpen = true;
  }

  async function doDelete() {
    if (!instance) return;
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
                  {#if serverMap[instance.server_id]}
                    <a href="/servers/{instance.server_id}" class="text-primary hover:underline">
                      <span class={getResourceStatusClass(serverStatusMap[instance.server_id], 'server')}>
                        {formatResourceWithStatus(serverMap[instance.server_id], serverStatusMap[instance.server_id], 'server')}
                      </span>
                    </a>
                  {:else}
                    <span class="text-muted-foreground italic">已删除</span>
                  {/if}
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
                {#if backupPlans.length > 0}
                  <div class="flex flex-col items-end gap-1">
                    {#each backupPlans as bp}
                      <a href="/backup-plans/{bp.id}" class="text-primary hover:underline">{bp.name}</a>
                    {/each}
                  </div>
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">监控目标</dt>
              <dd>
                {#if monitorTargets.length > 0}
                  <div class="flex flex-col items-end gap-1">
                    {#each monitorTargets as mt}
                      <a href="/monitor-targets/{mt.id}" class="text-primary hover:underline">{mt.name}</a>
                    {/each}
                  </div>
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
    <Card.Root>
      <Card.Header>
        <Card.Title>所属站点</Card.Title>
        <Card.Description>该数据库实例被哪些站点使用（在站点详情页维护关联）</Card.Description>
      </Card.Header>
      <Card.Content>
        <Table.Root>
          <Table.Header>
            <Table.Row>
              <Table.Head>站点</Table.Head>
              <Table.Head>用途</Table.Head>
            </Table.Row>
          </Table.Header>
          <Table.Body>
            {#each sites as s}
              <Table.Row>
                <Table.Cell>
                  <a href="/ops-sites/{s.site_id}" class="text-primary hover:underline">{s.site_name}</a>
                </Table.Cell>
                <Table.Cell>{getOptionLabel($siteDatabaseUsageOptions, s.usage_type) || '-'}</Table.Cell>
              </Table.Row>
            {:else}
              <Table.Row>
                <Table.Cell colspan={2} class="text-center text-muted-foreground">
                  暂未关联站点
                </Table.Cell>
              </Table.Row>
            {/each}
          </Table.Body>
        </Table.Root>
      </Card.Content>
    </Card.Root>
    <AttachmentSection targetType="database" targetId={instance.id} />
  {/if}

    <ConfirmDialog
      bind:open={confirmOpen}
      title="确认删除"
      description={`确定要删除数据库实例「${instance?.name}」吗？此操作可在回收站恢复。`}
      confirmLabel="删除"
      onConfirm={doDelete}
    />
</div>
