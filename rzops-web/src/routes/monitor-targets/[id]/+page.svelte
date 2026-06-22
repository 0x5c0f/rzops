<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { monitorTargetsApi } from '$lib/api/monitor-targets';
  import type { MonitorTargetResponse, UpdateMonitorTargetRequest } from '$lib/types/monitor_target';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { onMount } from 'svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { monitorTypeOptions, commonStatusOptions } from '$lib/utils/enum-options';

  let target = $state<MonitorTargetResponse | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let form = $state<UpdateMonitorTargetRequest>({});

  onMount(async () => {
    try {
      target = await monitorTargetsApi.getById($page.params.id ?? "");
      form = {
        name: target.name,
        monitor_type: target.monitor_type ?? undefined,
        endpoint: target.endpoint ?? undefined,
        interval_seconds: target.interval_seconds ?? undefined,
        remarks: target.remarks ?? undefined,
        status: target.status,
      };
    } catch (err) {
      console.error('Failed to load monitor-target:', err);
      goto('/monitor-targets');
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    if (!target) return;
    saving = true;
    try {
      await monitorTargetsApi.update(target.id, form);
      goto('/monitor-targets');
    } catch (err) {
      console.error('Failed to save monitor-target:', err);
    } finally {
      saving = false;
    }
  }

  async function handleDelete() {
    if (!target) return;
    if (!confirm(`确定要删除监控目标 "${target.name}" 吗？`)) return;
    try {
      await monitorTargetsApi.delete(target.id);
      goto('/monitor-targets');
    } catch (err) {
      console.error('Failed to delete monitor-target:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '监控目标', href: '/monitor-targets' },
    { label: target?.name || '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if target}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{target.name}</h1>
        <StatusBadge status={target.status} />
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
        <FormSelect label="监控类型" bind:value={form.monitor_type} options={monitorTypeOptions} />
        <div class="space-y-2">
          <Label for="endpoint">端点</Label>
          <Input id="endpoint" bind:value={form.endpoint} placeholder="URL / IP:Port / ..." />
        </div>
        <div class="space-y-2">
          <Label for="interval_seconds">间隔(秒)</Label>
          <Input id="interval_seconds" type="number" bind:value={form.interval_seconds} />
        </div>
        <FormSelect label="状态" bind:value={form.status} options={commonStatusOptions} />
        <div class="space-y-2 md:col-span-2 lg:col-span-3">
          <Label for="remarks">备注</Label>
          <Input id="remarks" bind:value={form.remarks} />
        </div>
      </Card.Content>
    </Card.Root>
  {/if}
</div>
