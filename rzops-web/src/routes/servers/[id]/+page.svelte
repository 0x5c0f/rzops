<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { serversApi } from '$lib/api/servers';
  import type { ServerResponse, UpdateServerRequest } from '$lib/types/server';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import FormMultiSelect from '$lib/components/shared/FormMultiSelect.svelte';
  import {
    serverTypeOptions,
    hostingTypeOptions,
    serverRoleOptions,
    serverStatusOptions,
    architectureOptions,
    raidLevelOptions,
    webServerSoftwareOptions,
  } from '$lib/utils/enum-options';
  import { getProviderOptions, getDataCenterOptions, ensureOption } from '$lib/utils/entity-options';
  import { onMount } from 'svelte';

  let server = $state<ServerResponse | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let form = $state<UpdateServerRequest>({});
  let dataCenterOptions = $state<{ label: string; value: string }[]>([]);
  let providerOptions = $state<{ label: string; value: string }[]>([]);

  onMount(async () => {
    const [providers, dataCenters] = await Promise.all([
      getProviderOptions(),
      getDataCenterOptions(),
    ]);
    providerOptions = providers;
    dataCenterOptions = dataCenters;
    try {
      server = await serversApi.getById($page.params.id ?? "");
      form = {
        name: server.name,
        asset_code: server.asset_code ?? undefined,
        primary_ip: server.primary_ip ?? undefined,
        location: server.location ?? undefined,
        server_type: server.server_type ?? undefined,
        status: server.status,
        data_center_id: server.data_center_id ?? undefined,
        hosting_type: server.hosting_type ?? undefined,
        isp_provider_id: server.isp_provider_id ?? undefined,
        server_provider_id: server.server_provider_id ?? undefined,
        software_provider_id: server.software_provider_id ?? undefined,
        is_dual_line: server.is_dual_line ?? false,
        cpu: server.cpu ?? undefined,
        memory_gb: server.memory_gb ?? undefined,
        disk_layout: server.disk_layout ?? undefined,
        hardware_config: server.hardware_config ?? undefined,
        is_raid: server.is_raid ?? false,
        raid_level: server.raid_level ?? undefined,
        is_database_server: server.is_database_server ?? false,
        role_tags: server.role_tags ?? undefined,
        web_server_type: server.web_server_type ?? undefined,
        operating_system: server.operating_system ?? undefined,
        brand: server.brand ?? undefined,
        architecture: server.architecture ?? undefined,
        lease_start_date: server.lease_start_date ?? undefined,
        lease_end_date: server.lease_end_date ?? undefined,
        price: server.price ?? undefined,
        price_currency: server.price_currency ?? undefined,
        warranty_info: server.warranty_info ?? undefined,
        remarks: server.remarks ?? undefined,
      };
      dataCenterOptions = ensureOption(dataCenterOptions, server.data_center_id ?? undefined, server.data_center_id ?? undefined);
      providerOptions = ensureOption(providerOptions, server.isp_provider_id ?? undefined, server.isp_provider_id ?? undefined);
      providerOptions = ensureOption(providerOptions, server.server_provider_id ?? undefined, server.server_provider_id ?? undefined);
      providerOptions = ensureOption(providerOptions, server.software_provider_id ?? undefined, server.software_provider_id ?? undefined);
    } catch (err) {
      console.error('Failed to load server:', err);
      goto('/servers');
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    if (!server) return;
    saving = true;
    try {
      await serversApi.update(server.id, form);
      goto('/servers');
    } catch (err) {
      console.error('Failed to save server:', err);
    } finally {
      saving = false;
    }
  }

  async function handleDelete() {
    if (!server) return;
    if (!confirm(`确定要删除服务器 "${server.name}" 吗？`)) return;
    try {
      await serversApi.delete(server.id);
      goto('/servers');
    } catch (err) {
      console.error('Failed to delete server:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '服务器', href: '/servers' },
    { label: server?.name || '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if server}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{server.name}</h1>
        <StatusBadge status={server.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
        <Button onclick={handleSave} disabled={saving}>
          {saving ? '保存中...' : '保存'}
        </Button>
      </div>
    </div>

    <!-- 基本信息 -->
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
          <Label for="asset_code">资产编号</Label>
          <Input id="asset_code" bind:value={form.asset_code} />
        </div>
        <div class="space-y-2">
          <Label for="primary_ip">主IP</Label>
          <Input id="primary_ip" bind:value={form.primary_ip} />
        </div>
        <FormSelect
          label="服务器类型"
          bind:value={form.server_type}
          options={serverTypeOptions}
          placeholder="选择服务器类型"
        />
        <FormSelect
          label="状态"
          bind:value={form.status}
          options={serverStatusOptions}
        />
        <FormSelect
          label="架构"
          bind:value={form.architecture}
          options={architectureOptions}
          placeholder="选择架构"
        />
        <div class="space-y-2">
          <Label for="location">位置</Label>
          <Input id="location" bind:value={form.location} />
        </div>
        <div class="space-y-2">
          <Label for="brand">品牌</Label>
          <Input id="brand" bind:value={form.brand} />
        </div>
        <div class="space-y-2">
          <Label for="operating_system">操作系统</Label>
          <Input id="operating_system" bind:value={form.operating_system} />
        </div>
      </Card.Content>
    </Card.Root>

    <!-- 关联信息 -->
    <Card.Root>
      <Card.Header>
        <Card.Title>关联信息</Card.Title>
      </Card.Header>
      <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <FormSelect
          label="数据中心"
          bind:value={form.data_center_id}
          options={dataCenterOptions}
          placeholder="选择数据中心"
        />
        <FormSelect
          label="托管类型"
          bind:value={form.hosting_type}
          options={hostingTypeOptions}
          placeholder="选择托管类型"
        />
        <FormSelect
          label="ISP供应商"
          bind:value={form.isp_provider_id}
          options={providerOptions}
          placeholder="选择ISP供应商"
        />
        <FormSelect
          label="服务器供应商"
          bind:value={form.server_provider_id}
          options={providerOptions}
          placeholder="选择服务器供应商"
        />
        <FormSelect
          label="软件供应商"
          bind:value={form.software_provider_id}
          options={providerOptions}
          placeholder="选择软件供应商"
        />
        <div class="flex items-center gap-2 pt-6">
          <input type="checkbox" id="is_dual_line" bind:checked={form.is_dual_line} class="h-4 w-4" />
          <Label for="is_dual_line">双线</Label>
        </div>
      </Card.Content>
    </Card.Root>

    <!-- 硬件配置 -->
    <Card.Root>
      <Card.Header>
        <Card.Title>硬件配置</Card.Title>
      </Card.Header>
      <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <div class="space-y-2">
          <Label for="cpu">CPU</Label>
          <Input id="cpu" bind:value={form.cpu} />
        </div>
        <div class="space-y-2">
          <Label for="memory_gb">内存(GB)</Label>
          <Input id="memory_gb" type="number" bind:value={form.memory_gb} />
        </div>
        <div class="space-y-2">
          <Label for="disk_layout">磁盘配置</Label>
          <Input id="disk_layout" bind:value={form.disk_layout} />
        </div>
        <div class="space-y-2">
          <Label for="hardware_config">硬件配置</Label>
          <Input id="hardware_config" bind:value={form.hardware_config} />
        </div>
        <div class="flex items-center gap-2 pt-6">
          <input type="checkbox" id="is_raid" bind:checked={form.is_raid} class="h-4 w-4" />
          <Label for="is_raid">RAID</Label>
        </div>
        {#if form.is_raid}
          <FormSelect
            label="RAID级别"
            bind:value={form.raid_level}
            options={raidLevelOptions}
            placeholder="选择RAID级别"
          />
        {/if}
        <div class="flex items-center gap-2 pt-6">
          <input type="checkbox" id="is_database_server" bind:checked={form.is_database_server} class="h-4 w-4" />
          <Label for="is_database_server">数据库服务器</Label>
        </div>
        <FormMultiSelect
          label="角色标签"
          bind:value={form.role_tags}
          options={serverRoleOptions}
          placeholder="选择角色"
        />
        <FormMultiSelect
          label="Web服务器软件"
          bind:value={form.web_server_type}
          options={webServerSoftwareOptions}
          placeholder="选择Web服务器"
        />
      </Card.Content>
    </Card.Root>

    <!-- 租赁信息 -->
    <Card.Root>
      <Card.Header>
        <Card.Title>租赁信息</Card.Title>
      </Card.Header>
      <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <div class="space-y-2">
          <Label for="lease_start_date">租赁开始日期</Label>
          <Input id="lease_start_date" type="date" bind:value={form.lease_start_date} />
        </div>
        <div class="space-y-2">
          <Label for="lease_end_date">租赁结束日期</Label>
          <Input id="lease_end_date" type="date" bind:value={form.lease_end_date} />
        </div>
        <div class="space-y-2">
          <Label for="price">价格</Label>
          <Input id="price" type="number" step="0.01" bind:value={form.price} />
        </div>
        <div class="space-y-2">
          <Label for="price_currency">币种</Label>
          <Input id="price_currency" bind:value={form.price_currency} placeholder="CNY" />
        </div>
        <div class="space-y-2">
          <Label for="warranty_info">保修信息</Label>
          <Input id="warranty_info" bind:value={form.warranty_info} />
        </div>
      </Card.Content>
    </Card.Root>

    <!-- 备注 -->
    <Card.Root>
      <Card.Header>
        <Card.Title>其他</Card.Title>
      </Card.Header>
      <Card.Content>
        <div class="space-y-2">
          <Label for="remarks">备注</Label>
          <Input id="remarks" bind:value={form.remarks} />
        </div>
      </Card.Content>
    </Card.Root>
  {/if}
</div>
