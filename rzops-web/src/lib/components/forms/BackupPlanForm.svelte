<script lang="ts">
  import type { CreateBackupPlanRequest } from '$lib/types/backup_plan';
import AttachmentFormSection from '$lib/components/shared/AttachmentFormSection.svelte';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TableSelectModal from '$lib/components/shared/TableSelectModal.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { commonStatusOptions, backupTargetTypeOptions, databaseStatusOptions, siteStatusOptions, serverStatusOptions, serverTypeOptions, getOptionLabel } from '$lib/utils/enum-options';
  import {
    searchServerPaginated,
    searchDatabasePaginated,
    searchSitePaginated,
  } from '$lib/utils/entity-options';
  import { validate } from '$lib/utils/validation';
  import { onMount } from 'svelte';

  let {
    initial = {} as CreateBackupPlanRequest,
    entityId = '',
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateBackupPlanRequest;
    submitLabel?: string;
    onSubmit: (data: CreateBackupPlanRequest) => Promise<string | void>;
  } = $props();

  // svelte-ignore state_referenced_locally —— 表单仅初始化一次，有意读取 prop 初始值
  const initialSnapshot = $state.snapshot(initial);

  let saving = $state(false);
  let formError = $state<string | null>(null);
  let attachmentRef = $state<{ uploadAll: (id: string) => Promise<void> } | null>(null);
  let targetIds = $state<string[]>([]);
  let displayTargetOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateBackupPlanRequest>(createInitial(initialSnapshot));

  // 根据目标类型获取搜索函数和列配置
  let targetSearchFn = $derived.by(() => {
    const map: Record<string, (keyword: string, page: number, perPage: number) => Promise<{ data: Record<string, unknown>[]; total: number }>> = {
      server: searchServerPaginated,
      database: searchDatabasePaginated,
      site: searchSitePaginated,
    };
    return map[form.target_type] || null;
  });

  let targetColumns = $derived.by(() => {
    if (form.target_type === 'server') {
      return [
        { key: 'name', label: '服务器名称' },
        { key: 'primary_ip', label: '主IP', width: 'w-32' },
        { key: 'status', label: '状态', width: 'w-20', render: (item: Record<string, unknown>) => getOptionLabel($serverStatusOptions, String(item.status ?? '')) },
        { key: 'server_type', label: '类型', width: 'w-24', render: (item: Record<string, unknown>) => getOptionLabel($serverTypeOptions, String(item.server_type ?? '')) },
      ];
    }
    if (form.target_type === 'database') {
      return [
        { key: 'name', label: '实例名称' },
        { key: 'db_type', label: '类型', width: 'w-24' },
        { key: 'port', label: '端口', width: 'w-20' },
        { key: 'status', label: '状态', width: 'w-20', render: (item: Record<string, unknown>) => getOptionLabel($databaseStatusOptions, String(item.status ?? '')) },
      ];
    }
    if (form.target_type === 'site') {
      return [
        { key: 'name', label: '站点名称' },
        { key: 'url', label: 'URL', width: 'w-48' },
        { key: 'status', label: '状态', width: 'w-20', render: (item: Record<string, unknown>) => getOptionLabel($siteStatusOptions, String(item.status ?? '')) },
      ];
    }
    return [];
  });

  let targetModalTitle = $derived.by(() => {
    const map: Record<string, string> = { server: '选择服务器', database: '选择数据库实例', site: '选择站点' };
    return map[form.target_type] || '选择目标';
  });

  function createInitial(initial?: CreateBackupPlanRequest): CreateBackupPlanRequest {
    return {
      name: '',
      target_type: '',
      schedule: '',
      status: 'active',
      ...JSON.parse(JSON.stringify(initial ?? {})),
    };
  }

  // 目标类型改变时清空已选目标
  $effect(() => {
    if (form.target_type) {
      targetIds = [];
      displayTargetOptions = [];
    }
  });

  // 编辑时初始化已选目标
  onMount(() => {
    if (form.target_id) {
      targetIds = [form.target_id];
      displayTargetOptions = [{ label: form.target_id, value: form.target_id }];
    }
  });

  async function handleSave() {
    formError = validate([
      { value: form.name, label: '备份计划名称', required: true, maxLength: 100 },
      { value: form.target_type, label: '目标类型', required: true },
      { value: form.schedule, label: '执行计划', required: true, maxLength: 100 },
      { value: form.retention_days, label: '保留天数', format: 'positiveNumber' },
    ]);
    if (formError) return;
    saving = true;
    try {
      const id = await onSubmit({
        ...form,
        target_id: targetIds[0] || undefined,
        target_type: form.target_type || undefined,
      });
      if (id) await attachmentRef?.uploadAll(id);
    } catch (err) {
      console.error('Failed to save backup plan:', err);
      formError = '保存失败，请重试';
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  {#if formError}
    <div class="rounded-md border border-destructive/50 bg-destructive/10 px-4 py-3 text-sm text-destructive">
      {formError}
    </div>
  {/if}

  <div class="flex items-center justify-between">
    <div></div>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => history.back()}>取消</Button>
      <Button onclick={handleSave} disabled={saving}>
        {saving ? '保存中...' : submitLabel}
      </Button>
    </div>
  </div>

<Card.Root>
    <Card.Header>
      <Card.Title>基本信息</Card.Title>
    </Card.Header>
    <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <div class="space-y-2">
        <Label for="name">名称 <span class="text-destructive">*</span></Label>
        <Input id="name" bind:value={form.name} required />
      </div>

      <div class="space-y-2">
        <Label for="schedule">调度计划</Label>
        <Input id="schedule" bind:value={form.schedule} placeholder="cron 表达式" />
      </div>

      <div class="space-y-2">
        <Label for="retention_days">保留天数</Label>
        <Input id="retention_days" type="number" bind:value={form.retention_days} />
      </div>

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={$commonStatusOptions}
      />

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="remarks">备注</Label>
        <TextArea id="remarks" bind:value={form.remarks} rows={3} />
      </div>
    </Card.Content>
  </Card.Root>

  <Card.Root>
    <Card.Header>
      <Card.Title>关联信息</Card.Title>
      <Card.Description>该备份计划的备份对象（可选）</Card.Description>
    </Card.Header>
    <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <FormSelect
        label="目标类型"
        bind:value={form.target_type}
        options={$backupTargetTypeOptions}
        placeholder="选择目标类型"
      />
      {#if targetSearchFn && targetColumns.length > 0}
        <TableSelectModal
          label="目标对象"
          multiple={false}
          bind:value={targetIds}
          searchFn={targetSearchFn}
          displayOptions={displayTargetOptions}
          placeholder="选择目标对象"
          searchPlaceholder="输入名称搜索..."
          modalTitle={targetModalTitle}
          columns={targetColumns}
        />
      {/if}
    </Card.Content>
  </Card.Root>

    <AttachmentFormSection bind:this={attachmentRef} targetType="backup_plan" targetId={entityId} />

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
