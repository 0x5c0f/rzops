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
  import ConfirmDialog from '$lib/components/shared/ConfirmDialog.svelte';

  let attachment = $state<AttachmentResponse | null>(null);

  let confirmOpen = $state(false);
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
    contract: '合同',
    other: '其他',
  };

  // 目标类型 → 详情页路由前缀
  const targetRoute: Record<string, string> = {
    server: '/servers/',
    database: '/database-instances/',
    site: '/ops-sites/',
    domain: '/domains/',
    certificate: '/certificates/',
    provider: '/providers/',
    data_center: '/datacenters/',
    monitor_target: '/monitor-targets/',
    backup_plan: '/backup-plans/',
    contract: '/contracts/',
  };

  function targetHref(targetType: string | null, targetId: string | null): string | null {
    if (!targetType || !targetId) return null;
    const prefix = targetRoute[targetType];
    return prefix ? `${prefix}${targetId}` : null;
  }

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

  function handleDelete() {
    if (!attachment) return;
    confirmOpen = true;
  }

  async function doDelete() {
    if (!attachment) return;
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
          <Label>关联目标</Label>
          {#if attachment.target_id}
            {@const href = targetHref(attachment.target_type, attachment.target_id)}
            {#if href}
              <a href={href} class="text-sm font-medium text-primary hover:underline">
                {attachment.target_name || attachment.target_id}
              </a>
            {:else}
              <div class="text-sm">{attachment.target_name || attachment.target_id}</div>
            {/if}
          {:else}
            <div class="text-sm">-</div>
          {/if}
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
          <Label>上传者</Label>
          <div class="text-sm">{attachment.uploader_name ?? '-'}</div>
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

    <ConfirmDialog
      bind:open={confirmOpen}
      title="确认删除"
      description={`确定要删除附件「${attachment?.filename}」吗？此操作可在回收站恢复。`}
      confirmLabel="删除"
      onConfirm={doDelete}
    />
</div>
