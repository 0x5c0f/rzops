<script lang="ts">
  import { goto } from '$app/navigation';
  import { contractsApi } from '$lib/api/contracts';
  import type { CreateContractRequest } from '$lib/types/contract';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { contractStatusOptions } from '$lib/utils/enum-options';

  let saving = $state(false);
  let form = $state<CreateContractRequest>({
    name: '',
    provider_id: '',
    contract_no: '',
    start_date: '',
    end_date: '',
    amount: '',
    currency: '',
    status: 'active',
    subject_type: '',
    subject_id: '',
    remarks: '',
  });

  async function handleSave() {
    saving = true;
    try {
      await contractsApi.create(form);
      goto('/contracts');
    } catch (err) {
      console.error('Failed to create contract:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '合同', href: '/contracts' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建合同</h1>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => goto('/contracts')}>取消</Button>
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
        <Label for="name">名称 *</Label>
        <Input id="name" bind:value={form.name} required />
      </div>
      <div class="space-y-2">
        <Label for="provider_id">供应商ID</Label>
        <Input id="provider_id" bind:value={form.provider_id} />
      </div>
      <div class="space-y-2">
        <Label for="contract_no">合同编号</Label>
        <Input id="contract_no" bind:value={form.contract_no} />
      </div>
      <div class="space-y-2">
        <Label for="start_date">开始日期</Label>
        <Input id="start_date" type="date" bind:value={form.start_date} />
      </div>
      <div class="space-y-2">
        <Label for="end_date">结束日期</Label>
        <Input id="end_date" type="date" bind:value={form.end_date} />
      </div>
      <div class="space-y-2">
        <Label for="amount">金额</Label>
        <Input id="amount" bind:value={form.amount} />
      </div>
      <div class="space-y-2">
        <Label for="currency">货币</Label>
        <Input id="currency" bind:value={form.currency} placeholder="CNY / USD / ..." />
      </div>
      <FormSelect label="状态" bind:value={form.status} options={contractStatusOptions} />
      <div class="space-y-2">
        <Label for="subject_type">主体类型</Label>
        <Input id="subject_type" bind:value={form.subject_type} />
      </div>
      <div class="space-y-2">
        <Label for="subject_id">主体ID</Label>
        <Input id="subject_id" bind:value={form.subject_id} />
      </div>
      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="remarks">备注</Label>
        <Input id="remarks" bind:value={form.remarks} />
      </div>
    </Card.Content>
  </Card.Root>
</div>
