<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { providersApi } from '$lib/api/providers';
  import type { ProviderResponse, UpdateProviderRequest } from '$lib/types/provider';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { providerTypeOptions } from '$lib/utils/enum-options';
  import { onMount } from 'svelte';

  let provider = $state<ProviderResponse | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let form = $state<UpdateProviderRequest>({});

  onMount(async () => {
    try {
      provider = await providersApi.getById($page.params.id ?? "");
      form = {
        name: provider.name,
        provider_type: provider.provider_type ?? undefined,
        contact_name: provider.contact_name ?? undefined,
        contact_phone: provider.contact_phone ?? undefined,
        contact_email: provider.contact_email ?? undefined,
        contact_qq: provider.contact_qq ?? undefined,
        fax: provider.fax ?? undefined,
        address: provider.address ?? undefined,
        country: provider.country ?? undefined,
        description: provider.description ?? undefined,
        website: provider.website ?? undefined,
        remarks: provider.remarks ?? undefined,
        status: provider.status,
      };
    } catch (err) {
      console.error('Failed to load provider:', err);
      goto('/providers');
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    if (!provider) return;
    saving = true;
    try {
      await providersApi.update(provider.id, form);
      goto('/providers');
    } catch (err) {
      console.error('Failed to save provider:', err);
    } finally {
      saving = false;
    }
  }

  async function handleDelete() {
    if (!provider) return;
    if (!confirm(`确定要删除供应商 "${provider.name}" 吗？`)) return;
    try {
      await providersApi.delete(provider.id);
      goto('/providers');
    } catch (err) {
      console.error('Failed to delete provider:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '供应商', href: '/providers' },
    { label: provider?.name || '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if provider}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{provider.name}</h1>
        <StatusBadge status={provider.status} />
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
        <FormSelect
          label="供应商类型"
          bind:value={form.provider_type}
          options={providerTypeOptions}
          placeholder="选择供应商类型"
        />
        <div class="space-y-2">
          <Label for="contact">联系人</Label>
          <Input id="contact" bind:value={form.contact_name} />
        </div>
        <div class="space-y-2">
          <Label for="phone">电话</Label>
          <Input id="phone" bind:value={form.contact_phone} />
        </div>
        <div class="space-y-2">
          <Label for="email">邮箱</Label>
          <Input id="email" type="email" bind:value={form.contact_email} />
        </div>
        <div class="space-y-2">
          <Label for="website">网站</Label>
          <Input id="website" bind:value={form.website} />
        </div>
        <div class="space-y-2">
          <Label for="contact_qq">QQ</Label>
          <Input id="contact_qq" bind:value={form.contact_qq} />
        </div>
        <div class="space-y-2">
          <Label for="fax">传真</Label>
          <Input id="fax" bind:value={form.fax} />
        </div>
        <div class="space-y-2">
          <Label for="address">地址</Label>
          <Input id="address" bind:value={form.address} />
        </div>
        <div class="space-y-2">
          <Label for="country">国家</Label>
          <Input id="country" bind:value={form.country} />
        </div>
        <div class="space-y-2 md:col-span-2 lg:col-span-3">
          <Label for="description">描述</Label>
          <Input id="description" bind:value={form.description} />
        </div>
        <div class="space-y-2 md:col-span-2 lg:col-span-3">
          <Label for="remarks">备注</Label>
          <Input id="remarks" bind:value={form.remarks} />
        </div>
      </Card.Content>
    </Card.Root>
  {/if}
</div>
