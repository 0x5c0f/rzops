<script lang="ts">
  import { goto } from '$app/navigation';
  import { attachmentsApi } from '$lib/api/attachments';
  import type { CreateAttachmentRequest } from '$lib/types/attachment';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';

  let saving = $state(false);
  let form = $state<CreateAttachmentRequest>({
    filename: '',
    target_type: '',
    target_id: '',
    storage_key: '',
    content_type: '',
    size_bytes: undefined,
    uploaded_by_id: '',
    status: 'active',
    remarks: '',
  });

  async function handleSave() {
    saving = true;
    try {
      await attachmentsApi.create(form);
      goto('/attachments');
    } catch (err) {
      console.error('Failed to create attachment:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '附件', href: '/attachments' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建附件</h1>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => goto('/attachments')}>取消</Button>
      <Button onclick={handleSave} disabled={saving}>
        {saving ? '创建中...' : '创建'}
      </Button>
    </div>
  </div>

  <Card.Root>
    <Card.Header>
      <Card.Title>基本信息</Card.Title>
    </Card.Header>
    <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <div class="space-y-2">
        <Label for="filename">文件名 *</Label>
        <Input id="filename" bind:value={form.filename} required />
      </div>
      <div class="space-y-2">
        <Label for="target_type">目标类型</Label>
        <Input id="target_type" bind:value={form.target_type} />
      </div>
      <div class="space-y-2">
        <Label for="target_id">目标ID</Label>
        <Input id="target_id" bind:value={form.target_id} />
      </div>
      <div class="space-y-2">
        <Label for="storage_key">存储键</Label>
        <Input id="storage_key" bind:value={form.storage_key} />
      </div>
      <div class="space-y-2">
        <Label for="content_type">内容类型</Label>
        <Input id="content_type" bind:value={form.content_type} placeholder="application/pdf / image/png / ..." />
      </div>
      <div class="space-y-2">
        <Label for="size_bytes">文件大小(字节)</Label>
        <Input id="size_bytes" type="number" bind:value={form.size_bytes} />
      </div>
      <div class="space-y-2">
        <Label for="uploaded_by_id">上传者ID</Label>
        <Input id="uploaded_by_id" bind:value={form.uploaded_by_id} />
      </div>
      <div class="space-y-2">
        <Label for="status">状态</Label>
        <Input id="status" bind:value={form.status} />
      </div>
      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="remarks">备注</Label>
        <Input id="remarks" bind:value={form.remarks} />
      </div>
    </Card.Content>
  </Card.Root>
</div>
