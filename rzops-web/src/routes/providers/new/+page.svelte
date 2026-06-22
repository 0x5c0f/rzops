<script lang="ts">
  import { goto } from '$app/navigation';
  import { providersApi } from '$lib/api/providers';
  import type { CreateProviderRequest } from '$lib/types/provider';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { providerTypeOptions } from '$lib/utils/enum-options';

  let saving = $state(false);
  let form = $state<CreateProviderRequest>({
    name: '',
    provider_type: '',
    contact_name: '',
    contact_phone: '',
    contact_email: '',
    website: '',
    remarks: '',
    status: 'active',
  });

  async function handleSave() {
    saving = true;
    try {
      await providersApi.create(form);
      goto('/providers');
    } catch (err) {
      console.error('Failed to create provider:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '供应商', href: '/providers' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建供应商</h1>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => goto('/providers')}>取消</Button>
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
</div>
