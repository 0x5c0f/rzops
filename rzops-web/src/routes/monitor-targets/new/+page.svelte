<script lang="ts">
  import { goto } from '$app/navigation';
  import { monitorTargetsApi } from '$lib/api/monitor-targets';
  import type { CreateMonitorTargetRequest } from '$lib/types/monitor_target';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { monitorTypeOptions, commonStatusOptions } from '$lib/utils/enum-options';

  let saving = $state(false);
  let form = $state<CreateMonitorTargetRequest>({
    name: '',
    monitor_type: '',
    endpoint: '',
    interval_seconds: undefined,
    remarks: '',
    status: 'active',
  });

  async function handleSave() {
    saving = true;
    try {
      await monitorTargetsApi.create(form);
      goto('/monitor-targets');
    } catch (err) {
      console.error('Failed to create monitor-target:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '监控目标', href: '/monitor-targets' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建监控目标</h1>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => goto('/monitor-targets')}>取消</Button>
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
</div>
