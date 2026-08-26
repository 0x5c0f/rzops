<script lang="ts">
  import type { CreateDataCenterRequest } from '$lib/types/datacenter';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { commonStatusOptions } from '$lib/utils/enum-options';
  import { onMount } from 'svelte';

  let {
    initial = {} as CreateDataCenterRequest,
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateDataCenterRequest;
    submitLabel?: string;
    onSubmit: (data: CreateDataCenterRequest) => Promise<void>;
  } = $props();

  let saving = $state(false);
  let providerOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateDataCenterRequest>(createInitial(initial));

  function createInitial(initial?: CreateDataCenterRequest): CreateDataCenterRequest {
    return {
      name: '',
      status: 'active',
      ...structuredClone(initial ?? {}),
    };
  }

  onMount(async () => {
    providerOptions = await getProviderOptions();
  });

  async function handleSave() {
    saving = true;
    try {
      await onSubmit(form);
    } catch (err) {
      console.error('Failed to save datacenter:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
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
        label="供应商"
        bind:value={form.provider_id}
        options={providerOptions}
        placeholder="选择供应商"
      />

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={$commonStatusOptions}
      />

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

      <div class="space-y-2">
        <Label for="address">地址</Label>
        <Input id="address" bind:value={form.address} />
      </div>

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="description">描述</Label>
        <TextArea id="description" bind:value={form.description} rows={3} />
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
