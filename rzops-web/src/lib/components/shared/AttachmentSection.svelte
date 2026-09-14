<script lang="ts">
  import { onMount } from 'svelte';
  import { attachmentsApi } from '$lib/api/attachments';
  import type { AttachmentResponse } from '$lib/types/attachment';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import { formatDate, formatBytes } from '$lib/utils/format';
  import ConfirmDialog from '$lib/components/shared/ConfirmDialog.svelte';
  import { canCreate, canDelete } from '$lib/utils/permissions';

  let {
    targetType,
    targetId,
  }: {
    targetType: string;
    targetId: string;
  } = $props();

  let items = $state<AttachmentResponse[]>([]);
  let loading = $state(true);
  let uploading = $state(false);
  let error = $state('');
  let confirmOpen = $state(false);
  let pendingDelete = $state<AttachmentResponse | null>(null);

  async function load() {
    loading = true;
    try {
      const res = await attachmentsApi.list({ target_type: targetType, target_id: targetId, per_page: 100 });
      items = res.data;
    } catch (err) {
      console.error('Failed to load attachments:', err);
    } finally {
      loading = false;
    }
  }

  onMount(load);

  async function handleFile(e: Event) {
    const input = e.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;
    uploading = true;
    error = '';
    try {
      await attachmentsApi.upload(file, { target_type: targetType, target_id: targetId });
      input.value = '';
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : '上传失败';
    } finally {
      uploading = false;
    }
  }

  async function handleDownload(item: AttachmentResponse) {
    try {
      await attachmentsApi.download(item.id, item.filename);
    } catch (err) {
      error = err instanceof Error ? err.message : '下载失败';
    }
  }

  function handleDelete(item: AttachmentResponse) {
    pendingDelete = item;
    confirmOpen = true;
  }

  async function doDelete() {
    if (!pendingDelete) return;
    try {
      await attachmentsApi.delete(pendingDelete.id);
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : '删除失败';
    } finally {
      pendingDelete = null;
    }
  }
</script>

<Card.Root>
  <Card.Header>
    <Card.Title>附件 ({items.length})</Card.Title>
    <Card.Description>上传与本资源相关的文件</Card.Description>
  </Card.Header>
  <Card.Content class="space-y-3">
    <div class="flex items-center gap-2">
      {#if canCreate('attachment')}
      <input
        type="file"
        class="block max-w-sm text-sm file:mr-2 file:rounded-md file:border-0 file:bg-primary file:px-3 file:py-1.5 file:text-sm file:text-primary-foreground hover:file:bg-primary/90"
        onchange={handleFile}
        disabled={uploading}
      />
      {#if uploading}
        <span class="text-sm text-muted-foreground">上传中...</span>
      {/if}
      {/if}
    </div>
    {#if error}
      <p class="text-sm text-red-600">{error}</p>
    {/if}

    {#if loading}
      <div class="text-sm text-muted-foreground">加载中...</div>
    {:else if items.length === 0}
      <div class="text-sm text-muted-foreground">暂无附件</div>
    {:else}
      <div class="divide-y">
        {#each items as item}
          <div class="flex items-center justify-between py-2">
            <div class="min-w-0">
              <div class="truncate text-sm font-medium">{item.filename}</div>
              <div class="text-xs text-muted-foreground">
                {formatBytes(item.size_bytes ?? 0)} · {formatDate(item.created_at)}
              </div>
            </div>
            <div class="flex shrink-0 gap-1">
              <Button variant="ghost" size="sm" onclick={() => handleDownload(item)}>下载</Button>
              {#if canDelete('attachment')}
              <Button variant="ghost" size="sm" onclick={() => handleDelete(item)} class="text-red-600 hover:text-red-700">删除</Button>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </Card.Content>
</Card.Root>

<ConfirmDialog
  bind:open={confirmOpen}
  title="确认删除"
  description={`确定要删除附件「${pendingDelete?.filename}」吗？此操作可在回收站恢复。`}
  confirmLabel="删除"
  onConfirm={doDelete}
/>
