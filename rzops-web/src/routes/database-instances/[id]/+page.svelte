<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import type { DatabaseInstanceResponse, UpdateDatabaseInstanceRequest } from '$lib/types/database_instance';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { databaseStatusOptions, databaseTypeOptions, importanceOptions } from '$lib/utils/enum-options';
  import { getServerOptions, ensureOption } from '$lib/utils/entity-options';
  import { onMount } from 'svelte';

  let dbInstance = $state<DatabaseInstanceResponse | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let form = $state<UpdateDatabaseInstanceRequest>({});
  let serverOptions = $state<{ label: string; value: string }[]>([]);

  onMount(async () => {
    try {
      const [instance, servers] = await Promise.all([
        databaseInstancesApi.getById($page.params.id ?? ""),
        getServerOptions(),
      ]);
      dbInstance = instance;
      serverOptions = ensureOption(servers, dbInstance.server_id ?? undefined, dbInstance.server_id ?? undefined);
      form = {
        name: dbInstance.name,
        db_type: dbInstance.db_type,
        server_id: dbInstance.server_id ?? undefined,
        description: dbInstance.description ?? undefined,
        status: dbInstance.status,
        offline_time: dbInstance.offline_time ?? undefined,
        is_self_installed: dbInstance.is_self_installed ?? undefined,
        importance: dbInstance.importance ?? undefined,
        is_ops_managed: dbInstance.is_ops_managed ?? undefined,
        management_credential_id: dbInstance.management_credential_id ?? undefined,
        backup_plan_id: dbInstance.backup_plan_id ?? undefined,
        monitor_target_id: dbInstance.monitor_target_id ?? undefined,
        port: dbInstance.port ?? undefined,
        instance_name: dbInstance.instance_name ?? undefined,
      };
    } catch (err) {
      console.error('Failed to load database instance:', err);
      goto('/database-instances');
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    if (!dbInstance) return;
    saving = true;
    try {
      await databaseInstancesApi.update(dbInstance.id, form);
      goto('/database-instances');
    } catch (err) {
      console.error('Failed to save database instance:', err);
    } finally {
      saving = false;
    }
  }

  async function handleDelete() {
    if (!dbInstance) return;
    if (!confirm(`确定要删除数据库实例 "${dbInstance.name}" 吗？`)) return;
    try {
      await databaseInstancesApi.delete(dbInstance.id);
      goto('/database-instances');
    } catch (err) {
      console.error('Failed to delete database instance:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '数据库实例', href: '/database-instances' },
    { label: dbInstance?.name || '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if dbInstance}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{dbInstance.name}</h1>
        <StatusBadge status={dbInstance.status} />
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
        <FormSelect label="状态" bind:value={form.status} options={databaseStatusOptions} required />
        <FormSelect label="数据库类型" bind:value={form.db_type} options={databaseTypeOptions} />
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
          <Label for="offline_time">下线时间</Label>
          <Input id="offline_time" type="datetime-local" bind:value={form.offline_time} />
        </div>
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
  {/if}
</div>
