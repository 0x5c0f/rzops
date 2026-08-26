<script lang="ts">
  import type { CreateServerRequest } from '$lib/types/server';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import FormMultiSelect from '$lib/components/shared/FormMultiSelect.svelte';
  import DateField from '$lib/components/shared/DateField.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
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

  let {
    initial = {} as CreateServerRequest,
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateServerRequest;
    submitLabel?: string;
    onSubmit: (data: CreateServerRequest) => Promise<void>;
  } = $props();

  let saving = $state(false);
  let providerOptions = $state<{ label: string; value: string }[]>([]);
  let dataCenterOptions = $state<{ label: string; value: string }[]>([]);

  // 默认值为空字符串，确保编辑时清空字段能正确提交（后端部分更新语义）
  let form = $state<CreateServerRequest>(createInitial(initial));

  function createInitial(initial?: CreateServerRequest): CreateServerRequest {
    return {
      name: '',
      asset_code: '',
      primary_ip: '',
      server_type: '',
      status: 'active',
      is_dual_line: false,
      is_database_server: false,
      is_raid: false,
      price_currency: 'CNY',
      ...structuredClone(initial ?? {}),
    };
  }

  onMount(async () => {
    const [providers, dataCenters] = await Promise.all([
      getProviderOptions(),
      getDataCenterOptions(),
    ]);
    providerOptions = providers;
    dataCenterOptions = dataCenters;
  });

  async function handleSave() {
    if (
      form.lease_start_date &&
      form.lease_end_date &&
      form.lease_end_date < form.lease_start_date
    ) {
      alert('租赁结束日期不能早于开始日期');
      return;
    }
    saving = true;
    try {
      await onSubmit(form);
    } catch (err) {
      console.error('Failed to save server:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <div></div>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => history.back()}>取消</Button>
      <Button onclick={handleSave} disabled={saving}>
        {saving ? '保存中...' : submitLabel}
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
        <TextArea id="disk_layout" bind:value={form.disk_layout} rows={2} />
      </div>

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="hardware_config">硬件配置</Label>
        <TextArea id="hardware_config" bind:value={form.hardware_config} rows={3} />
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
      <DateField
        id="lease_start_date"
        label="租赁开始日期"
        bind:value={form.lease_start_date}
        max={form.lease_end_date || undefined}
      />

      <DateField
        id="lease_end_date"
        label="租赁结束日期"
        bind:value={form.lease_end_date}
        min={form.lease_start_date || undefined}
      />

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
        <TextArea id="remarks" bind:value={form.remarks} rows={3} />
      </div>
    </Card.Content>
  </Card.Root>

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
