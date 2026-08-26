<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { contractsApi } from '$lib/api/contracts';
  import type { ContractResponse } from '$lib/types/contract';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';

  let contract = $state<ContractResponse | null>(null);
  let loading = $state(true);
  let providerMap = $state<Record<string, string>>({});

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/contracts'); return; }

    try {
      const [contractData, provOptions] = await Promise.all([
        contractsApi.getById(id),
        getProviderOptions(),
      ]);
      contract = contractData;
      providerMap = Object.fromEntries(provOptions.map(o => [o.value, o.label]));
    } catch (err) {
      console.error('Failed to load contract:', err);
      goto('/contracts');
    } finally {
      loading = false;
    }
  });

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

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '合同', href: '/contracts' },
    { label: contract?.name || '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if contract}
    <!-- 页面标题 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold">{contract.name}</h1>
        <StatusBadge status={contract.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/contracts')}>返回列表</Button>
        <Button onclick={() => goto(`/contracts/${contract?.id}/edit`)}>编辑</Button>
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
      </div>
    </div>

    <div class="grid gap-6 lg:grid-cols-2">
      <!-- 基本信息 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>基本信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">名称</dt>
              <dd>{contract.name}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">供应商</dt>
              <dd>
                {#if contract.provider_id}
                  <a href="/providers/{contract.provider_id}" class="text-primary hover:underline">
                    {providerMap[contract.provider_id] || contract.provider_id}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">合同编号</dt>
              <dd class="font-mono">{contract.contract_no || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">主体类型</dt>
              <dd>{contract.subject_type || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">主体ID</dt>
              <dd class="font-mono">{contract.subject_id || '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>

      <!-- 财务与时间 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>财务与时间</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">开始日期</dt>
              <dd>{formatDate(contract.start_date)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">结束日期</dt>
              <dd>{formatDate(contract.end_date)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">金额</dt>
              <dd>{contract.amount ? `${contract.amount} ${contract.currency || ''}` : '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">货币</dt>
              <dd>{contract.currency || '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>
    </div>
  {/if}
</div>
