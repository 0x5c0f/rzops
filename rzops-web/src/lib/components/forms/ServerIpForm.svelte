<script lang="ts">
  import type { CreateServerIpRequest } from '$lib/types/server_ip';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TableSelectModal from '$lib/components/shared/TableSelectModal.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { ipStatusOptions, ipTypeOptions, serverStatusOptions, serverTypeOptions, getOptionLabel } from '$lib/utils/enum-options';
  import { searchServerPaginated, getProviderOptions } from '$lib/utils/entity-options';
  import { validate } from '$lib/utils/validation';
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
  let formError = $state<string | null>(null);
  let displayServerOptions = $state<{ label: string; value: string }[]>([]);
  let providerOptions = $state<{ label: string; value: string }[]>([]);
  let serverIds = $state<string[]>([]);

  let form = $state<CreateServerIpRequest>(createInitial(initial));

  let serverName = $derived(
    serverIds.length > 0
      ? (displayServerOptions.find(o => o.value === serverIds[0])?.label || serverIds[0])
      : '-'
  );

  function createInitial(initial?: CreateServerIpRequest): CreateServerIpRequest {
    return {
      server_id: '',
      ip_address: '',
      ip_type: '',
      is_primary: false,
      status: 'enabled',
      ...JSON.parse(JSON.stringify(initial ?? {})),
    };
  }

  onMount(async () => {
    providerOptions = await getProviderOptions();
    // 初始化服务器选中项
    if (form.server_id) {
      serverIds = [form.server_id];
      try {
        const res = await searchServerPaginated('', 1, 100);
        const found = res.data.find(s => s.id === form.server_id);
        if (found) {
          displayServerOptions = [{ label: String(found.name), value: form.server_id }];
        }
      } catch { /* ignore */ }
    }
  });

  async function handleSave() {
    formError = validate([
      { value: form.ip_address, label: 'IP地址', required: true, format: 'ip' },
      { value: form.ip_type, label: 'IP类型', required: true },
      { value: form.nic_name, label: '网卡名称', maxLength: 100 },
    ]);
    if (formError) return;
    saving = true;
    try {
      form.server_id = serverIds[0] || '';
      await onSubmit(form);
    } catch (err) {
      console.error('Failed to save server IP:', err);
      formError = '保存失败，请重试';
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  {#if formError}
    <div class="rounded-md border border-destructive/50 bg-destructive/10 px-4 py-3 text-sm text-destructive">
      {formError}
    </div>
  {/if}
  <Card.Root>
    <Card.Header>
      <Card.Title>基本信息</Card.Title>
    </Card.Header>
    <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      {#if editing}
        <div class="space-y-2">
          <Label for="server_id">服务器</Label>
          <Input id="server_id" value={serverName} disabled />
        </div>
      {:else}
        <TableSelectModal
          label="服务器"
          multiple={false}
          bind:value={serverIds}
          searchFn={searchServerPaginated}
          displayOptions={displayServerOptions}
          placeholder="选择服务器（可留空，如未绑定的EIP）"
          searchPlaceholder="输入名称或 IP 搜索..."
          modalTitle="选择服务器"
          columns={[
            { key: 'name', label: '服务器名称' },
            { key: 'primary_ip', label: '主IP', width: 'w-32' },
            {
              key: 'status',
              label: '状态',
              width: 'w-20',
              render: (item) => getOptionLabel($serverStatusOptions, String(item.status ?? '')),
            },
            {
              key: 'server_type',
              label: '类型',
              width: 'w-24',
              render: (item) => getOptionLabel($serverTypeOptions, String(item.server_type ?? '')),
            },
          ]}
        />
      {/if}

      <div class="space-y-2">
        <Label for="ip_address">IP地址 <span class="text-destructive">*</span></Label>
        <Input id="ip_address" bind:value={form.ip_address} required placeholder="如 192.168.1.10" />
      </div>

      <div class="space-y-2">
        <Label for="nic_name">网卡名称</Label>
        <Input id="nic_name" bind:value={form.nic_name} placeholder="如 eth0 / ens33 / 内网网卡" />
      </div>

      <FormSelect
        label="IP类型"
        bind:value={form.ip_type}
        options={$ipTypeOptions}
        placeholder="选择类型"
      />

      <FormSelect
        label="ISP供应商"
        bind:value={form.isp_provider_id}
        options={providerOptions}
        placeholder="选择供应商"
      />

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={$ipStatusOptions}
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
