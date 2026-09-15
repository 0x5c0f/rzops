<script lang="ts">
  import type { CreateMonitorTargetRequest } from '$lib/types/monitor_target';
import AttachmentFormSection from '$lib/components/shared/AttachmentFormSection.svelte';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import RemoteSearchSelect from '$lib/components/shared/RemoteSearchSelect.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { monitorTypeOptions, commonStatusOptions, assetTargetTypeOptions, getOptionLabel } from '$lib/utils/enum-options';
  import {
    searchServerOptions,
    searchDatabaseInstanceOptions,
    searchOpsSiteOptions,
    searchDomainOptions,
    searchCertificateOptions,
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

  // svelte-ignore state_referenced_locally —— 仅初始化用一次，有意读取初始值
  const initialSnapshot = $state.snapshot(initial);

  let saving = $state(false);
  let formError = $state<string | null>(null);
  let attachmentRef = $state<{ uploadAll: (id: string) => Promise<void> } | null>(null);
  let targetDisplayOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateMonitorTargetRequest>(createInitial(initialSnapshot));

  // 根据目标类型获取远程搜索函数（单选，与数据库实例编辑页服务器选择一致）
  let targetSearchFn = $derived.by(() => {
    const map: Record<string, (keyword: string) => Promise<{ label: string; value: string }[]>> = {
      server: searchServerOptions,
      database: searchDatabaseInstanceOptions,
      site: searchOpsSiteOptions,
      domain: searchDomainOptions,
      certificate: searchCertificateOptions,
    };
    return map[form.target_type] || null;
  });

  let targetPlaceholder = $derived.by(() => {
    const map: Record<string, string> = { server: '选择服务器', database: '选择数据库实例', site: '选择站点', domain: '选择域名', certificate: '选择证书' };
    return map[form.target_type] || '选择目标对象';
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

  // 目标类型改变时清空已选目标（仅用户切换时触发；首跑与初始值相同则不清空，避免编辑回显丢失）
  let prevTargetType = $state(form.target_type);
  $effect(() => {
    const t = form.target_type;
    if (t !== prevTargetType) {
      prevTargetType = t;
      form.target_id = '';
      targetDisplayOptions = [];
    }
  });

  // 编辑时初始化已选目标：解析 id 为真实名称，避免先显示 id
  onMount(async () => {
    if (form.target_id && targetSearchFn) {
      const opts = await targetSearchFn('');
      const found = opts.find(o => o.value === form.target_id);
      targetDisplayOptions = found ? [found] : [{ label: form.target_id, value: form.target_id }];
    }
  });

  async function handleSave() {
    formError = validate([
      { value: form.name, label: '监控目标名称', required: true, maxLength: 100 },
      { value: form.monitor_type, label: '监控类型', required: true },
      { value: form.endpoint, label: '监控地址', maxLength: 500 },
      { value: form.interval_seconds, label: '间隔时间(秒)', format: 'positiveNumber' },
    ]);
    if (formError) return;
    saving = true;
    try {
      const id = await onSubmit({
        ...form,
        target_id: form.target_id || undefined,
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

      <FormSelect
        label="监控类型"
        bind:value={form.monitor_type}
        options={$monitorTypeOptions}
        placeholder="选择监控类型"
        required
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
      {#if targetSearchFn}
        <RemoteSearchSelect
          label="目标对象"
          bind:value={form.target_id}
          searchFn={targetSearchFn}
          displayOptions={targetDisplayOptions}
          placeholder={targetPlaceholder}
          searchPlaceholder="输入名称搜索..."
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

