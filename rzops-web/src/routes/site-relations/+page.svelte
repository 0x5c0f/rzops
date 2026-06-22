<script lang="ts">
  import { goto } from '$app/navigation';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import type { OpsSiteResponse } from '$lib/types/ops_site';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import * as Card from '$lib/ui/card';
  import * as Tabs from '$lib/ui/tabs';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';

  let sites = $state<OpsSiteResponse[]>([]);
  let loading = $state(true);
  let search = $state('');

  onMount(async () => {
    try {
      const res = await opsSitesApi.list({ limit: 100 });
      sites = res.data;
    } catch (err) {
      console.error('Failed to load sites:', err);
    } finally {
      loading = false;
    }
  });

  let filteredSites = $derived(
    search
      ? sites.filter(s => s.name.toLowerCase().includes(search.toLowerCase()))
      : sites
  );
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '站点关联' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">站点关联管理</h1>
  </div>

  <Input
    placeholder="搜索站点..."
    class="max-w-sm"
    bind:value={search}
  />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if filteredSites.length === 0}
    <div class="text-muted-foreground">暂无站点数据</div>
  {:else}
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      {#each filteredSites as site}
        <Card.Root>
          <Card.Header>
            <Card.Title class="text-lg">{site.name}</Card.Title>
            <Card.Description>{site.url || '-'}</Card.Description>
          </Card.Header>
          <Card.Content>
            <div class="space-y-2 text-sm">
              <div class="flex justify-between">
                <span class="text-muted-foreground">服务目标</span>
                <span>{site.service_target || '-'}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground">重要性</span>
                <span>{site.importance || '-'}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-muted-foreground">状态</span>
                <span>{site.status}</span>
              </div>
            </div>
          </Card.Content>
          <Card.Footer>
            <Button variant="outline" size="sm" onclick={() => goto(`/site-relations/${site.id}`)}>
              管理关联
            </Button>
          </Card.Footer>
        </Card.Root>
      {/each}
    </div>
  {/if}
</div>
