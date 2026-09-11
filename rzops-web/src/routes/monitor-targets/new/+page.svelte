<script lang="ts">
  import { goto } from '$app/navigation';
  import { monitorTargetsApi } from '$lib/api/monitor-targets';
  import type { CreateMonitorTargetRequest } from '$lib/types/monitor_target';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import MonitorTargetForm from '$lib/components/forms/MonitorTargetForm.svelte';

  async function handleCreate(data: CreateMonitorTargetRequest) {
    const res = await monitorTargetsApi.create(data);
    goto('/monitor-targets');
    return res.id;
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '监控目标', href: '/monitor-targets' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建监控目标</h1>
  </div>

  <MonitorTargetForm submitLabel="创建" onSubmit={handleCreate} />
</div>
