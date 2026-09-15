<script lang="ts">
  import type { CreateServerRequest } from '$lib/types/server';
  import type { ServerIpResponse } from '$lib/types/server_ip';
  import type { ServerPortResponse } from '$lib/types/server_port';
  import { goto } from '$app/navigation';
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
    ipTypeOptions,
    protocolOptions,
    currencyOptions,
    databaseTypeOptions,
    importanceOptions,
    environmentOptions,
  } from '$lib/utils/enum-options';
  import { serverIpsApi } from '$lib/api/server-ips';
  import { serverPortsApi } from '$lib/api/server-ports';
  import { serverPortTemplatesApi } from '$lib/api/server-port-templates';
  import TableSelectModal from '$lib/components/shared/TableSelectModal.svelte';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import { siteRelationsApi } from '$lib/api/site-relations';
  import AttachmentFormSection from '$lib/components/shared/AttachmentFormSection.svelte';
  import { getProviderOptions, getDataCenterOptions, getOpsSiteOptions, searchServerPortTemplatePaginated } from '$lib/utils/entity-options';
  import { siteServerRoleOptions } from '$lib/utils/enum-options';
  import { validate, validateDateRange } from '$lib/utils/validation';
  import { onMount } from 'svelte';

  // IP / 端口 明细草稿行（id 存在 = 已有记录，用于编辑增量同步）
  interface IpDraft {
    id?: string;
    ip_address: string;
    nic_name: string;
    ip_type: string;
    is_primary: boolean;
    isp_provider_id: string;
    description: string;
  }
  interface PortDraft {
    id?: string;
    protocol: string;
    port: string;
    service_name: string;
    access_scope: string;
    is_enabled: boolean;
    description: string;
  }
  interface DbDraft {
    id?: string;
    name: string;
    db_type: string;
    port: string;
    instance_name: string;
    importance: string;
    description: string;
  }
  // 关联站点草稿行（id = 关联关系 id，编辑时用于增量删除）
  interface SiteDraft {
    id?: string;
    site_id: string;
    deploy_role: string;
  }

  let {
    initial = {} as CreateServerRequest,
    initialIps = [] as IpDraft[],
    initialPorts = [] as PortDraft[],
    initialDbInstances = [] as DbDraft[],
    entityId = '',
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateServerRequest;
    initialIps?: IpDraft[];
    initialPorts?: PortDraft[];
    initialDbInstances?: DbDraft[];
    entityId?: string;
    submitLabel?: string;
    onSubmit: (data: CreateServerRequest) => Promise<string | void>;
  } = $props();

  // svelte-ignore state_referenced_locally —— 仅初始化用一次，有意读取初始值
  const initialSnapshot = $state.snapshot(initial);
  // svelte-ignore state_referenced_locally —— 仅初始化用一次，有意读取初始值
  const initialIpsSnapshot = $state.snapshot(initialIps);
  // svelte-ignore state_referenced_locally —— 仅初始化用一次，有意读取初始值
  const initialPortsSnapshot = $state.snapshot(initialPorts);
  // svelte-ignore state_referenced_locally —— 仅初始化用一次，有意读取初始值
  const initialDbInstancesSnapshot = $state.snapshot(initialDbInstances);

  let saving = $state(false);
  let formError = $state<string | null>(null);
  let providerOptions = $state<{ label: string; value: string }[]>([]);
  let dataCenterOptions = $state<{ label: string; value: string }[]>([]);
  let siteOptions = $state<{ label: string; value: string }[]>([]);
  let attachmentRef = $state<{ uploadAll: (id: string) => Promise<void> } | null>(null);

  // 默认值为空字符串，确保编辑时清空字段能正确提交（后端部分更新语义）
  let form = $state<CreateServerRequest>(createInitial(initialSnapshot));
  let ips = $state<IpDraft[]>(initialIpsSnapshot.length ? JSON.parse(JSON.stringify(initialIpsSnapshot)) : []);
  let ports = $state<PortDraft[]>(initialPortsSnapshot.length ? JSON.parse(JSON.stringify(initialPortsSnapshot)) : []);
  let dbInstances = $state<DbDraft[]>(
    initialDbInstancesSnapshot.length ? JSON.parse(JSON.stringify(initialDbInstancesSnapshot)) : [],
  );
  let siteRels = $state<SiteDraft[]>([]);
  let initialSiteRels = $state<SiteDraft[]>([]);

  function createInitial(initial?: CreateServerRequest): CreateServerRequest {
    return {
      name: '',
      asset_code: '',
      primary_ip: '',
      location: '',
      isp_provider_id: '',
      data_center_id: '',
      hosting_type: '',
      is_dual_line: false,
      is_database_server: false,
      is_raid: false,
      price_currency: 'CNY',
      status: 'active',
      environment: '',
      ...JSON.parse(JSON.stringify(initial ?? {})),
    };
  }

  onMount(async () => {
    const [providers, dataCenters, sites] = await Promise.all([
      getProviderOptions(),
      getDataCenterOptions(),
      getOpsSiteOptions(),
    ]);
    providerOptions = providers;
    dataCenterOptions = dataCenters;
    siteOptions = sites;
    // 编辑时加载已有关联站点
    if (entityId) {
      try {
        const rels = await siteRelationsApi.listSitesByServer(entityId);
        siteRels = rels.map(r => ({
          id: r.relation_id,
          site_id: r.site_id,
          deploy_role: r.deploy_role || '',
        }));
        initialSiteRels = JSON.parse(JSON.stringify(siteRels));
      } catch (err) {
        console.error('Failed to load site relations:', err);
      }
    }
  });

  function emptyIp(): IpDraft {
    return { ip_address: '', nic_name: '', ip_type: '', is_primary: false, isp_provider_id: '', description: '' };
  }
  function emptyPort(): PortDraft {
    return { protocol: 'tcp', port: '', service_name: '', access_scope: '', is_enabled: true, description: '' };
  }

  // 端口模板快速添加：单选模板，确认后按模板字段追加一行端口草稿
  // 端口模板快速添加：多选模板，确认后按模板字段逐个追加端口草稿
  let templatePickerValue = $state<string[]>([]);
  function handleTemplateConfirm(items: Record<string, unknown>[]) {
    for (const t of items) {
      if (!t) continue;
      ports = [
        ...ports,
        {
          protocol: String(t.protocol ?? 'tcp'),
          port: String(t.port ?? ''),
          service_name: String(t.service_name ?? ''),
          access_scope: String(t.access_scope ?? ''),
          is_enabled: Boolean(t.is_enabled ?? true),
          description: String(t.description ?? ''),
        },
      ];
    }
    templatePickerValue = [];
  }
  function emptyDb(): DbDraft {
    return { name: '', db_type: '', port: '', instance_name: '', importance: '', description: '' };
  }
  function emptySiteRel(): SiteDraft {
    return { site_id: '', deploy_role: '' };
  }

  function addIpRow() {
    ips = [...ips, emptyIp()];
  }
  function removeIpRow(index: number) {
    ips = ips.filter((_, i) => i !== index);
  }
  function addPortRow() {
    ports = [...ports, emptyPort()];
  }
  function removePortRow(index: number) {
    ports = ports.filter((_, i) => i !== index);
  }
  function addDbRow() {
    dbInstances = [...dbInstances, emptyDb()];
  }
  function removeDbRow(index: number) {
    dbInstances = dbInstances.filter((_, i) => i !== index);
  }
  function addSiteRelRow() {
    siteRels = [...siteRels, emptySiteRel()];
  }
  function removeSiteRelRow(index: number) {
    siteRels = siteRels.filter((_, i) => i !== index);
  }

  // 主 IP 同步策略（保证编辑时主 IP 稳定、尊重用户显式勾选）：
  // 1) 有用户显式勾选的主 IP 行 → 跟随该行
  // 2) 无显式勾选且已有主 IP 仍在列表 → 保持该主 IP，并标记对应行
  // 3) 新建 / 原主 IP 已被移除 → 第一行设为主
  function syncPrimaryIp() {
    const manually = ips.find(i => i.is_primary);
    if (manually) {
      form.primary_ip = manually.ip_address.trim();
      return;
    }
    const existing = (form.primary_ip || '').trim();
    const listAddrs = ips.map(i => i.ip_address.trim());
    if (existing && listAddrs.includes(existing)) {
      for (const i of ips) i.is_primary = i.ip_address.trim() === existing;
      return;
    }
    if (ips.length > 0) {
      ips[0].is_primary = true;
      form.primary_ip = ips[0].ip_address.trim();
    } else {
      form.primary_ip = '';
    }
  }

  function validateRows(): string | null {
    for (const ip of ips) {
      if (!ip.ip_address.trim()) return 'IP 地址为必填，请填写完整或删除空行';
    }
    for (const p of ports) {
      const portNum = Number(p.port);
      if (!p.port || !Number.isInteger(portNum) || portNum < 1 || portNum > 65535) {
        return '端口号必须是 1-65535 的整数';
      }
      if (!p.protocol || !p.service_name.trim()) {
        return '端口记录中协议和服务名为必填';
      }
    }
    return null;
  }

  // 增量同步 IP：删除表单中已移除的、更新有 id 的、新增无 id 的
  async function syncIps(serverId: string) {
    for (const dbIp of initialIps) {
      if (dbIp.id && !ips.some(r => r.id === dbIp.id)) {
        await serverIpsApi.unbind(dbIp.id);
      }
    }
    for (const row of ips) {
      const payload = {
        ip_address: row.ip_address.trim(),
        nic_name: row.nic_name || undefined,
        ip_type: row.ip_type || undefined,
        is_primary: row.is_primary,
        isp_provider_id: row.isp_provider_id || undefined,
        description: row.description || undefined,
      };
      if (row.id) {
        await serverIpsApi.update(row.id, payload);
      } else {
        await serverIpsApi.create({ server_id: serverId, ...payload });
      }
    }
  }

  async function syncPorts(serverId: string) {
    for (const dbPort of initialPorts) {
      if (dbPort.id && !ports.some(r => r.id === dbPort.id)) {
        await serverPortsApi.delete(dbPort.id);
      }
    }
    for (const row of ports) {
      const payload = {
        protocol: row.protocol,
        port: Number(row.port),
        service_name: row.service_name.trim(),
        access_scope: row.access_scope || undefined,
        is_enabled: row.is_enabled,
        description: row.description || undefined,
      };
      if (row.id) {
        await serverPortsApi.update(row.id, payload);
      } else {
        // 端口为每台服务器独立的记录（一对多），新建时归属当前服务器即可，不影响其他服务器
        await serverPortsApi.create({ server_id: serverId, ...payload });
      }
    }
  }

  // 数据库实例增量同步（仅当勾选"数据库服务器"时维护）
  async function syncDbInstances(serverId: string) {
    for (const db of initialDbInstances) {
      if (db.id && !dbInstances.some(r => r.id === db.id)) {
        await databaseInstancesApi.unbind(db.id);
      }
    }
    for (const row of dbInstances) {
      const payload = {
        name: row.name.trim(),
        db_type: row.db_type,
        port: row.port ? Number(row.port) : undefined,
        instance_name: row.instance_name || undefined,
        importance: row.importance || undefined,
        description: row.description || undefined,
        status: 'active',
      };
      if (row.id) {
        await databaseInstancesApi.update(row.id, payload);
      } else {
        await databaseInstancesApi.create({ server_id: serverId, ...payload });
      }
    }
  }

  // 站点关联增量同步：删除已移除的、更新有 id 的、新增无 id 的
  async function syncSiteRels(serverId: string) {
    for (const rel of initialSiteRels) {
      if (rel.id && !siteRels.some(r => r.id === rel.id)) {
        await siteRelationsApi.deleteServer(rel.id);
      }
    }
    for (const rel of siteRels) {
      if (!rel.site_id) continue;
      if (rel.id) {
        await siteRelationsApi.updateServer(rel.id, {
          deploy_role: rel.deploy_role || undefined,
        });
      } else {
        await siteRelationsApi.createServer({
          site_id: rel.site_id,
          server_id: serverId,
          deploy_role: rel.deploy_role || undefined,
        });
      }
    }
  }

  async function handleSave() {
    // 基本字段校验
    formError = validate([
      { value: form.name, label: '服务器名称', required: true, maxLength: 100 },
      { value: form.primary_ip, label: '主IP', format: 'ip' },
      { value: form.asset_code, label: '资产编号', maxLength: 50 },
      { value: form.price, label: '租赁金额', format: 'positiveNumber' },
    ]);
    if (formError) return;
    // 日期范围校验
    formError = validateDateRange(form.lease_start_date, form.lease_end_date, '租赁开始日期', '租赁结束日期');
    if (formError) return;
    // 行校验
    formError = validateRows();
    if (formError) return;
    // 数据库实例行校验
    if (dbInstances.some(r => !r.name.trim() || !r.db_type)) {
      formError = '数据库实例的实例名和类型为必填，请填写完整或删除空行';
      return;
    }
    syncPrimaryIp();
    saving = true;
    try {
      const serverId = await onSubmit(form);
      if (serverId) {
        await syncIps(serverId);
        await syncPorts(serverId);
        await syncDbInstances(serverId);
        await syncSiteRels(serverId);
        await attachmentRef?.uploadAll(serverId);
        goto(`/servers/${serverId}`);
      }
    } catch (err) {
      console.error('Failed to save server:', err);
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
        <Label for="name">名称 <span class="text-destructive">*</span></Label>
        <Input id="name" bind:value={form.name} required />
      </div>

      <div class="space-y-2">
        <Label for="asset_code">资产编号</Label>
        <Input id="asset_code" bind:value={form.asset_code} />
      </div>

      <div class="space-y-2">
        <Label for="location">位置</Label>
        <Input id="location" bind:value={form.location} />
      </div>

      <FormSelect
        label="服务器类型"
        bind:value={form.server_type}
        options={$serverTypeOptions}
        placeholder="选择服务器类型"
      />

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={$serverStatusOptions}
      />

      <FormSelect
        label="环境"
        bind:value={form.environment}
        options={$environmentOptions}
        placeholder="选择环境"
      />

      <FormSelect
        label="架构"
        bind:value={form.architecture}
        options={$architectureOptions}
        placeholder="选择架构"
      />

      <div class="space-y-2">
        <Label for="brand">品牌</Label>
        <Input id="brand" bind:value={form.brand} />
      </div>

      <div class="space-y-2">
        <Label for="operating_system">操作系统</Label>
        <Input id="operating_system" bind:value={form.operating_system} />
      </div>

      <FormMultiSelect
        label="角色标签"
        bind:value={form.role_tags}
        options={$serverRoleOptions}
        placeholder="选择角色"
      />

      <FormMultiSelect
        label="Web服务器软件"
        bind:value={form.web_server_type}
        options={$webServerSoftwareOptions}
        placeholder="选择Web服务器"
      />

      <div class="flex items-center gap-2 pt-6">
        <input type="checkbox" id="is_database_server" bind:checked={form.is_database_server} class="h-4 w-4" />
        <Label for="is_database_server">数据库服务器</Label>
      </div>
    </Card.Content>
  </Card.Root>

  <!-- IP 地址 -->
  <Card.Root>
    <Card.Header>
      <Card.Title>IP 地址</Card.Title>
      <p class="text-sm text-muted-foreground">一台服务器可维护多个 IP，第一行或标记"主 IP"的行将作为服务器主 IP</p>
    </Card.Header>
    <Card.Content class="space-y-3">
      {#each ips as ip, i}
        <div class="grid gap-3 rounded-lg border p-3 md:grid-cols-12">
          <div class="space-y-1 md:col-span-3">
            <Label>IP 地址 <span class="text-destructive">*</span></Label>
            <Input bind:value={ip.ip_address} placeholder="192.168.1.10" />
          </div>
          <div class="space-y-1 md:col-span-2">
            <Label>网卡名称</Label>
            <Input bind:value={ip.nic_name} placeholder="eth0 / 内网网卡" />
          </div>
          <div class="space-y-1 md:col-span-2">
            <FormSelect
              label="类型"
              bind:value={ip.ip_type}
              options={$ipTypeOptions}
              placeholder="选择类型"
            />
          </div>
          <div class="space-y-1 md:col-span-2">
            <FormSelect
              label="ISP 供应商"
              bind:value={ip.isp_provider_id}
              options={providerOptions}
              placeholder="选择供应商"
            />
          </div>
          <div class="flex items-end gap-2 md:col-span-1">
            <label class="flex items-center gap-2 pb-2 text-sm">
              <input type="checkbox" bind:checked={ip.is_primary} class="h-4 w-4" />
              主 IP
            </label>
          </div>
          <div class="flex items-end justify-end gap-2 md:col-span-2">
            <Button variant="ghost" size="sm" type="button" onclick={() => removeIpRow(i)}>删除</Button>
          </div>
          <div class="space-y-1 md:col-span-12">
            <Label>描述</Label>
            <Input bind:value={ip.description} placeholder="用途 / 备注" />
          </div>
        </div>
      {/each}
      <Button variant="outline" size="sm" type="button" onclick={addIpRow}>+ 添加 IP</Button>
    </Card.Content>
  </Card.Root>

  <!-- 服务端口 -->
  <Card.Root>
    <Card.Header>
      <Card.Title>服务端口</Card.Title>
      <Card.Action class="justify-self-end self-center">
        <div class="w-64">
          <TableSelectModal
            label="从模板添加"
            multiple
            bind:value={templatePickerValue}
            searchFn={searchServerPortTemplatePaginated}
            displayOptions={[]}
            placeholder="选择端口模板快速添加"
            searchPlaceholder="输入模板名 / 服务名 / 端口..."
            modalTitle="选择端口模板"
            onConfirm={handleTemplateConfirm}
            columns={[
              { key: 'name', label: '模板名' },
              { key: 'protocol', label: '协议', width: 'w-16' },
              { key: 'port', label: '端口', width: 'w-20' },
              { key: 'service_name', label: '服务名' },
              { key: 'access_scope', label: '访问范围', width: 'w-24' },
            ]}
          />
        </div>
      </Card.Action>
      <p class="text-sm text-muted-foreground col-span-2">维护该服务器对外提供的服务端口（每台服务器独立，互不影响）</p>
    </Card.Header>
    <Card.Content class="space-y-3">
      {#each ports as port, i}
        <div class="grid gap-3 rounded-lg border p-3 md:grid-cols-12">
          <div class="space-y-1 md:col-span-2">
            <FormSelect
              label="协议 *"
              bind:value={port.protocol}
              options={$protocolOptions}
              placeholder="TCP"
            />
          </div>
          <div class="space-y-1 md:col-span-2">
            <Label>端口 <span class="text-destructive">*</span></Label>
            <Input type="number" bind:value={port.port} placeholder="80" min={1} max={65535} />
          </div>
          <div class="space-y-1 md:col-span-3">
            <Label>服务名 <span class="text-destructive">*</span></Label>
            <Input bind:value={port.service_name} placeholder="nginx" />
          </div>
          <div class="space-y-1 md:col-span-2">
            <Label>访问范围</Label>
            <Input bind:value={port.access_scope} placeholder="如：内网 / 公网 / 192.168.1.0/24" />
          </div>
          <div class="flex items-end gap-2 md:col-span-2">
            <label class="flex items-center gap-2 pb-2 text-sm">
              <input type="checkbox" bind:checked={port.is_enabled} class="h-4 w-4" />
              启用
            </label>
          </div>
          <div class="flex items-end justify-end md:col-span-1">
            <Button variant="ghost" size="sm" type="button" onclick={() => removePortRow(i)}>删除</Button>
          </div>
          <div class="space-y-1 md:col-span-12">
            <Label>描述</Label>
            <Input bind:value={port.description} placeholder="用途说明" />
          </div>
        </div>
      {/each}
      <Button variant="outline" size="sm" type="button" onclick={addPortRow}>+ 添加端口</Button>
    </Card.Content>
  </Card.Root>

  <!-- 数据库实例（勾选"数据库服务器"后维护，便于在此直接关联） -->
  {#if form.is_database_server}
    <Card.Root>
      <Card.Header>
        <Card.Title>数据库实例</Card.Title>
        <p class="text-sm text-muted-foreground">勾选了"数据库服务器"，可在此直接维护本服务器承载的数据库实例（也可在"数据库实例"菜单中维护）</p>
      </Card.Header>
      <Card.Content class="space-y-3">
        {#each dbInstances as db, i}
          <div class="grid gap-3 rounded-lg border p-3 md:grid-cols-12">
            <div class="space-y-1 md:col-span-3">
              <Label>实例名 <span class="text-destructive">*</span></Label>
              <Input bind:value={db.name} placeholder="mysql-master" />
            </div>
            <div class="space-y-1 md:col-span-2">
              <FormSelect
                label="数据库类型 *"
                bind:value={db.db_type}
                options={$databaseTypeOptions}
                placeholder="选择类型"
              />
            </div>
            <div class="space-y-1 md:col-span-2">
              <Label>端口</Label>
              <Input type="number" bind:value={db.port} placeholder="3306" min={1} max={65535} />
            </div>
            <div class="space-y-1 md:col-span-2">
              <Label>内部实例名</Label>
              <Input bind:value={db.instance_name} placeholder="如：PROD-DB-01" />
            </div>
            <div class="space-y-1 md:col-span-2">
              <FormSelect
                label="重要性"
                bind:value={db.importance}
                options={$importanceOptions}
                placeholder="选择重要性"
              />
            </div>
            <div class="flex items-end justify-end md:col-span-1">
              <Button variant="ghost" size="sm" type="button" onclick={() => removeDbRow(i)}>删除</Button>
            </div>
            <div class="space-y-1 md:col-span-12">
              <Label>描述</Label>
              <Input bind:value={db.description} placeholder="用途说明" />
            </div>
          </div>
        {/each}
        <Button variant="outline" size="sm" type="button" onclick={addDbRow}>+ 添加数据库实例</Button>
      </Card.Content>
    </Card.Root>
  {/if}

  <!-- 关联站点 -->
  <Card.Root>
    <Card.Header>
      <Card.Title>关联站点</Card.Title>
      <p class="text-sm text-muted-foreground">维护该服务器部署承载的站点（一台服务器可关联多个站点，含部署角色）</p>
    </Card.Header>
    <Card.Content class="space-y-3">
      {#each siteRels as rel, i}
        <div class="grid gap-3 rounded-lg border p-3 md:grid-cols-12">
          <div class="space-y-1 md:col-span-6">
            <FormSelect
              label="站点"
              required
              bind:value={rel.site_id}
              options={siteOptions.filter(o => o.value === rel.site_id || !siteRels.some(r => r.site_id === o.value && r !== rel))}
              placeholder="选择站点"
            />
          </div>
          <div class="space-y-1 md:col-span-3">
            <FormSelect
              label="部署角色"
              bind:value={rel.deploy_role}
              options={$siteServerRoleOptions}
              placeholder="选择角色"
            />
          </div>
          <div class="flex items-end justify-end md:col-span-1">
            <Button variant="ghost" size="sm" type="button" onclick={() => removeSiteRelRow(i)}>删除</Button>
          </div>
        </div>
      {/each}
      <Button variant="outline" size="sm" type="button" onclick={addSiteRelRow}>+ 添加站点</Button>
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
        options={$hostingTypeOptions}
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

      <FormSelect
        label="RAID级别"
        bind:value={form.raid_level}
        options={$raidLevelOptions}
        placeholder="选择RAID级别"
        disabled={!form.is_raid}
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

      <FormSelect
        label="币种"
        bind:value={form.price_currency}
        options={$currencyOptions}
        placeholder="选择币种"
      />

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

  <AttachmentFormSection bind:this={attachmentRef} targetType="server" targetId={entityId} />

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
