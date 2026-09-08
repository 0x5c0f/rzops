<script lang="ts">
  import { attachmentsApi } from '$lib/api/attachments';
  import type { AttachmentResponse, ListAttachmentsQuery } from '$lib/types/attachment';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate, formatBytes } from '$lib/utils/format';
  import { onMount } from 'svelte';
import { canCreate, canUpdate, canDelete } from '$lib/utils/permissions';

  let data = $state<AttachmentResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let error = $state('');
  let query = $state<ListAttachmentsQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let showAdvancedFilter = $state(false);

  let activeFilterCount = $derived.by(() => {
    let count = 0;
    if (query.target_type) count++;
    return count;
  });

  const targetTypeZh: Record<string, string> = {
    server: '服务器',
    database: '数据库',
    site: '站点',
    domain: '域名',
    certificate: '证书',
    provider: '供应商',
    data_center: '数据中心',
    credential: '凭据',
    backup_plan: '备份计划',
    monitor_target: '监控目标',
    contract: '合同',
    other: '其他',
  };

  const targetRoute: Record<string, string> = {
    server: '/servers/',
    database: '/database-instances/',
    site: '/ops-sites/',
    domain: '/domains/',
    certificate: '/certificates/',
    provider: '/providers/',
    data_center: '/data-centers/',
    monitor_target: '/monitor-targets/',
    backup_plan: '/backup-plans/',
    contract: '/contracts/',
  };

  function targetHref(targetType: string | null, targetId: string | null): string | null {
    if (!targetType || !targetId) return null;
    const prefix = targetRoute[targetType];
    return prefix ? `${prefix}${targetId}` : null;
  }

  const columns = [
    { key: 'filename', label: '文件名' , link: (item: AttachmentResponse) => `/attachments/${item.id}` },
    {
      key: 'target_type',
      label: '关联目标',
      render: (v: unknown, item: AttachmentResponse) => {
        const type = targetTypeZh[v as string] || (v as string) || '';
        const name = item.target_name;
        return name ? `${type} / ${name}` : (type || '-');
      },
      link: (item: AttachmentResponse) => targetHref(item.target_type, item.target_id),
    },
    { key: 'content_type', label: '内容类型', hideBelow: 'md' },
    { key: 'size_bytes', label: '文件大小', render: (v: unknown) => formatBytes(v as number) },
    { key: 'created_at', label: '上传时间', render: (v: unknown) => formatDate(v as string) },
  ];

  async function loadData() {
    loading = true;
    try {
      const res = await attachmentsApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load attachments:', err);
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

  function resetFilters() {
    query = { page: 1, per_page: perPage };
    loadData();
  }

  function clearFilter(key: keyof ListAttachmentsQuery) {
    const newQuery = { ...query };
    delete newQuery[key];
    newQuery.page = 1;
    query = newQuery;
    loadData();
  }

  function handlePageChange(newPage: number) {
    query = { ...query, page: newPage };
    loadData();
  }

  function handlePerPageChange(newPerPage: number) {
    query = { ...query, per_page: newPerPage, page: 1 };
    loadData();
  }

  async function handleDownload(item: AttachmentResponse) {
    error = '';
    try {
      await attachmentsApi.download(item.id, item.filename);
    } catch (err) {
      error = err instanceof Error ? err.message : '下载失败';
    }
  }

  async function handleDelete(item: AttachmentResponse) {
    try {
      await attachmentsApi.delete(item.id);
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
    } catch (err) {
      console.error('Failed to delete attachment:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '附件' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">附件管理</h1>
    <span class="text-sm text-muted-foreground">附件在各资源详情页中上传，此处可全局查看与下载</span>
  </div>

  {#if error}
    <p class="text-sm text-red-600">{error}</p>
  {/if}

  <div class="space-y-3">
    <div class="flex flex-wrap items-center gap-2">
      <Input
        placeholder="搜索文件名..."
        class="max-w-sm"
        value={query.q ?? ''}
        oninput={handleSearch}
      />
      <Button
        variant={showAdvancedFilter ? 'default' : 'outline'}
        size="sm"
        onclick={() => (showAdvancedFilter = !showAdvancedFilter)}
      >
        高级筛选
        {#if activeFilterCount > 0}
          <span class="ml-1 rounded-full bg-primary px-1.5 text-xs text-primary-foreground">{activeFilterCount}</span>
        {/if}
      </Button>
      {#if activeFilterCount > 0}
        <Button variant="ghost" size="sm" onclick={resetFilters}>重置</Button>
      {/if}
    </div>

    {#if activeFilterCount > 0}
      <div class="flex flex-wrap items-center gap-2">
        <span class="text-sm text-muted-foreground">已选条件：</span>
        {#if query.target_type}
          <span class="inline-flex items-center gap-1 rounded-full bg-secondary px-2 py-0.5 text-xs">
            目标类型: {targetTypeZh[query.target_type] ?? query.target_type}
            <button class="ml-1 hover:text-destructive" onclick={() => clearFilter('target_type')}>×</button>
          </span>
        {/if}
      </div>
    {/if}

    {#if showAdvancedFilter}
      <div class="grid gap-3 rounded-lg border p-4 md:grid-cols-2">
        <div class="space-y-1">
          <label class="text-xs font-medium text-muted-foreground">目标类型</label>
          <select
            class="w-full rounded-md border px-3 py-2 text-sm"
            value={query.target_type ?? ''}
            onchange={(e) => {
              const val = (e.target as HTMLSelectElement).value;
              query = { ...query, target_type: val || undefined, page: 1 };
              loadData();
            }}
          >
            <option value="">全部</option>
            {#each Object.entries(targetTypeZh) as [value, label]}
              <option value={value}>{label}</option>
            {/each}
          </select>
        </div>
      </div>
    {/if}
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    editLabel="下载"
    onEdit={handleDownload}
    onDelete={canDelete('attachment') ? handleDelete : undefined} storageKey="attachments" />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
