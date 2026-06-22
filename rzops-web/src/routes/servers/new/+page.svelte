<script lang="ts">
  import { goto } from '$app/navigation';
  import { serversApi } from '$lib/api/servers';
  import type { CreateServerRequest } from '$lib/types/server';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
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
  import { getProviderOptions, getDataCenterOptions } from '$lib/utils/entity-options';
  import { onMount } from 'svelte';

  let saving = $state(false);
  let providerOptions = $state<{ label: string; value: string }[]>([]);
  let dataCenterOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateServerRequest>({
    name: '',
    primary_ip: '',
    server_type: '',
    status: 'active',
    is_dual_line: false,
    is_database_server: false,
    is_raid: false,
    price_currency: 'CNY',
  });

  onMount(async () => {
    const [providers, dataCenters] = await Promise.all([
      getProviderOptions(),
      getDataCenterOptions(),
    ]);
    providerOptions = providers;
    dataCenterOptions = dataCenters;
  });

  async function handleSave() {
    saving = true;
    try {
      await serversApi.create(form);
      goto('/servers');
    } catch (err) {
      console.error('Failed to create server:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '服务器', href: '/servers' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建服务器</h1>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => goto('/servers')}>取消</Button>
      <Button onclick={handleSave} disabled={saving}>
        {saving ? '创建中...' : '创建'}
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
        <Label for="name">名称 *</Label>
        <Input id="name" bind:value={form.name} required />
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
</div>
