<script lang="ts">
  import { onMount } from 'svelte';
  import { attachmentsApi } from '$lib/api/attachments';
  import type { AttachmentResponse } from '$lib/types/attachment';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import X from '@lucide/svelte/icons/x';
  import { formatDate, formatBytes } from '$lib/utils/format';

  let {
    targetType,
    targetId = '',
    title = '附件',
  }: {
    targetType: string;
    targetId?: string;
    title?: string;
  } = $props();

  // 新建（无 targetId）时暂存待上传文件，保存成功后由外部调用 uploadAll() 统一上传
  let pendingFiles = $state<File[]>([]);
  let items = $state<AttachmentResponse[]>([]);
  let loading = $state(false);
  let uploading = $state(false);
  let error = $state('');

  let isEdit = $derived(!!targetId);

  async function load() {
    if (!targetId) return;
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

  function selectFiles(e: Event) {
    const input = e.target as HTMLInputElement;
    const files = Array.from(input.files ?? []);
    if (!files.length) return;
    input.value = '';
    if (isEdit) {
      uploadNow(files[0]);
    } else {
      pendingFiles = [...pendingFiles, ...files];
    }
  }

  function removePending(index: number) {
    pendingFiles = pendingFiles.filter((_, i) => i !== index);
  }

  async function uploadNow(file: File) {
    uploading = true;
    error = '';
    try {
      await attachmentsApi.upload(file, { target_type: targetType, target_id: targetId });
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : '上传失败';
    } finally {
      uploading = false;
    }
  }

  /** 保存成功后调用：将新建时暂存的文件上传到实际 targetId */
  export async function uploadAll(resolvedTargetId: string): Promise<void> {
    if (!pendingFiles.length) return;
    error = '';
    uploading = true;
    try {
      for (const file of pendingFiles) {
        await attachmentsApi.upload(file, { target_type: targetType, target_id: resolvedTargetId });
      }
      pendingFiles = [];
    } catch (err) {
      error = err instanceof Error ? err.message : '附件上传失败（主数据已保存）';
      throw err;
    } finally {
      uploading = false;
    }
  }

  export function pendingCount(): number {
    return pendingFiles.length;
  }

  async function handleDownload(item: AttachmentResponse) {
    try {
      await attachmentsApi.download(item.id, item.filename);
    } catch (err) {
      error = err instanceof Error ? err.message : '下载失败';
    }
  }

  async function handleDelete(item: AttachmentResponse) {
    if (!confirm(`确定删除附件 "${item.filename}" 吗？`)) return;
    try {
      await attachmentsApi.delete(item.id);
      await load();
    } catch (err) {
      error = err instanceof Error ? err.message : '删除失败';
    }
  }
</script>

<Card.Root>
  <Card.Header>
    <Card.Title>
      {title}
      {#if isEdit}
        ({items.length})
      {:else if pendingFiles.length}
        (待上传 {pendingFiles.length})
      {/if}
    </Card.Title>
    <Card.Description>
      {isEdit ? '上传与本资源相关的文件' : '可在此提前选择文件，保存后自动上传'}
    </Card.Description>
  </Card.Header>
  <Card.Content class="space-y-3">
    <div class="flex items-center gap-2">
      <input
        type="file"
        class="block max-w-sm text-sm file:mr-2 file:rounded-md file:border-0 file:bg-primary file:px-3 file:py-1.5 file:text-sm file:text-primary-foreground hover:file:bg-primary/90"
        onchange={selectFiles}
        disabled={uploading}
      />
      {#if uploading}
        <span class="text-sm text-muted-foreground">上传中...</span>
      {/if}
    </div>
    {#if error}
      <p class="text-sm text-red-600">{error}</p>
    {/if}

    {#if !isEdit && pendingFiles.length > 0}
      <div class="divide-y rounded-lg border">
        {#each pendingFiles as file, i}
          <div class="flex items-center justify-between py-2 pl-3">
            <div class="min-w-0">
              <div class="truncate text-sm font-medium">{file.name}</div>
              <div class="text-xs text-muted-foreground">{formatBytes(file.size)}</div>
            </div>
            <button
              type="button"
              onclick={() => removePending(i)}
              class="m-1 rounded-full p-1 text-muted-foreground hover:bg-muted hover:text-foreground"
              aria-label="移除"
            >
              <X class="h-4 w-4" />
            </button>
          </div>
        {/each}
      </div>
    {/if}

    {#if isEdit}
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
                <Button variant="ghost" size="sm" onclick={() => handleDelete(item)} class="text-red-600 hover:text-red-700">删除</Button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    {/if}
  </Card.Content>
</Card.Root>
