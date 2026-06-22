<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { datacentersApi } from '$lib/api/datacenters';
  import type { DataCenterResponse, UpdateDataCenterRequest } from '$lib/types/datacenter';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { commonStatusOptions } from '$lib/utils/enum-options';
  import { getProviderOptions, ensureOption } from '$lib/utils/entity-options';
  import { onMount } from 'svelte';

  let datacenter = $state<DataCenterResponse | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let form = $state<UpdateDataCenterRequest>({});
  let providerOptions = $state<{ label: string; value: string }[]>([]);

  onMount(async () => {
    providerOptions = await getProviderOptions();
    try {
      datacenter = await datacentersApi.getById($page.params.id ?? "");
      form = {
        name: datacenter.name,
        code: datacenter.code ?? undefined,
        location: datacenter.location ?? undefined,
        address: datacenter.address ?? undefined,
        provider_id: datacenter.provider_id ?? undefined,
        tier_level: datacenter.tier_level ?? undefined,
        total_racks: datacenter.total_racks ?? undefined,
        used_racks: datacenter.used_racks ?? undefined,
        power_capacity_kw: datacenter.power_capacity_kw ?? undefined,
        contact_name: datacenter.contact_name ?? undefined,
        contact_phone: datacenter.contact_phone ?? undefined,
        phone: datacenter.phone ?? undefined,
        country: datacenter.country ?? undefined,
        province: datacenter.province ?? undefined,
        city: datacenter.city ?? undefined,
        line_type: datacenter.line_type ?? undefined,
        description: datacenter.description ?? undefined,
        status: datacenter.status,
        remarks: datacenter.remarks ?? undefined,
      };
      providerOptions = ensureOption(providerOptions, datacenter.provider_id ?? undefined, datacenter.provider_id ?? undefined);
    } catch (err) {
      console.error('Failed to load datacenter:', err);
      goto('/datacenters');
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    if (!datacenter) return;
    saving = true;
    try {
      await datacentersApi.update(datacenter.id, form);
      goto('/datacenters');
    } catch (err) {
      console.error('Failed to save datacenter:', err);
    } finally {
      saving = false;
    }
  }

  async function handleDelete() {
    if (!datacenter) return;
    if (!confirm(`确定要删除数据中心 "${datacenter.name}" 吗？`)) return;
    try {
      await datacentersApi.delete(datacenter.id);
      goto('/datacenters');
    } catch (err) {
      console.error('Failed to delete datacenter:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '数据中心', href: '/datacenters' },
    { label: datacenter?.name || '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if datacenter}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{datacenter.name}</h1>
        <StatusBadge status={datacenter.status} />
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
        <div class="space-y-2">
          <Label for="code">编码</Label>
          <Input id="code" bind:value={form.code} />
        </div>
        <div class="space-y-2">
          <Label for="location">位置</Label>
          <Input id="location" bind:value={form.location} />
        </div>
        <div class="space-y-2">
          <Label for="address">地址</Label>
          <Input id="address" bind:value={form.address} />
        </div>
        <FormSelect
          label="供应商"
          bind:value={form.provider_id}
          options={providerOptions}
          placeholder="选择供应商"
        />
        <div class="space-y-2">
          <Label for="tier_level">等级</Label>
          <Input id="tier_level" bind:value={form.tier_level} placeholder="T1 / T2 / T3 / T4" />
        </div>
        <div class="space-y-2">
          <Label for="total_racks">总机架数</Label>
          <Input id="total_racks" type="number" bind:value={form.total_racks} />
        </div>
        <div class="space-y-2">
          <Label for="used_racks">已用机架数</Label>
          <Input id="used_racks" type="number" bind:value={form.used_racks} />
        </div>
        <div class="space-y-2">
          <Label for="power_capacity_kw">电力容量(KW)</Label>
          <Input id="power_capacity_kw" type="number" bind:value={form.power_capacity_kw} />
        </div>
        <div class="space-y-2">
          <Label for="contact_name">联系人</Label>
          <Input id="contact_name" bind:value={form.contact_name} />
        </div>
        <div class="space-y-2">
          <Label for="contact_phone">联系电话</Label>
          <Input id="contact_phone" bind:value={form.contact_phone} />
        </div>
        <div class="space-y-2">
          <Label for="phone">电话</Label>
          <Input id="phone" bind:value={form.phone} />
        </div>
        <div class="space-y-2">
          <Label for="country">国家</Label>
          <Input id="country" bind:value={form.country} />
        </div>
        <div class="space-y-2">
          <Label for="province">省份</Label>
          <Input id="province" bind:value={form.province} />
        </div>
        <div class="space-y-2">
          <Label for="city">城市</Label>
          <Input id="city" bind:value={form.city} />
        </div>
        <div class="space-y-2">
          <Label for="line_type">线路类型</Label>
          <Input id="line_type" bind:value={form.line_type} placeholder="电信 / 联通 / BGP" />
        </div>
        <div class="space-y-2 md:col-span-2 lg:col-span-3">
          <Label for="description">描述</Label>
          <Input id="description" bind:value={form.description} />
        </div>
        <FormSelect
          label="状态"
          bind:value={form.status}
          options={commonStatusOptions}
        />
        <div class="space-y-2 md:col-span-2 lg:col-span-3">
          <Label for="remarks">备注</Label>
          <Input id="remarks" bind:value={form.remarks} />
        </div>
      </Card.Content>
    </Card.Root>
  {/if}
</div>
