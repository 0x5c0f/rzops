<script lang="ts">
  import AttachmentSection from '$lib/components/shared/AttachmentSection.svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { monitorTargetsApi } from '$lib/api/monitor-targets';
  import type { MonitorTargetResponse } from '$lib/types/monitor_target';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { monitorTypeOptions, getOptionLabel } from '$lib/utils/enum-options';
  import { onMount } from 'svelte';

  let target = $state<MonitorTargetResponse | null>(null);
  let loading = $state(true);

  const targetTypeZh: Record<string, string> = {
    server: '服务器',
    database: '数据库',
    site: '站点',
    domain: '域名',
    certificate: '证书',
    other: '其他',
  };

  // 目标类型 → 详情页路由前缀
  const targetRoute: Record<string, string> = {
    server: '/servers/',
    database: '/database-instances/',
    site: '/ops-sites/',
    domain: '/domains/',
    certificate: '/certificates/',
  };

  function targetHref(t: string | null, id: string | null): string | null {
    if (!t || !id) return null;
    const prefix = targetRoute[t];
    return prefix ? `${prefix}${id}` : null;
  }

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/monitor-targets'); return; }

    try {
      target = await monitorTargetsApi.getById(id);
    } catch (err) {
      console.error('Failed to load monitor-target:', err);
      goto('/monitor-targets');
    } finally {
      loading = false;
    }
  });

  async function handleDelete() {
    if (!target) return;
    if (!confirm(`确定要删除监控目标 "${target.name}" 吗？`)) return;
    try {
      await monitorTargetsApi.delete(target.id);
      goto('/monitor-targets');
    } catch (err) {
      console.error('Failed to delete monitor-target:', err);
    }
  }
</script>

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '监控目标', href: '/monitor-targets' },
    { label: target?.name || '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if target}
    <!-- 页面标题 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold">{target.name}</h1>
        <StatusBadge status={target.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/monitor-targets')}>返回列表</Button>
        <Button onclick={() => goto(`/monitor-targets/${target?.id}/edit`)}>编辑</Button>
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
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
              <dd>{target.name}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">监控类型</dt>
              <dd>{getOptionLabel($monitorTypeOptions, target.monitor_type)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">端点</dt>
              <dd class="font-mono">{target.endpoint || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">间隔(秒)</dt>
              <dd>{target.interval_seconds ?? '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>

      <!-- 关联信息 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>关联信息</Card.Title>
          <Card.Description>监控的具体对象（可在编辑页调整）</Card.Description>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">目标类型</dt>
              <dd>{targetTypeZh[target.target_type ?? ''] || target.target_type || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">目标对象</dt>
              <dd>
                {#if target.target_name && target.target_id}
                  <a href={targetHref(target.target_type, target.target_id)} class="font-medium text-primary hover:underline">
                    {target.target_name}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>
    </div>
    <AttachmentSection targetType="monitor_target" targetId={target.id} />
  {/if}
</div>
