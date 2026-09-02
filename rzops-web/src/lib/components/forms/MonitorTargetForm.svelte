<script lang="ts">
  import type { CreateMonitorTargetRequest } from '$lib/types/monitor_target';
import AttachmentFormSection from '$lib/components/shared/AttachmentFormSection.svelte';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TableSelectModal from '$lib/components/shared/TableSelectModal.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { monitorTypeOptions, commonStatusOptions, assetTargetTypeOptions, serverStatusOptions, serverTypeOptions, databaseStatusOptions, siteStatusOptions, certificateStatusOptions, getOptionLabel } from '$lib/utils/enum-options';
  import {
    searchServerPaginated,
    searchDatabasePaginated,
    searchSitePaginated,
    searchDomainPaginated,
    searchCertificatePaginated,
  } from '$lib/utils/entity-options';
  import { validate } from '$lib/utils/validation';
  import { onMount } from 'svelte';

  let {
    initial = {} as CreateMonitorTargetRequest,
    entityId = '',
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateMonitorTargetRequest;
    submitLabel?: string;
    onSubmit: (data: CreateMonitorTargetRequest) => Promise<string | void>;
  } = $props();

  let saving = $state(false);
  let formError = $state<string | null>(null);
  let attachmentRef = $state<{ uploadAll: (id: string) => Promise<void> } | null>(null);
  let targetIds = $state<string[]>([]);
  let displayTargetOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateMonitorTargetRequest>(createInitial(initial));

  // 根据目标类型获取搜索函数和列配置
  let targetSearchFn = $derived.by(() => {
    const map: Record<string, (keyword: string, page: number, perPage: number) => Promise<{ data: Record<string, unknown>[]; total: number }>> = {
      server: searchServerPaginated,
      database: searchDatabasePaginated,
      site: searchSitePaginated,
      domain: searchDomainPaginated,
      certificate: searchCertificatePaginated,
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
    if (form.target_type === 'domain') {
      return [
        { key: 'name', label: '域名' },
        { key: 'registrar', label: '注册商', width: 'w-32' },
        { key: 'expire_date', label: '到期时间', width: 'w-28' },
      ];
    }
    if (form.target_type === 'certificate') {
      return [
        { key: 'name', label: '证书名称' },
        { key: 'issuer', label: '颁发者', width: 'w-32' },
        { key: 'expire_date', label: '到期时间', width: 'w-28' },
      ];
    }
    return [];
  });

  let targetModalTitle = $derived.by(() => {
    const map: Record<string, string> = { server: '选择服务器', database: '选择数据库实例', site: '选择站点', domain: '选择域名', certificate: '选择证书' };
    return map[form.target_type] || '选择目标';
  });

  function createInitial(initial?: CreateMonitorTargetRequest): CreateMonitorTargetRequest {
    return {
      name: '',
      target_type: '',
      target_id: '',
      monitor_type: '',
      endpoint: '',
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
      { value: form.name, label: '监控目标名称', required: true, maxLength: 100 },
      { value: form.target_type, label: '目标类型', required: true },
      { value: form.monitor_type, label: '监控类型', required: true },
      { value: form.endpoint, label: '监控地址', maxLength: 500 },
      { value: form.interval_seconds, label: '间隔时间(秒)', format: 'positiveNumber' },
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
      console.error('Failed to save monitor target:', err);
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
  <Card.Root>
    <Card.Header>
      <Card.Title>基本信息</Card.Title>
    </Card.Header>
    <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <div class="space-y-2">
        <Label for="name">名称 <span class="text-destructive">*</span></Label>
        <Input id="name" bind:value={form.name} required />
      </div>

      <FormSelect
        label="监控类型"
        bind:value={form.monitor_type}
        options={$monitorTypeOptions}
        placeholder="选择监控类型"
      />

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={$commonStatusOptions}
      />

      <div class="space-y-2">
        <Label for="endpoint">端点</Label>
        <Input id="endpoint" bind:value={form.endpoint} placeholder="URL / IP:Port / ..." />
      </div>

      <div class="space-y-2">
        <Label for="interval_seconds">间隔(秒)</Label>
        <Input id="interval_seconds" type="number" bind:value={form.interval_seconds} />
      </div>

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="remarks">备注</Label>
        <TextArea id="remarks" bind:value={form.remarks} rows={3} />
      </div>
    </Card.Content>
  </Card.Root>

  <Card.Root>
    <Card.Header>
      <Card.Title>关联信息</Card.Title>
      <Card.Description>该监控目标监控的具体对象（可选）</Card.Description>
    </Card.Header>
    <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <FormSelect
        label="目标类型"
        bind:value={form.target_type}
        options={$assetTargetTypeOptions}
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

    <AttachmentFormSection bind:this={attachmentRef} targetType="monitor_target" targetId={entityId} />

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>

