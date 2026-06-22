<script lang="ts">
  import { goto } from '$app/navigation';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import type { CreateDatabaseInstanceRequest } from '$lib/types/database_instance';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { databaseStatusOptions, databaseTypeOptions, importanceOptions } from '$lib/utils/enum-options';
  import { getServerOptions } from '$lib/utils/entity-options';
  import { onMount } from 'svelte';

  let saving = $state(false);
  let serverOptions = $state<{ label: string; value: string }[]>([]);
  let form = $state<CreateDatabaseInstanceRequest>({
    name: '',
    db_type: '',
    server_id: '',
    description: '',
    status: 'active',
    importance: '',
    instance_name: '',
    management_credential_id: '',
    backup_plan_id: '',
    monitor_target_id: '',
  });

  onMount(async () => {
    serverOptions = await getServerOptions();
  });

  async function handleSave() {
    saving = true;
    try {
      await databaseInstancesApi.create(form);
      goto('/database-instances');
    } catch (err) {
      console.error('Failed to create database instance:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '数据库实例', href: '/database-instances' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建数据库实例</h1>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => goto('/database-instances')}>取消</Button>
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
      <FormSelect label="状态" bind:value={form.status} options={databaseStatusOptions} required />
      <FormSelect label="数据库类型" bind:value={form.db_type} options={databaseTypeOptions} required />
      <FormSelect label="服务器" bind:value={form.server_id} options={serverOptions} />
      <div class="space-y-2">
        <Label for="instance_name">实例名称</Label>
        <Input id="instance_name" bind:value={form.instance_name} />
      </div>
      <div class="space-y-2">
        <Label for="port">端口</Label>
        <Input id="port" type="number" bind:value={form.port} />
      </div>
      <FormSelect label="重要性" bind:value={form.importance} options={importanceOptions} />
      <div class="space-y-2">
        <Label for="management_credential_id">管理凭证ID</Label>
        <Input id="management_credential_id" bind:value={form.management_credential_id} />
      </div>
      <div class="space-y-2">
        <Label for="backup_plan_id">备份计划ID</Label>
        <Input id="backup_plan_id" bind:value={form.backup_plan_id} />
      </div>
      <div class="space-y-2">
        <Label for="monitor_target_id">监控目标ID</Label>
        <Input id="monitor_target_id" bind:value={form.monitor_target_id} />
      </div>
      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="description">描述</Label>
        <Input id="description" bind:value={form.description} />
      </div>
    </Card.Content>
  </Card.Root>
</div>
