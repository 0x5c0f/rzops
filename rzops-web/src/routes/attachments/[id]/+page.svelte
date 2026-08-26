<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { attachmentsApi } from '$lib/api/attachments';
  import type { AttachmentResponse } from '$lib/types/attachment';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { formatDate, formatBytes } from '$lib/utils/format';
  import { onMount } from 'svelte';

  let attachment = $state<AttachmentResponse | null>(null);
  let loading = $state(true);
  let error = $state('');

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
    other: '其他',
  };

  onMount(async () => {
    try {
      attachment = await attachmentsApi.getById($page.params.id ?? "");
    } catch (err) {
      console.error('Failed to load attachment:', err);
      goto('/attachments');
    } finally {
      loading = false;
    }
  });

  async function handleDownload() {
    if (!attachment) return;
    error = '';
    try {
      await attachmentsApi.download(attachment.id, attachment.filename);
    } catch (err) {
      error = err instanceof Error ? err.message : '下载失败';
    }
  }

  async function handleDelete() {
    if (!attachment) return;
    if (!confirm(`确定要删除附件 "${attachment.filename}" 吗？`)) return;
    try {
      await attachmentsApi.delete(attachment.id);
      goto('/attachments');
    } catch (err) {
      console.error('Failed to delete attachment:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '附件', href: '/attachments' },
    { label: attachment?.filename || '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if attachment}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{attachment.filename}</h1>
        <StatusBadge status={attachment.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={handleDownload}>下载</Button>
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
      </div>
    </div>

    {#if error}
      <p class="text-sm text-red-600">{error}</p>
    {/if}

    <Card.Root>
      <Card.Header>
        <Card.Title>基本信息</Card.Title>
      </Card.Header>
      <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <div class="space-y-2">
          <Label>文件名</Label>
          <div class="text-sm">{attachment.filename}</div>
        </div>
        <div class="space-y-2">
          <Label>目标类型</Label>
          <div class="text-sm">{targetTypeZh[attachment.target_type ?? ''] || attachment.target_type || '-'}</div>
        </div>
        <div class="space-y-2">
          <Label>目标ID</Label>
          <div class="text-sm">{attachment.target_id ?? '-'}</div>
        </div>
        <div class="space-y-2">
          <Label>内容类型</Label>
          <div class="text-sm">{attachment.content_type ?? '-'}</div>
        </div>
        <div class="space-y-2">
          <Label>文件大小</Label>
          <div class="text-sm">{formatBytes(attachment.size_bytes)}</div>
        </div>
        <div class="space-y-2">
          <Label>存储键</Label>
          <div class="text-sm">{attachment.storage_key ?? '-'}</div>
        </div>
        <div class="space-y-2">
          <Label>上传者ID</Label>
          <div class="text-sm">{attachment.uploaded_by_id ?? '-'}</div>
        </div>
        <div class="space-y-2">
          <Label>状态</Label>
          <div class="text-sm"><StatusBadge status={attachment.status} /></div>
        </div>
        <div class="space-y-2">
          <Label>创建时间</Label>
          <div class="text-sm">{formatDate(attachment.created_at)}</div>
        </div>
        <div class="space-y-2 md:col-span-2 lg:col-span-3">
          <Label>备注</Label>
          <div class="text-sm">{attachment.remarks ?? '-'}</div>
        </div>
      </Card.Content>
    </Card.Root>
  {/if}
</div>
