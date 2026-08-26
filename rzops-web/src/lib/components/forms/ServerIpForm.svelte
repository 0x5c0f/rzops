<script lang="ts">
  import type { CreateServerIpRequest } from '$lib/types/server_ip';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { commonStatusOptions } from '$lib/utils/enum-options';
  import { getServerOptions, getProviderOptions } from '$lib/utils/entity-options';
  import { onMount } from 'svelte';

  let {
    initial = {} as CreateServerIpRequest,
    editing = false,
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateServerIpRequest;
    editing?: boolean;
    submitLabel?: string;
    onSubmit: (data: CreateServerIpRequest) => Promise<void>;
  } = $props();

  let saving = $state(false);
  let serverOptions = $state<{ label: string; value: string }[]>([]);
  let providerOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateServerIpRequest>(createInitial(initial));

  function createInitial(initial?: CreateServerIpRequest): CreateServerIpRequest {
    return {
      server_id: '',
      ip_address: '',
      ip_type: '',
      is_primary: false,
      status: 'active',
      ...structuredClone(initial ?? {}),
    };
  }

  onMount(async () => {
    const [servers, providers] = await Promise.all([
      getServerOptions(),
      getProviderOptions(),
    ]);
    serverOptions = servers;
    providerOptions = providers;
  });

  async function handleSave() {
    saving = true;
    try {
      await onSubmit(form);
    } catch (err) {
      console.error('Failed to save server IP:', err);
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
      {#if editing}
        <div class="space-y-2">
          <Label for="server_id">服务器</Label>
          <Input id="server_id" value={form.server_id} disabled />
        </div>
      {:else}
        <FormSelect
          label="服务器 *"
          bind:value={form.server_id}
          options={serverOptions}
          placeholder="选择服务器"
          required
        />
      {/if}

      <div class="space-y-2">
        <Label for="ip_address">IP地址 *</Label>
        <Input id="ip_address" bind:value={form.ip_address} required placeholder="如 192.168.1.10" />
      </div>

      <div class="space-y-2">
        <Label for="ip_type">IP类型</Label>
        <Input id="ip_type" bind:value={form.ip_type} placeholder="公网 / 内网 / ..." />
      </div>

      <FormSelect
        label="ISP供应商"
        bind:value={form.isp_provider_id}
        options={providerOptions}
        placeholder="选择供应商"
      />

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={$commonStatusOptions}
      />

      <div class="flex items-center gap-2 pt-6">
        <input id="is_primary" type="checkbox" bind:checked={form.is_primary} class="h-4 w-4 rounded border-gray-300" />
        <Label for="is_primary">主 IP</Label>
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
