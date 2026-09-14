<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { monitorTargetsApi } from '$lib/api/monitor-targets';
  import type { CreateMonitorTargetRequest, MonitorTargetResponse } from '$lib/types/monitor_target';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import MonitorTargetForm from '$lib/components/forms/MonitorTargetForm.svelte';
  import { onMount } from 'svelte';

  let target = $state<MonitorTargetResponse | null>(null);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/monitor-targets'); return; }
    try {
      target = await monitorTargetsApi.getById(id);
    } catch (err) {
      console.error('Failed to load monitor target:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(t: MonitorTargetResponse): CreateMonitorTargetRequest {
    return {
      name: t.name,
      target_type: t.target_type ?? '',
      target_id: t.target_id ?? '',
      monitor_type: t.monitor_type ?? '',
      endpoint: t.endpoint ?? '',
      interval_seconds: t.interval_seconds ?? undefined,
      status: t.status || 'active',
      remarks: t.remarks ?? '',
    };
  }

  async function handleUpdate(data: CreateMonitorTargetRequest) {
    const id = $page.params.id;
    if (!id) return;
    await monitorTargetsApi.update(id, data);
    goto(`/monitor-targets/${id}`);
    return id;
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '监控目标', href: '/monitor-targets' },
    { label: target?.name || '详情', href: target ? `/monitor-targets/${target.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑监控目标</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !target}
    <p class="text-sm text-muted-foreground">加载失败，监控目标可能不存在。</p>
  {:else}
    <MonitorTargetForm
      initial={toForm(target)}
      entityId={target.id}
      submitLabel="保存"
      onSubmit={handleUpdate}
    />
  {/if}
</div>
