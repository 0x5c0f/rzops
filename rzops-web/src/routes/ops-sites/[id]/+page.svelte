<script lang="ts">
  import AttachmentSection from '$lib/components/shared/AttachmentSection.svelte';
  import SiteRelationsSection from '$lib/components/shared/SiteRelationsSection.svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import { backupPlansApi } from '$lib/api/backup-plans';
  import { monitorTargetsApi } from '$lib/api/monitor-targets';
  import type { OpsSiteResponse } from '$lib/types/ops_site';
  import type { BackupPlanResponse } from '$lib/types/backup_plan';
  import type { MonitorTargetResponse } from '$lib/types/monitor_target';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { siteStatusOptions, importanceOptions, serviceTargetOptions, environmentOptions, getOptionLabel } from '$lib/utils/enum-options';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';
  import ConfirmDialog from '$lib/components/shared/ConfirmDialog.svelte';
import { canUpdate, canDelete } from '$lib/utils/permissions';

  let site = $state<OpsSiteResponse | null>(null);

  let confirmOpen = $state(false);
  let loading = $state(true);
  let backupPlans = $state<BackupPlanResponse[]>([]);
  let monitorTargets = $state<MonitorTargetResponse[]>([]);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/ops-sites'); return; }

    try {
      site = await opsSitesApi.getById(id);
      backupPlansApi.list({ target_type: 'site', target_id: id, per_page: 100 }).then(r => { backupPlans = r.data; }).catch(() => {});
      monitorTargetsApi.list({ target_type: 'site', target_id: id, per_page: 100 }).then(r => { monitorTargets = r.data; }).catch(() => {});
    } catch (err) {
      console.error('Failed to load ops-site:', err);
      goto('/ops-sites');
    } finally {
      loading = false;
    }
  });

  function handleDelete() {
    if (!site) return;
    confirmOpen = true;
  }

  async function doDelete() {
    if (!site) return;
    try {
      await opsSitesApi.delete(site.id);
      goto('/ops-sites');
      } catch (err) {
      console.error('Failed to delete ops-site:', err);
    }
  }
</script>

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '站点', href: '/ops-sites' },
    { label: site?.name || '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if site}
    <!-- 页面标题 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold">{site.name}</h1>
        <StatusBadge status={site.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/ops-sites')}>返回列表</Button>
        {#if canUpdate('ops_site')}
        <Button onclick={() => goto(`/ops-sites/${site?.id}/edit`)}>编辑</Button>
      {/if}
        {#if canDelete('ops_site')}
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
      {/if}
      </div>
    </div>

    <div class="grid gap-6 lg:grid-cols-2">
      <!-- 基本信息 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>基本信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">名称</dt>
              <dd>{site.name}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">URL</dt>
              <dd class="font-mono">{site.url || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">服务目标</dt>
              <dd>{getOptionLabel($serviceTargetOptions, site.service_target)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">环境</dt>
              <dd>{site.environment ? getOptionLabel($environmentOptions, site.environment) : '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">重要性</dt>
              <dd>{getOptionLabel($importanceOptions, site.importance)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">上线时间</dt>
              <dd>{formatDate(site.online_time)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">用途</dt>
              <dd>{site.purpose || '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>

      <!-- 技术信息 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>技术信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">代码仓库类型</dt>
              <dd>{site.code_repo_type || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">代码仓库地址</dt>
              <dd class="font-mono">{site.code_repo_url || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">语言/运行时</dt>
              <dd>{site.language_runtime || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">Web框架</dt>
              <dd>{site.web_framework || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">测试站点</dt>
              <dd>{site.is_test_site ? '是' : '否'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>
    </div>
    <SiteRelationsSection siteId={site.id} />
    <Card.Root>
      <Card.Header>
        <Card.Title>运维配置</Card.Title>
        <Card.Description>该站点的备份计划与监控目标（在编辑页维护）</Card.Description>
      </Card.Header>
      <Card.Content class="grid gap-6 md:grid-cols-2">
        <div class="space-y-2">
          <h3 class="text-sm font-medium">备份计划</h3>
          {#if backupPlans.length > 0}
            <ul class="space-y-1 text-sm">
              {#each backupPlans as bp}
                <li>
                  <a href="/backup-plans/{bp.id}" class="text-primary hover:underline">{bp.name}</a>
                  <span class="ml-2 text-muted-foreground">{bp.schedule || ''}</span>
                </li>
              {/each}
            </ul>
          {:else}
            <p class="text-sm text-muted-foreground">暂未配置备份计划</p>
          {/if}
        </div>
        <div class="space-y-2">
          <h3 class="text-sm font-medium">监控目标</h3>
          {#if monitorTargets.length > 0}
            <ul class="space-y-1 text-sm">
              {#each monitorTargets as mt}
                <li>
                  <a href="/monitor-targets/{mt.id}" class="text-primary hover:underline">{mt.name}</a>
                  <span class="ml-2 text-muted-foreground">{mt.endpoint || ''}</span>
                </li>
              {/each}
            </ul>
          {:else}
            <p class="text-sm text-muted-foreground">暂未配置监控目标</p>
          {/if}
        </div>
      </Card.Content>
    </Card.Root>
    <AttachmentSection targetType="site" targetId={site.id} />
  {/if}

    <ConfirmDialog
      bind:open={confirmOpen}
      title="确认删除"
      description={`确定要删除站点「${site?.name}」吗？此操作可在回收站恢复。`}
      confirmLabel="删除"
      onConfirm={doDelete}
    />
</div>
