<script lang="ts">
  import { goto } from '$app/navigation';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import type { OpsSiteResponse } from '$lib/types/ops_site';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import * as Table from '$lib/ui/table';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';
  import { siteStatusOptions, serviceTargetOptions, importanceOptions } from '$lib/utils/enum-options';

  let sites = $state<OpsSiteResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let search = $state('');
  let page = $state(1);
  const perPage = 20;
  let offset = $derived((page - 1) * perPage);
  let siteStatusMap = $derived(Object.fromEntries($siteStatusOptions.map(o => [o.value, o.label])));
  let importanceMap = $derived(Object.fromEntries($importanceOptions.map(o => [o.value, o.label])));
  let serviceTargetMap = $derived(Object.fromEntries($serviceTargetOptions.map(o => [o.value, o.label])));

  async function loadData() {
    loading = true;
    try {
      const res = await opsSitesApi.list({ q: search || undefined, page, per_page: perPage });
      sites = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load sites:', err);
    } finally {
      loading = false;
    }
  }

  function handleSearch(e: Event) {
    search = (e.target as HTMLInputElement).value;
    page = 1;
    loadData();
  }

  onMount(loadData);
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '站点关联' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">站点关联管理</h1>
  </div>

  <Input
    placeholder="搜索站点..."
    class="max-w-sm"
    value={search}
    oninput={handleSearch}
  />

  <div class="rounded-md border">
    <Table.Root>
      <Table.Header>
        <Table.Row>
          <Table.Head>站点名称</Table.Head>
          <Table.Head>URL</Table.Head>
          <Table.Head>服务目标</Table.Head>
          <Table.Head>重要性</Table.Head>
          <Table.Head>状态</Table.Head>
          <Table.Head class="w-[120px]">操作</Table.Head>
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {#if loading}
          <Table.Row>
            <Table.Cell colspan={6} class="h-24 text-center text-muted-foreground">加载中...</Table.Cell>
          </Table.Row>
        {:else if sites.length === 0}
          <Table.Row>
            <Table.Cell colspan={6} class="h-24 text-center text-muted-foreground">暂无站点数据</Table.Cell>
          </Table.Row>
        {:else}
          {#each sites as site}
            <Table.Row>
              <Table.Cell>
                <a href={`/site-relations/${site.id}`} class="text-primary hover:underline">
                  {site.name}
                </a>
              </Table.Cell>
              <Table.Cell>{site.url || '-'}</Table.Cell>
              <Table.Cell>{site.service_target ? serviceTargetMap[site.service_target] || site.service_target : '-'}</Table.Cell>
              <Table.Cell>{site.importance ? importanceMap[site.importance] || site.importance : '-'}</Table.Cell>
              <Table.Cell>{siteStatusMap[site.status] || site.status}</Table.Cell>
              <Table.Cell>
                <Button variant="outline" size="sm" onclick={() => goto(`/site-relations/${site.id}`)}>
                  管理关联
                </Button>
              </Table.Cell>
            </Table.Row>
          {/each}
        {/if}
      </Table.Body>
    </Table.Root>
  </div>

  <div class="flex items-center justify-between text-sm text-muted-foreground">
    <span>显示 {total === 0 ? 0 : offset + 1}-{Math.min(offset + perPage, total)} / 共 {total} 条</span>
    <div class="flex gap-2">
      <Button
        variant="outline"
        size="sm"
        disabled={page <= 1}
        onclick={() => { page -= 1; loadData(); }}
      >
        上一页
      </Button>
      <Button
        variant="outline"
        size="sm"
        disabled={offset + perPage >= total}
        onclick={() => { page += 1; loadData(); }}
      >
        下一页
      </Button>
    </div>
  </div>
</div>
