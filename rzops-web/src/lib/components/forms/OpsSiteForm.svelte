<script lang="ts">
  import type { CreateOpsSiteRequest } from '$lib/types/ops_site';
import AttachmentFormSection from '$lib/components/shared/AttachmentFormSection.svelte';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import DateField from '$lib/components/shared/DateField.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { siteStatusOptions, importanceOptions, serviceTargetOptions, codeRepoTypeOptions } from '$lib/utils/enum-options';
  import { getBackupPlanOptions, getMonitorTargetOptions } from '$lib/utils/entity-options';
  import { onMount } from 'svelte';

  let {
    initial = {} as CreateOpsSiteRequest,
    entityId = '',
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateOpsSiteRequest;
    submitLabel?: string;
    onSubmit: (data: CreateOpsSiteRequest) => Promise<string | void>;
  } = $props();

  let saving = $state(false);
  let attachmentRef = $state<{ uploadAll: (id: string) => Promise<void> } | null>(null);
  let backupPlanOptions = $state<{ label: string; value: string }[]>([]);
  let monitorTargetOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateOpsSiteRequest>(createInitial(initial));

  function createInitial(initial?: CreateOpsSiteRequest): CreateOpsSiteRequest {
    return {
      name: '',
      url: '',
      service_target: '',
      importance: '',
      purpose: '',
      language_runtime: '',
      web_framework: '',
      code_repo_type: '',
      code_repo_url: '',
      function_summary: '',
      remarks: '',
      status: 'active',
      is_test_site: false,
      ...structuredClone(initial ?? {}),
    };
  }

  // 是否处于下线状态（临时/永久下线时才需要填写下线时间与原因）
  let isOffline = $derived(
    form.status === 'temporary_offline' || form.status === 'permanent_offline',
  );

  onMount(async () => {
    const [backups, monitors] = await Promise.all([
      getBackupPlanOptions(),
      getMonitorTargetOptions(),
    ]);
    backupPlanOptions = backups;
    monitorTargetOptions = monitors;
  });

  async function handleSave() {
    saving = true;
    try {
      const id = await onSubmit(form);
      if (id) await attachmentRef?.uploadAll(id);
    } catch (err) {
      console.error('Failed to save ops site:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
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
        label="状态"
        bind:value={form.status}
        options={$siteStatusOptions}
        required
      />

      <div class="space-y-2">
        <Label for="url">URL <span class="text-destructive">*</span></Label>
        <Input id="url" bind:value={form.url} required placeholder="https://..." />
      </div>

      <FormSelect
        label="服务目标"
        bind:value={form.service_target}
        options={$serviceTargetOptions}
        placeholder="选择服务目标"
      />

      <FormSelect
        label="重要性"
        bind:value={form.importance}
        options={$importanceOptions}
        placeholder="选择重要性"
      />

      <div class="space-y-2">
        <Label for="purpose">用途</Label>
        <Input id="purpose" bind:value={form.purpose} />
      </div>

      <div class="space-y-2">
        <Label for="language_runtime">语言/运行时</Label>
        <Input id="language_runtime" bind:value={form.language_runtime} />
      </div>

      <div class="space-y-2">
        <Label for="web_framework">Web框架</Label>
        <Input id="web_framework" bind:value={form.web_framework} />
      </div>

      <FormSelect
        label="代码仓库类型"
        bind:value={form.code_repo_type}
        options={$codeRepoTypeOptions}
        placeholder="选择仓库类型"
      />

      <div class="space-y-2">
        <Label for="code_repo_url">代码仓库地址</Label>
        <Input id="code_repo_url" bind:value={form.code_repo_url} />
      </div>

      <FormSelect
        label="备份计划"
        bind:value={form.backup_plan_id}
        options={backupPlanOptions}
        placeholder="选择备份计划"
      />

      <FormSelect
        label="监控目标"
        bind:value={form.monitor_target_id}
        options={monitorTargetOptions}
        placeholder="选择监控目标"
      />

      <div class="flex items-center gap-4 pt-6">
        <div class="flex items-center gap-2">
          <input id="is_test_site" type="checkbox" bind:checked={form.is_test_site} class="h-4 w-4 rounded border-gray-300" />
          <Label for="is_test_site">测试站点</Label>
        </div>
      </div>

      {#if isOffline}
        <DateField
          id="offline_time"
          label="下线时间"
          bind:value={form.offline_time}
        />

        <div class="space-y-2">
          <Label for="offline_reason">下线原因</Label>
          <Input id="offline_reason" bind:value={form.offline_reason} placeholder="说明下线原因" />
        </div>
      {/if}

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="function_summary">功能概述</Label>
        <TextArea id="function_summary" bind:value={form.function_summary} rows={3} />
      </div>

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="remarks">备注</Label>
        <TextArea id="remarks" bind:value={form.remarks} rows={3} />
      </div>
    </Card.Content>
  </Card.Root>

    <AttachmentFormSection bind:this={attachmentRef} targetType="site" targetId={entityId} />

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
