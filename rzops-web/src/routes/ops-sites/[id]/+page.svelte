<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import type { OpsSiteResponse } from '$lib/types/ops_site';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { siteStatusOptions, importanceOptions, serviceTargetOptions, getOptionLabel } from '$lib/utils/enum-options';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';

  let site = $state<OpsSiteResponse | null>(null);
  let loading = $state(true);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/ops-sites'); return; }

    try {
      site = await opsSitesApi.getById(id);
    } catch (err) {
      console.error('Failed to load ops-site:', err);
      goto('/ops-sites');
    } finally {
      loading = false;
    }
  });

  async function handleDelete() {
    if (!site) return;
    if (!confirm(`确定要删除站点 "${site.name}" 吗？`)) return;
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
        <Button onclick={() => goto(`/ops-sites/${site?.id}/edit`)}>编辑</Button>
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
              <dt class="text-muted-foreground">内部系统</dt>
              <dd>{site.is_internal_system ? '是' : '否'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">使用CDN</dt>
              <dd>{site.uses_cdn ? '是' : '否'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">测试站点</dt>
              <dd>{site.is_test_site ? '是' : '否'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>
    </div>
  {/if}
</div>
