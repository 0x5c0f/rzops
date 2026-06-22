<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { contractsApi } from '$lib/api/contracts';
  import type { ContractResponse, UpdateContractRequest } from '$lib/types/contract';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { onMount } from 'svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { contractStatusOptions } from '$lib/utils/enum-options';

  let contract = $state<ContractResponse | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let form = $state<UpdateContractRequest>({});

  onMount(async () => {
    try {
      contract = await contractsApi.getById($page.params.id ?? "");
      form = {
        name: contract.name,
        provider_id: contract.provider_id ?? undefined,
        contract_no: contract.contract_no ?? undefined,
        start_date: contract.start_date ?? undefined,
        end_date: contract.end_date ?? undefined,
        amount: contract.amount ?? undefined,
        currency: contract.currency ?? undefined,
        status: contract.status,
        subject_type: contract.subject_type ?? undefined,
        subject_id: contract.subject_id ?? undefined,
        remarks: contract.remarks ?? undefined,
      };
    } catch (err) {
      console.error('Failed to load contract:', err);
      goto('/contracts');
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    if (!contract) return;
    saving = true;
    try {
      await contractsApi.update(contract.id, form);
      goto('/contracts');
    } catch (err) {
      console.error('Failed to save contract:', err);
    } finally {
      saving = false;
    }
  }

  async function handleDelete() {
    if (!contract) return;
    if (!confirm(`确定要删除合同 "${contract.name}" 吗？`)) return;
    try {
      await contractsApi.delete(contract.id);
      goto('/contracts');
    } catch (err) {
      console.error('Failed to delete contract:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '合同', href: '/contracts' },
    { label: contract?.name || '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if contract}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{contract.name}</h1>
        <StatusBadge status={contract.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
        <Button onclick={handleSave} disabled={saving}>
          {saving ? '保存中...' : '保存'}
        </Button>
      </div>
    </div>

    <Card.Root>
      <Card.Header>
        <Card.Title>基本信息</Card.Title>
      </Card.Header>
      <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <div class="space-y-2">
          <Label for="name">名称</Label>
          <Input id="name" bind:value={form.name} />
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
  {/if}
</div>
