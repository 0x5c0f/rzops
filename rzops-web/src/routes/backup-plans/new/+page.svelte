<script lang="ts">
  import { goto } from '$app/navigation';
  import { backupPlansApi } from '$lib/api/backup-plans';
  import type { CreateBackupPlanRequest } from '$lib/types/backup_plan';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { commonStatusOptions } from '$lib/utils/enum-options';

  let saving = $state(false);
  let form = $state<CreateBackupPlanRequest>({
    name: '',
    target_type: '',
    schedule: '',
    retention_days: undefined,
    remarks: '',
    status: 'active',
  });

  async function handleSave() {
    saving = true;
    try {
      await backupPlansApi.create(form);
      goto('/backup-plans');
    } catch (err) {
      console.error('Failed to create backup-plan:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '备份计划', href: '/backup-plans' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建备份计划</h1>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => goto('/backup-plans')}>取消</Button>
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
        <Label for="target_type">目标类型</Label>
        <Input id="target_type" bind:value={form.target_type} placeholder="数据库 / 文件 / ..." />
      </div>
      <div class="space-y-2">
        <Label for="schedule">调度计划</Label>
        <Input id="schedule" bind:value={form.schedule} placeholder="cron 表达式" />
      </div>
      <div class="space-y-2">
        <Label for="retention_days">保留天数</Label>
        <Input id="retention_days" type="number" bind:value={form.retention_days} />
      </div>
      <FormSelect label="状态" bind:value={form.status} options={commonStatusOptions} />
      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="remarks">备注</Label>
        <Input id="remarks" bind:value={form.remarks} />
      </div>
    </Card.Content>
  </Card.Root>
</div>
