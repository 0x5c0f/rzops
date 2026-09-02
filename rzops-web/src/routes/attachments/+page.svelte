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

  let data = $state<AttachmentResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let error = $state('');
  let query = $state<ListAttachmentsQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);

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
    { key: 'content_type', label: '内容类型' },
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

  <div class="flex gap-2">
    <Input
      placeholder="搜索文件名..."
      class="max-w-sm"
      oninput={handleSearch}
    />
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    editLabel="下载"
    onEdit={handleDownload}
    onDelete={handleDelete}
  />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
