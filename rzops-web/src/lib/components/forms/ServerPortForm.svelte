<script lang="ts">
  import type { CreateServerPortRequest } from '$lib/types/server_port';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { getServerOptions } from '$lib/utils/entity-options';
  import { onMount } from 'svelte';

  let {
    initial = {} as CreateServerPortRequest,
    editing = false,
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateServerPortRequest;
    editing?: boolean;
    submitLabel?: string;
    onSubmit: (data: CreateServerPortRequest) => Promise<void>;
  } = $props();

  let saving = $state(false);
  let serverOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateServerPortRequest>(createInitial(initial));

  function createInitial(initial?: CreateServerPortRequest): CreateServerPortRequest {
    return {
      server_id: '',
      protocol: '',
      port: 0,
      service_name: '',
      access_scope: '',
      is_enabled: true,
      ...structuredClone(initial ?? {}),
    };
  }

  onMount(async () => {
    serverOptions = await getServerOptions();
  });

  async function handleSave() {
    saving = true;
    try {
      await onSubmit(form);
    } catch (err) {
      console.error('Failed to save server port:', err);
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
        <Label for="protocol">协议 *</Label>
        <Input id="protocol" bind:value={form.protocol} placeholder="TCP / UDP / ..." required />
      </div>

      <div class="space-y-2">
        <Label for="port">端口 *</Label>
        <Input id="port" type="number" bind:value={form.port} required />
      </div>

      <div class="space-y-2">
        <Label for="service_name">服务名称 *</Label>
        <Input id="service_name" bind:value={form.service_name} placeholder="如 nginx / mysql" required />
      </div>

      <div class="space-y-2">
        <Label for="access_scope">访问范围</Label>
        <Input id="access_scope" bind:value={form.access_scope} placeholder="公网 / 内网 / ..." />
      </div>

      <div class="flex items-center gap-2 pt-6">
        <input id="is_enabled" type="checkbox" bind:checked={form.is_enabled} class="h-4 w-4 rounded border-gray-300" />
        <Label for="is_enabled">启用</Label>
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
