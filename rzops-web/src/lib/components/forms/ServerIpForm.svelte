<script lang="ts">
  import type { CreateServerIpRequest } from '$lib/types/server_ip';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import RemoteSearchSelect from '$lib/components/shared/RemoteSearchSelect.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { ipStatusOptions, ipTypeOptions, serverStatusOptions, serverTypeOptions, getOptionLabel } from '$lib/utils/enum-options';
  import { searchServerOptions, getProviderOptions } from '$lib/utils/entity-options';
  import { validate } from '$lib/utils/validation';
  import { api } from '$lib/api/client';
  import ConfirmDialog from '$lib/components/shared/ConfirmDialog.svelte';
  import { onMount } from 'svelte';

  let {
    initial = {} as CreateServerIpRequest,
    initialServerName = null,
    editing = false,
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateServerIpRequest;
    /** 编辑时已绑定服务器的名称，用于可靠回显（避免依赖搜索命中） */
    initialServerName?: string | null;
    editing?: boolean;
    submitLabel?: string;
    onSubmit: (data: CreateServerIpRequest) => Promise<void>;
  } = $props();

  // svelte-ignore state_referenced_locally —— 仅初始化用一次，有意读取初始值
  const initialSnapshot = $state.snapshot(initial);
  // svelte-ignore state_referenced_locally —— 仅初始化用一次，有意读取初始值
  const initialServerNameSnapshot = $state.snapshot(initialServerName);

  let saving = $state(false);
  let formError = $state<string | null>(null);
  let providerOptions = $state<{ label: string; value: string }[]>([]);
  let confirmOpen = $state(false);
  let duplicateDescription = $state('');

  let form = $state<CreateServerIpRequest>(createInitial(initialSnapshot));
  // 同步初始化服务器回显选项：编辑时直接使用传入的 server_name，
  // 避免异步搜索期间 RemoteSearchSelect 回退显示原始 id（先闪 id 再变名字）
  let serverDisplayOptions = $state<{ label: string; value: string }[]>(
    form.server_id && initialServerNameSnapshot ? [{ label: initialServerNameSnapshot, value: form.server_id }] : []
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
    // 编辑回显已由 serverDisplayOptions 同步初始化；
    // 仅当已绑定服务器但缺少名称信息时（异常数据）才 fallback 搜索
    if (form.server_id && serverDisplayOptions.length === 0) {
      serverDisplayOptions = await searchServerOptions('');
      const found = serverDisplayOptions.find(o => o.value === form.server_id);
      if (!found) {
        serverDisplayOptions = [...serverDisplayOptions, { label: form.server_id, value: form.server_id }];
      }
    }
  });

  async function handleSave() {
    formError = validate([
      { value: form.ip_address, label: 'IP地址', required: true, format: 'ip' },
      { value: form.ip_type, label: 'IP类型', required: true },
      { value: form.nic_name, label: '网卡名称', maxLength: 100 },
    ]);
    if (formError) return;

    // 新建时检查（软删记录已被接口天然排除）：
    // 1) 同 IP 且未绑定服务器 → 可能重复登记空闲 IP
    // 2) 同 IP 且绑定与本次相同的服务器 → 同一服务器重复绑定同一 IP
    // 命中则提示确认（不拦截）；编辑自身记录跳过检查；绑定其他服务器的同 IP 不提醒（异地机房同网段合法）
    if (!editing && form.ip_address) {
      try {
        // _t 时间戳参数：防止浏览器对同 URL GET 的内存缓存导致检查结果过期
        const res = await api.get<{ data: { ip_address: string; server_id: string | null }[] }>('/api/v1/server-ips', {
          q: form.ip_address,
          per_page: 100,
          _t: Date.now(),
        });
        const hits = res.data.filter(
          item =>
            item.ip_address === form.ip_address &&
            (!item.server_id || (form.server_id && item.server_id === form.server_id)),
        );
        console.info(
          `[dup-check] ip=${form.ip_address} server_id=${form.server_id ?? '(empty)'} hits=${hits.length}`,
        );
        if (hits.length > 0) {
          const unboundCount = hits.filter(i => !i.server_id).length;
          const sameServerCount = hits.length - unboundCount;
          const parts: string[] = [];
          if (unboundCount > 0) parts.push(`${unboundCount} 条未绑定服务器的记录`);
          if (sameServerCount > 0) parts.push(`${sameServerCount} 条绑定当前服务器的记录`);
          duplicateDescription = `该 IP 已存在${parts.join('、')}，确认继续添加吗？`;
          confirmOpen = true;
          return;
        }
      } catch (err) {
        // 检查失败不阻塞保存，静默跳过
        console.warn('Duplicate IP check failed:', err);
      }
    }

    await doSave();
  }

  async function doSave() {
    saving = true;
    try {
      await onSubmit(form);
    } catch (err) {
      console.error('Failed to save server IP:', err);
      formError = err instanceof Error && err.message ? err.message : '保存失败，请重试';
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

  <div class="flex items-center justify-between">
    <div></div>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => history.back()}>取消</Button>
      <Button onclick={handleSave} disabled={saving}>
        {saving ? '保存中...' : submitLabel}
      </Button>
    </div>
  </div>

<Card.Root>
    <Card.Header>
      <Card.Title>基本信息</Card.Title>
    </Card.Header>
    <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <RemoteSearchSelect
        label="服务器"
        bind:value={form.server_id}
        searchFn={searchServerOptions}
        displayOptions={serverDisplayOptions}
        placeholder={editing ? '' : '选择服务器（可留空，如未绑定的EIP）'}
        searchPlaceholder="输入名称或 IP 搜索..."
        disabled={editing}
      />

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
        required
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

<ConfirmDialog
  bind:open={confirmOpen}
  title="IP 重复提醒"
  description={duplicateDescription}
  confirmLabel="确认添加"
  onConfirm={doSave}
/>
