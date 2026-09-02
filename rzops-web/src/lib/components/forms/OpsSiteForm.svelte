<script lang="ts">
  import type { CreateOpsSiteRequest } from '$lib/types/ops_site';
import AttachmentFormSection from '$lib/components/shared/AttachmentFormSection.svelte';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TableSelectModal from '$lib/components/shared/TableSelectModal.svelte';
  import DateField from '$lib/components/shared/DateField.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { siteStatusOptions, importanceOptions, serviceTargetOptions, codeRepoTypeOptions, monitorTypeOptions, commonStatusOptions, serverStatusOptions, serverTypeOptions, databaseStatusOptions, getOptionLabel } from '$lib/utils/enum-options';
  import { backupPlansApi } from '$lib/api/backup-plans';
  import { monitorTargetsApi } from '$lib/api/monitor-targets';
  import { siteRelationsApi } from '$lib/api/site-relations';
  import { searchServerPaginated, searchDatabasePaginated, searchDomainPaginated } from '$lib/utils/entity-options';
  import { validate } from '$lib/utils/validation';
  import { onMount } from 'svelte';
  import * as Tabs from '$lib/ui/tabs';
  import * as Dialog from '$lib/ui/dialog';

  interface ServerRelationDraft {
    id?: string;
    server_id: string;
    server_name?: string;
    deploy_role: string;
    is_primary: boolean;
  }
  interface DatabaseRelationDraft {
    id?: string;
    database_instance_id: string;
    database_name?: string;
    usage_type: string;
    is_primary: boolean;
  }
  interface DomainRelationDraft {
    id?: string;
    domain_id: string;
    domain_name?: string;
    is_primary: boolean;
  }

  interface BackupDraft {
    id?: string;
    name: string;
    schedule: string;
    retention_days: string;
    status: string;
  }
  interface MonitorDraft {
    id?: string;
    name: string;
    monitor_type: string;
    endpoint: string;
    interval_seconds: string;
    status: string;
  }

  let {
    initial = {} as CreateOpsSiteRequest,
    entityId = '',
    initialBackupPlans = [] as BackupDraft[],
    initialMonitorTargets = [] as MonitorDraft[],
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateOpsSiteRequest;
    entityId?: string;
    initialBackupPlans?: BackupDraft[];
    initialMonitorTargets?: MonitorDraft[];
    submitLabel?: string;
    onSubmit: (data: CreateOpsSiteRequest) => Promise<string | void>;
  } = $props();

  let saving = $state(false);
  let formError = $state<string | null>(null);
  let attachmentRef = $state<{ uploadAll: (id: string) => Promise<void> } | null>(null);

  let form = $state<CreateOpsSiteRequest>(createInitial(initial));
  let backupPlans = $state<BackupDraft[]>(JSON.parse(JSON.stringify(initialBackupPlans)));
  let monitorTargets = $state<MonitorDraft[]>(JSON.parse(JSON.stringify(initialMonitorTargets)));

  // 关联资源草稿
  let serverRelations = $state<ServerRelationDraft[]>([]);
  let databaseRelations = $state<DatabaseRelationDraft[]>([]);
  let domainRelations = $state<DomainRelationDraft[]>([]);
  let activeRelationTab = $state('servers');

  // 添加关联对话框状态
  let addRelationOpen = $state(false);
  let addRelationType = $state<'server' | 'database' | 'domain'>('server');
  let addRelationIds = $state<string[]>([]);
  let addRelationRole = $state('');
  let addRelationIsPrimary = $state(false);
  let addRelationSaving = $state(false);

  // 编辑时加载已有关联
  onMount(async () => {
    if (!entityId) return;
    try {
      const [servers, databases, domains] = await Promise.all([
        siteRelationsApi.listServers(entityId),
        siteRelationsApi.listDatabases(entityId),
        siteRelationsApi.listDomains(entityId),
      ]);
      serverRelations = servers.map(s => ({
        id: s.id,
        server_id: s.server_id,
        deploy_role: s.deploy_role || '',
        is_primary: s.is_primary,
      }));
      databaseRelations = databases.map(d => ({
        id: d.id,
        database_instance_id: d.database_instance_id,
        usage_type: d.usage_type || '',
        is_primary: d.is_primary,
      }));
      domainRelations = domains.map(d => ({
        id: d.id,
        domain_id: d.domain_id,
        is_primary: d.is_primary,
      }));
    } catch (err) {
      console.error('Failed to load site relations:', err);
    }
  });

  function createInitial(initial?: CreateOpsSiteRequest): CreateOpsSiteRequest {
    return {
      name: '',
      url: '',
      service_target: '',
      importance: '',
      purpose: '',
      language_runtime: '',
      web_framework: '',
      code_repo_type: '',
      code_repo_url: '',
      function_summary: '',
      remarks: '',
      status: 'active',
      is_test_site: false,
      ...JSON.parse(JSON.stringify(initial ?? {})),
    };
  }

  // 是否处于下线状态（临时/永久下线时才需要填写下线时间与原因）
  let isOffline = $derived(
    form.status === 'temporary_offline' || form.status === 'permanent_offline',
  );

  function emptyBackup(): BackupDraft {
    return { name: '', schedule: '', retention_days: '', status: 'active' };
  }
  function emptyMonitor(): MonitorDraft {
    return { name: '', monitor_type: '', endpoint: '', interval_seconds: '', status: 'active' };
  }
  function addBackupRow() {
    backupPlans = [...backupPlans, emptyBackup()];
  }
  function removeBackupRow(index: number) {
    backupPlans = backupPlans.filter((_, i) => i !== index);
  }
  function addMonitorRow() {
    monitorTargets = [...monitorTargets, emptyMonitor()];
  }
  function removeMonitorRow(index: number) {
    monitorTargets = monitorTargets.filter((_, i) => i !== index);
  }

  // 备份计划增量同步（target_type 固定为 site）
  async function syncBackupPlans(siteId: string) {
    for (const bp of initialBackupPlans) {
      if (bp.id && !backupPlans.some(r => r.id === bp.id)) {
        await backupPlansApi.delete(bp.id);
      }
    }
    for (const row of backupPlans) {
      const payload = {
        name: row.name.trim(),
        target_type: 'site',
        target_id: siteId,
        schedule: row.schedule || undefined,
        retention_days: row.retention_days ? Number(row.retention_days) : undefined,
        status: row.status,
      };
      if (row.id) {
        await backupPlansApi.update(row.id, payload);
      } else {
        await backupPlansApi.create(payload);
      }
    }
  }

  // 监控目标增量同步（target_type 固定为 site）
  async function syncMonitorTargets(siteId: string) {
    for (const mt of initialMonitorTargets) {
      if (mt.id && !monitorTargets.some(r => r.id === mt.id)) {
        await monitorTargetsApi.delete(mt.id);
      }
    }
    for (const row of monitorTargets) {
      const payload = {
        name: row.name.trim(),
        target_type: 'site',
        target_id: siteId,
        monitor_type: row.monitor_type || undefined,
        endpoint: row.endpoint || undefined,
        interval_seconds: row.interval_seconds ? Number(row.interval_seconds) : undefined,
        status: row.status,
      };
      if (row.id) {
        await monitorTargetsApi.update(row.id, payload);
      } else {
        await monitorTargetsApi.create(payload);
      }
    }
  }

  // 关联资源同步
  async function syncRelations(siteId: string) {
    // 删除已移除的旧关联
    const currentServerIds = new Set(serverRelations.filter(r => r.id).map(r => r.id));
    const currentDbIds = new Set(databaseRelations.filter(r => r.id).map(r => r.id));
    const currentDomainIds = new Set(domainRelations.filter(r => r.id).map(r => r.id));

    // 注意：这里需要知道初始的关联列表，但我们没有保存初始列表
    // 简化方案：只创建新关联，不删除旧关联（删除通过UI直接调用API）
    // 实际上，我们在 removeRelation 时已经直接调用了删除API

    // 创建新关联
    for (const rel of serverRelations) {
      if (!rel.id) {
        await siteRelationsApi.createServer({
          site_id: siteId,
          server_id: rel.server_id,
          deploy_role: rel.deploy_role || undefined,
          is_primary: rel.is_primary,
        });
      }
    }
    for (const rel of databaseRelations) {
      if (!rel.id) {
        await siteRelationsApi.createDatabase({
          site_id: siteId,
          database_instance_id: rel.database_instance_id,
          usage_type: rel.usage_type || undefined,
          is_primary: rel.is_primary,
        });
      }
    }
    for (const rel of domainRelations) {
      if (!rel.id) {
        await siteRelationsApi.createDomain({
          site_id: siteId,
          domain_id: rel.domain_id,
          is_primary: rel.is_primary,
        });
      }
    }
  }

  // 打开添加关联对话框
  function openAddRelation(type: 'server' | 'database' | 'domain') {
    addRelationType = type;
    addRelationIds = [];
    addRelationRole = '';
    addRelationIsPrimary = false;
    addRelationOpen = true;
  }

  // 确认添加关联
  async function confirmAddRelation() {
    if (addRelationIds.length === 0) return;
    addRelationSaving = true;
    try {
      // 如果是编辑模式，直接创建关联并获取 id
      if (entityId) {
        for (const targetId of addRelationIds) {
          if (addRelationType === 'server') {
            const created = await siteRelationsApi.createServer({
              site_id: entityId,
              server_id: targetId,
              deploy_role: addRelationRole || undefined,
              is_primary: addRelationIsPrimary,
            });
            serverRelations = [...serverRelations, {
              id: created.id,
              server_id: targetId,
              deploy_role: addRelationRole,
              is_primary: addRelationIsPrimary,
            }];
          } else if (addRelationType === 'database') {
            const created = await siteRelationsApi.createDatabase({
              site_id: entityId,
              database_instance_id: targetId,
              usage_type: addRelationRole || undefined,
              is_primary: addRelationIsPrimary,
            });
            databaseRelations = [...databaseRelations, {
              id: created.id,
              database_instance_id: targetId,
              usage_type: addRelationRole,
              is_primary: addRelationIsPrimary,
            }];
          } else {
            const created = await siteRelationsApi.createDomain({
              site_id: entityId,
              domain_id: targetId,
              is_primary: addRelationIsPrimary,
            });
            domainRelations = [...domainRelations, {
              id: created.id,
              domain_id: targetId,
              is_primary: addRelationIsPrimary,
            }];
          }
        }
      } else {
        // 新建模式，添加到草稿
        for (const targetId of addRelationIds) {
          if (addRelationType === 'server') {
            serverRelations = [...serverRelations, {
              server_id: targetId,
              deploy_role: addRelationRole,
              is_primary: addRelationIsPrimary,
            }];
          } else if (addRelationType === 'database') {
            databaseRelations = [...databaseRelations, {
              database_instance_id: targetId,
              usage_type: addRelationRole,
              is_primary: addRelationIsPrimary,
            }];
          } else {
            domainRelations = [...domainRelations, {
              domain_id: targetId,
              is_primary: addRelationIsPrimary,
            }];
          }
        }
      }
      addRelationOpen = false;
    } catch (err) {
      console.error('Failed to add relation:', err);
      formError = '添加关联失败，请重试';
    } finally {
      addRelationSaving = false;
    }
  }

  // 删除关联
  async function removeRelation(type: 'server' | 'database' | 'domain', index: number) {
    if (type === 'server') {
      const rel = serverRelations[index];
      if (rel.id && entityId) {
        try { await siteRelationsApi.deleteServer(rel.id); } catch { /* ignore */ }
      }
      serverRelations = serverRelations.filter((_, i) => i !== index);
    } else if (type === 'database') {
      const rel = databaseRelations[index];
      if (rel.id && entityId) {
        try { await siteRelationsApi.deleteDatabase(rel.id); } catch { /* ignore */ }
      }
      databaseRelations = databaseRelations.filter((_, i) => i !== index);
    } else {
      const rel = domainRelations[index];
      if (rel.id && entityId) {
        try { await siteRelationsApi.deleteDomain(rel.id); } catch { /* ignore */ }
      }
      domainRelations = domainRelations.filter((_, i) => i !== index);
    }
  }

  async function handleSave() {
    formError = validate([
      { value: form.name, label: '站点名称', required: true, maxLength: 100 },
      { value: form.url, label: '站点URL', required: true, format: 'url' },
      { value: form.status, label: '状态', required: true },
    ]);
    if (formError) return;
    if (backupPlans.some(r => !r.name.trim())) {
      formError = '备份计划的名称必填，请填写完整或删除空行';
      return;
    }
    if (monitorTargets.some(r => !r.name.trim())) {
      formError = '监控目标的名称必填，请填写完整或删除空行';
      return;
    }
    saving = true;
    try {
      const id = await onSubmit(form);
      if (id) {
        await syncBackupPlans(id);
        await syncMonitorTargets(id);
        await syncRelations(id);
        await attachmentRef?.uploadAll(id);
      }
    } catch (err) {
      console.error('Failed to save ops site:', err);
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
      <div class="space-y-2">
        <Label for="name">名称 <span class="text-destructive">*</span></Label>
        <Input id="name" bind:value={form.name} required />
      </div>

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={$siteStatusOptions}
        required
      />

      <div class="space-y-2">
        <Label for="url">URL <span class="text-destructive">*</span></Label>
        <Input id="url" bind:value={form.url} required placeholder="https://..." />
      </div>

      <FormSelect
        label="服务目标"
        bind:value={form.service_target}
        options={$serviceTargetOptions}
        placeholder="选择服务目标"
      />

      <FormSelect
        label="重要性"
        bind:value={form.importance}
        options={$importanceOptions}
        placeholder="选择重要性"
      />

      <div class="space-y-2">
        <Label for="purpose">用途</Label>
        <Input id="purpose" bind:value={form.purpose} />
      </div>

      <div class="space-y-2">
        <Label for="language_runtime">语言/运行时</Label>
        <Input id="language_runtime" bind:value={form.language_runtime} />
      </div>

      <div class="space-y-2">
        <Label for="web_framework">Web框架</Label>
        <Input id="web_framework" bind:value={form.web_framework} />
      </div>

      <FormSelect
        label="代码仓库类型"
        bind:value={form.code_repo_type}
        options={$codeRepoTypeOptions}
        placeholder="选择仓库类型"
      />

      <div class="space-y-2">
        <Label for="code_repo_url">代码仓库地址</Label>
        <Input id="code_repo_url" bind:value={form.code_repo_url} />
      </div>

      <div class="flex items-center gap-4 pt-6">
        <div class="flex items-center gap-2">
          <input id="is_test_site" type="checkbox" bind:checked={form.is_test_site} class="h-4 w-4 rounded border-gray-300" />
          <Label for="is_test_site">测试站点</Label>
        </div>
      </div>

      {#if isOffline}
        <DateField
          id="offline_time"
          label="下线时间"
          bind:value={form.offline_time}
        />

        <div class="space-y-2">
          <Label for="offline_reason">下线原因</Label>
          <Input id="offline_reason" bind:value={form.offline_reason} placeholder="说明下线原因" />
        </div>
      {/if}

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="function_summary">功能概述</Label>
        <TextArea id="function_summary" bind:value={form.function_summary} rows={3} />
      </div>

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="remarks">备注</Label>
        <TextArea id="remarks" bind:value={form.remarks} rows={3} />
      </div>
    </Card.Content>
  </Card.Root>

  <!-- 关联资源 -->
  <Card.Root>
    <Card.Header>
      <Card.Title>关联资源</Card.Title>
      <Card.Description>站点与服务器、数据库实例、域名的关联关系</Card.Description>
    </Card.Header>
    <Card.Content>
      <Tabs.Root value={activeRelationTab} onValueChange={(v: string) => activeRelationTab = v}>
        <Tabs.List>
          <Tabs.Trigger value="servers">服务器 ({serverRelations.length})</Tabs.Trigger>
          <Tabs.Trigger value="databases">数据库 ({databaseRelations.length})</Tabs.Trigger>
          <Tabs.Trigger value="domains">域名 ({domainRelations.length})</Tabs.Trigger>
        </Tabs.List>

        <Tabs.Content value="servers" class="space-y-3 pt-4">
          <div class="flex justify-end">
            <Button size="sm" onclick={() => openAddRelation('server')}>添加服务器</Button>
          </div>
          {#if serverRelations.length === 0}
            <div class="py-6 text-center text-sm text-muted-foreground">暂无服务器关联</div>
          {:else}
            <div class="space-y-2">
              {#each serverRelations as rel, i (rel.id || `new-${i}`)}
                <div class="flex items-center justify-between rounded-lg border p-3">
                  <div class="flex items-center gap-3">
                    <a href="/servers/{rel.server_id}" class="text-primary hover:underline">{rel.server_name || rel.server_id}</a>
                    {#if rel.deploy_role}
                      <span class="text-xs text-muted-foreground">角色: {rel.deploy_role}</span>
                    {/if}
                    {#if rel.is_primary}
                      <span class="rounded bg-primary/10 px-2 py-0.5 text-xs text-primary">主用</span>
                    {/if}
                  </div>
                  <Button variant="ghost" size="sm" onclick={() => removeRelation('server', i)}>删除</Button>
                </div>
              {/each}
            </div>
          {/if}
        </Tabs.Content>

        <Tabs.Content value="databases" class="space-y-3 pt-4">
          <div class="flex justify-end">
            <Button size="sm" onclick={() => openAddRelation('database')}>添加数据库</Button>
          </div>
          {#if databaseRelations.length === 0}
            <div class="py-6 text-center text-sm text-muted-foreground">暂无数据库关联</div>
          {:else}
            <div class="space-y-2">
              {#each databaseRelations as rel, i (rel.id || `new-${i}`)}
                <div class="flex items-center justify-between rounded-lg border p-3">
                  <div class="flex items-center gap-3">
                    <a href="/database-instances/{rel.database_instance_id}" class="text-primary hover:underline">{rel.database_name || rel.database_instance_id}</a>
                    {#if rel.usage_type}
                      <span class="text-xs text-muted-foreground">用途: {rel.usage_type}</span>
                    {/if}
                    {#if rel.is_primary}
                      <span class="rounded bg-primary/10 px-2 py-0.5 text-xs text-primary">主用</span>
                    {/if}
                  </div>
                  <Button variant="ghost" size="sm" onclick={() => removeRelation('database', i)}>删除</Button>
                </div>
              {/each}
            </div>
          {/if}
        </Tabs.Content>

        <Tabs.Content value="domains" class="space-y-3 pt-4">
          <div class="flex justify-end">
            <Button size="sm" onclick={() => openAddRelation('domain')}>添加域名</Button>
          </div>
          {#if domainRelations.length === 0}
            <div class="py-6 text-center text-sm text-muted-foreground">暂无域名关联</div>
          {:else}
            <div class="space-y-2">
              {#each domainRelations as rel, i (rel.id || `new-${i}`)}
                <div class="flex items-center justify-between rounded-lg border p-3">
                  <div class="flex items-center gap-3">
                    <a href="/domains/{rel.domain_id}" class="text-primary hover:underline">{rel.domain_name || rel.domain_id}</a>
                    {#if rel.is_primary}
                      <span class="rounded bg-primary/10 px-2 py-0.5 text-xs text-primary">主用</span>
                    {/if}
                  </div>
                  <Button variant="ghost" size="sm" onclick={() => removeRelation('domain', i)}>删除</Button>
                </div>
              {/each}
            </div>
          {/if}
        </Tabs.Content>
      </Tabs.Root>
    </Card.Content>
  </Card.Root>

  <!-- 添加关联对话框 -->
  <Dialog.Root bind:open={addRelationOpen}>
    <Dialog.Content class="sm:max-w-2xl">
      <Dialog.Header>
        <Dialog.Title>
          {#if addRelationType === 'server'}添加服务器关联
          {:else if addRelationType === 'database'}添加数据库关联
          {:else}添加域名关联{/if}
        </Dialog.Title>
        <Dialog.Description>选择要关联到该站点的资源。</Dialog.Description>
      </Dialog.Header>

      <div class="space-y-4 py-2">
        {#if addRelationType === 'server'}
          <TableSelectModal
            label="服务器 *"
            multiple
            bind:value={addRelationIds}
            searchFn={searchServerPaginated}
            placeholder="选择服务器（可多选）"
            searchPlaceholder="输入名称或 IP 搜索..."
            modalTitle="选择服务器"
            required
            columns={[
              { key: 'name', label: '服务器名称' },
              { key: 'primary_ip', label: '主IP', width: 'w-32' },
              { key: 'status', label: '状态', width: 'w-20', render: (item: Record<string, unknown>) => getOptionLabel($serverStatusOptions, String(item.status ?? '')) },
              { key: 'server_type', label: '类型', width: 'w-24', render: (item: Record<string, unknown>) => getOptionLabel($serverTypeOptions, String(item.server_type ?? '')) },
            ]}
          />
          <div class="space-y-2">
            <Label for="add-role">部署角色</Label>
            <Input id="add-role" bind:value={addRelationRole} placeholder="如 web / app / db" />
          </div>
        {:else if addRelationType === 'database'}
          <TableSelectModal
            label="数据库实例 *"
            multiple
            bind:value={addRelationIds}
            searchFn={searchDatabasePaginated}
            placeholder="选择数据库实例（可多选）"
            searchPlaceholder="输入名称搜索..."
            modalTitle="选择数据库实例"
            required
            columns={[
              { key: 'name', label: '实例名称' },
              { key: 'db_type', label: '类型', width: 'w-24' },
              { key: 'port', label: '端口', width: 'w-20' },
              { key: 'status', label: '状态', width: 'w-20', render: (item: Record<string, unknown>) => getOptionLabel($databaseStatusOptions, String(item.status ?? '')) },
            ]}
          />
          <div class="space-y-2">
            <Label for="add-role">用途</Label>
            <Input id="add-role" bind:value={addRelationRole} placeholder="如 主库 / 从库" />
          </div>
        {:else}
          <TableSelectModal
            label="域名 *"
            multiple
            bind:value={addRelationIds}
            searchFn={searchDomainPaginated}
            placeholder="选择域名（可多选）"
            searchPlaceholder="输入域名搜索..."
            modalTitle="选择域名"
            required
            columns={[
              { key: 'name', label: '域名' },
              { key: 'registrar', label: '注册商', width: 'w-32' },
              { key: 'expire_date', label: '到期时间', width: 'w-28' },
            ]}
          />
        {/if}

        <div class="flex items-center gap-2">
          <input type="checkbox" id="add-primary" bind:checked={addRelationIsPrimary} class="h-4 w-4" />
          <Label for="add-primary">主用</Label>
        </div>
      </div>

      <Dialog.Footer>
        <Button variant="outline" onclick={() => (addRelationOpen = false)}>取消</Button>
        <Button onclick={confirmAddRelation} disabled={addRelationSaving || addRelationIds.length === 0}>
          {addRelationSaving ? '添加中...' : '添加'}
        </Button>
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>

  <!-- 备份计划：直接在此内联维护 -->
  <Card.Root>
    <Card.Header>
      <Card.Title>备份计划</Card.Title>
      <Card.Description>站点可作为独立备份对象，直接在此维护备份计划（也可在"备份计划"菜单维护）</Card.Description>
    </Card.Header>
    <Card.Content class="space-y-3">
      {#each backupPlans as bp, i (i)}
        <div class="grid gap-3 rounded-lg border p-3 md:grid-cols-2 lg:grid-cols-5">
          <div class="space-y-1">
            <Label>名称 <span class="text-destructive">*</span></Label>
            <Input bind:value={bp.name} placeholder="如：官网站点每日备份" />
          </div>
          <div class="space-y-1">
            <Label>调度计划</Label>
            <Input bind:value={bp.schedule} placeholder="cron 表达式" />
          </div>
          <div class="space-y-1">
            <Label>保留天数</Label>
            <Input type="number" bind:value={bp.retention_days} />
          </div>
          <FormSelect label="状态" bind:value={bp.status} options={$commonStatusOptions} />
          <div class="flex items-end">
            <Button variant="outline" size="sm" onclick={() => removeBackupRow(i)}>移除</Button>
          </div>
        </div>
      {/each}
      <Button variant="outline" size="sm" onclick={addBackupRow}>+ 添加备份计划</Button>
    </Card.Content>
  </Card.Root>

  <!-- 监控目标：直接在此内联维护 -->
  <Card.Root>
    <Card.Header>
      <Card.Title>监控目标</Card.Title>
      <Card.Description>为站点配置监控（HTTP / TCP 等），直接在此维护（也可在"监控目标"菜单维护）</Card.Description>
    </Card.Header>
    <Card.Content class="space-y-3">
      {#each monitorTargets as mt, i (i)}
        <div class="grid gap-3 rounded-lg border p-3 md:grid-cols-2 lg:grid-cols-5">
          <div class="space-y-1">
            <Label>名称 <span class="text-destructive">*</span></Label>
            <Input bind:value={mt.name} placeholder="如：官网 HTTP 监控" />
          </div>
          <FormSelect label="监控类型" bind:value={mt.monitor_type} options={$monitorTypeOptions} placeholder="选择类型" />
          <div class="space-y-1">
            <Label>端点</Label>
            <Input bind:value={mt.endpoint} placeholder="URL / IP:Port" />
          </div>
          <div class="space-y-1">
            <Label>间隔(秒)</Label>
            <Input type="number" bind:value={mt.interval_seconds} />
          </div>
          <div class="flex items-end">
            <Button variant="outline" size="sm" onclick={() => removeMonitorRow(i)}>移除</Button>
          </div>
        </div>
      {/each}
      <Button variant="outline" size="sm" onclick={addMonitorRow}>+ 添加监控目标</Button>
    </Card.Content>
  </Card.Root>

    <AttachmentFormSection bind:this={attachmentRef} targetType="site" targetId={entityId} />

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
