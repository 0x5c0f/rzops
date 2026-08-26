<script lang="ts">
  import { goto } from '$app/navigation';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import type { OpsSiteResponse, ListOpsSitesQuery } from '$lib/types/ops_site';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';
  import { siteStatusOptions, serviceTargetOptions, importanceOptions } from '$lib/utils/enum-options';

  let data = $state<OpsSiteResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListOpsSitesQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let siteStatusMap = $derived(Object.fromEntries($siteStatusOptions.map(o => [o.value, o.label])));
  let serviceTargetMap = $derived(Object.fromEntries($serviceTargetOptions.map(o => [o.value, o.label])));
  let importanceMap = $derived(Object.fromEntries($importanceOptions.map(o => [o.value, o.label])));

  const columns = $derived([
    { key: 'name', label: '名称' , link: (item: OpsSiteResponse) => `/ops-sites/${item.id}` },
    { key: 'url', label: 'URL' },
    { key: 'service_target', label: '服务目标', valueMap: serviceTargetMap },
    { key: 'importance', label: '重要性', valueMap: importanceMap },
    { key: 'status', label: '状态', valueMap: siteStatusMap },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string) },
  ]);

  async function loadData() {
    loading = true;
    try {
      const res = await opsSitesApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load ops-sites:', err);
    } finally {
      loading = false;
    }
  }

  onMount(loadData);

  function handleSearch(e: Event) {
    const input = e.target as HTMLInputElement;
    query = { ...query, q: input.value, page: 1 };
    loadData();
  }

  function handlePageChange(newPage: number) {
    query = { ...query, page: newPage };
    loadData();
  }

  function handleEdit(item: OpsSiteResponse) {
    goto(`/ops-sites/${item.id}/edit`);
  }

  async function handleDelete(item: OpsSiteResponse) {
    if (!confirm(`确定要删除站点 "${item.name}" 吗？`)) return;
    try {
      await opsSitesApi.delete(item.id);
      loadData();
    } catch (err) {
      console.error('Failed to delete ops-site:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '站点' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">站点管理</h1>
    <Button onclick={() => goto('/ops-sites/new')}>新建站点</Button>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索站点..."
      class="max-w-sm"
      oninput={handleSearch}
    />
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={handleEdit}
    onDelete={handleDelete}
  />

  <div class="flex items-center justify-between text-sm text-muted-foreground">
    <span>显示 {Math.min((page - 1) * perPage + 1, total)}-{Math.min(page * perPage, total)} / 共 {total} 条</span>
    <div class="flex gap-2">
      <Button
        variant="outline"
        size="sm"
        disabled={page <= 1}
        onclick={() => handlePageChange(page - 1)}
      >
        上一页
      </Button>
      <Button
        variant="outline"
        size="sm"
        disabled={page * perPage >= total}
        onclick={() => handlePageChange(page + 1)}
      >
        下一页
      </Button>
    </div>
  </div>
</div>
