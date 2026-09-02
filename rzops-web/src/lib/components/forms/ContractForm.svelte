<script lang="ts">
  import type { CreateContractRequest } from '$lib/types/contract';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import DateField from '$lib/components/shared/DateField.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { contractStatusOptions } from '$lib/utils/enum-options';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { validate, validateDateRange } from '$lib/utils/validation';
  import { onMount } from 'svelte';

  let {
    initial = {} as CreateContractRequest,
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateContractRequest;
    submitLabel?: string;
    onSubmit: (data: CreateContractRequest) => Promise<void>;
  } = $props();

  let saving = $state(false);
  let formError = $state<string | null>(null);
  let providerOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateContractRequest>(createInitial(initial));

  function createInitial(initial?: CreateContractRequest): CreateContractRequest {
    return {
      name: '',
      currency: 'CNY',
      status: 'active',
      ...JSON.parse(JSON.stringify(initial ?? {})),
    };
  }

  onMount(async () => {
    providerOptions = await getProviderOptions();
  });

  async function handleSave() {
    formError = validate([
      { value: form.name, label: '合同名称', required: true, maxLength: 200 },
      { value: form.contract_no, label: '合同编号', maxLength: 100 },
      { value: form.amount, label: '合同金额', format: 'positiveNumber' },
    ]);
    if (formError) return;
    formError = validateDateRange(form.start_date, form.end_date, '开始日期', '结束日期');
    if (formError) return;
    saving = true;
    try {
      await onSubmit(form);
    } catch (err) {
      console.error('Failed to save contract:', err);
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
        label="供应商"
        bind:value={form.provider_id}
        options={providerOptions}
        placeholder="选择供应商"
      />

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={$contractStatusOptions}
      />

      <div class="space-y-2">
        <Label for="contract_no">合同编号</Label>
        <Input id="contract_no" bind:value={form.contract_no} />
      </div>

      <DateField
        id="start_date"
        label="开始日期"
        bind:value={form.start_date}
        max={form.end_date || undefined}
      />

      <DateField
        id="end_date"
        label="结束日期"
        bind:value={form.end_date}
        min={form.start_date || undefined}
      />

      <div class="space-y-2">
        <Label for="amount">金额</Label>
        <Input id="amount" bind:value={form.amount} />
      </div>

      <div class="space-y-2">
        <Label for="currency">货币</Label>
        <Input id="currency" bind:value={form.currency} placeholder="CNY / USD / ..." />
      </div>

      <div class="space-y-2">
        <Label for="subject_type">主体类型</Label>
        <Input id="subject_type" bind:value={form.subject_type} placeholder="如 server / site" />
      </div>

      <div class="space-y-2">
        <Label for="subject_id">主体ID</Label>
        <Input id="subject_id" bind:value={form.subject_id} />
      </div>

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="remarks">备注</Label>
        <TextArea id="remarks" bind:value={form.remarks} rows={3} />
      </div>
    </Card.Content>
  </Card.Root>

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
