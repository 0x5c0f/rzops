<script lang="ts">
  import type { CreateServerPortRequest } from '$lib/types/server_port';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TableSelectModal from '$lib/components/shared/TableSelectModal.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { searchServerPaginated } from '$lib/utils/entity-options';
  import { protocolOptions, serverStatusOptions, serverTypeOptions, getOptionLabel } from '$lib/utils/enum-options';
  import { validate } from '$lib/utils/validation';

  let {
    initial = {} as CreateServerPortRequest,
    editing = false,
    submitLabel = '保存',
    /** 编辑时传入当前已关联服务器（用于多选回显） */
    initialServers = [] as { id: string; name: string }[],
    onSubmit,
  }: {
    initial?: CreateServerPortRequest;
    editing?: boolean;
    submitLabel?: string;
    initialServers?: { id: string; name: string }[];
    onSubmit: (data: CreateServerPortRequest) => Promise<void>;
  } = $props();

  let saving = $state(false);
  let formError = $state<string | null>(null);

  let form = $state<CreateServerPortRequest>(createInitial(initial));

  function createInitial(initial?: CreateServerPortRequest): CreateServerPortRequest {
    // 注意：不能用 structuredClone(initial) —— Svelte 5 的 $state 会对含数组字段（如 server_ids）做 deep proxy，
    // structuredClone 无法克隆 proxy 数组，会抛 DataCloneError。用 JSON 深拷贝解包 proxy。
    return {
      server_ids: [],
      protocol: '',
      port: 0,
      service_name: '',
      access_scope: '',
      is_enabled: true,
      ...JSON.parse(JSON.stringify(initial ?? {})),
    };
  }

  let displayServerOptions = $derived(
    initialServers.map(s => ({ label: s.name, value: s.id }))
  );

  async function handleSave() {
    formError = validate([
      { value: form.server_ids, label: '服务器', required: true, custom: (v) => Array.isArray(v) && v.length > 0 ? null : '请至少选择一个服务器' },
      { value: form.protocol, label: '协议', required: true },
      { value: form.port, label: '端口号', required: true, format: 'port' },
      { value: form.service_name, label: '服务名称', required: true, maxLength: 100 },
    ]);
    if (formError) return;
    saving = true;
    try {
      await onSubmit(form);
    } catch (err) {
      console.error('Failed to save server port:', err);
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
      <div class="md:col-span-2 lg:col-span-3">
        <TableSelectModal
          label="服务器 *"
          multiple
          bind:value={form.server_ids}
          searchFn={searchServerPaginated}
          displayOptions={displayServerOptions}
          placeholder="选择服务器（可多选）"
          searchPlaceholder="输入名称或 IP 搜索..."
          modalTitle="选择服务器"
          required
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
      </div>

      <FormSelect
        label="协议 *"
        bind:value={form.protocol}
        options={$protocolOptions}
        placeholder="选择协议"
        required
      />

      <div class="space-y-2">
        <Label for="port">端口 <span class="text-destructive">*</span></Label>
        <Input id="port" type="number" bind:value={form.port} placeholder="如 80 / 443 / 3306" min={1} max={65535} required />
      </div>

      <div class="space-y-2">
        <Label for="service_name">服务名称 <span class="text-destructive">*</span></Label>
        <Input id="service_name" bind:value={form.service_name} placeholder="如 nginx / mysql / ssh" required />
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
