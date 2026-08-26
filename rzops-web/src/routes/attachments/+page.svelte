<script lang="ts">
  import { goto } from '$app/navigation';
  import { attachmentsApi } from '$lib/api/attachments';
  import type { AttachmentResponse, ListAttachmentsQuery } from '$lib/types/attachment';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate, formatBytes } from '$lib/utils/format';
  import { onMount } from 'svelte';
  import { commonStatusOptions } from '$lib/utils/enum-options';

  let data = $state<AttachmentResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListAttachmentsQuery>({ limit: 20, offset: 0 });
  let offset = $derived(query.offset ?? 0);
  let limit = $derived(query.limit ?? 20);

  let statusMap = $derived(Object.fromEntries(commonStatusOptions.map(o => [o.value, o.label])));

  const columns = [
    { key: 'filename', label: '文件名' },
    { key: 'target_type', label: '目标类型' },
    { key: 'content_type', label: '内容类型' },
    { key: 'size_bytes', label: '文件大小', render: (v: unknown) => v ? `${(v as number / 1024).toFixed(1)} KB` : '-' },
    { key: 'status', label: '状态', valueMap: statusMap },
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
    query = { ...query, q: input.value, offset: 0 };
    loadData();
  }

  function handlePageChange(newOffset: number) {
    query = { ...query, offset: newOffset };
    loadData();
  }

  function handleEdit(item: AttachmentResponse) {
    goto(`/attachments/${item.id}`);
  }

  async function handleDelete(item: AttachmentResponse) {
    if (!confirm(`确定要删除附件 "${item.filename}" 吗？`)) return;
    try {
      await attachmentsApi.delete(item.id);
      loadData();
    } catch (err) {
      console.error('Failed to delete attachment:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '附件' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">附件管理</h1>
    <Button onclick={() => goto('/attachments/new')}>新建附件</Button>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索附件..."
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
    <span>显示 {Math.min(offset + 1, total)}-{Math.min(offset + limit, total)} / 共 {total} 条</span>
    <div class="flex gap-2">
      <Button
        variant="outline"
        size="sm"
        disabled={offset === 0}
        onclick={() => handlePageChange(Math.max(0, offset - limit))}
      >
        上一页
      </Button>
      <Button
        variant="outline"
        size="sm"
        disabled={offset + limit >= total}
        onclick={() => handlePageChange(offset + limit)}
      >
        下一页
      </Button>
    </div>
  </div>
</div>
