<script lang="ts">
  import { siteRelationsApi } from '$lib/api/site-relations';
  import { serversApi } from '$lib/api/servers';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import { domainsApi } from '$lib/api/domains';
  import type {
    SiteServerRelationResponse,
    SiteDatabaseRelationResponse,
    SiteDomainRelationResponse,
  } from '$lib/types/site_relation';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import * as Table from '$lib/ui/table';
  import * as Tabs from '$lib/ui/tabs';
  import * as Dialog from '$lib/ui/dialog';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import {
    getServerOptions,
    getDatabaseInstanceOptions,
    getDomainOptions,
  } from '$lib/utils/entity-options';
  import { siteServerRoleOptions, siteDatabaseUsageOptions, domainRoleOptions, getOptionLabel } from '$lib/utils/enum-options';
  import { formatResourceWithStatus, getResourceStatusClass } from '$lib/utils/resource-status';
  import { onMount } from 'svelte';
  import ConfirmDialog from '$lib/components/shared/ConfirmDialog.svelte';

  let {
    siteId,
  }: {
    siteId: string;
  } = $props();

  let servers = $state<SiteServerRelationResponse[]>([]);
  let databases = $state<SiteDatabaseRelationResponse[]>([]);
  let domains = $state<SiteDomainRelationResponse[]>([]);
  let loading = $state(true);
  let activeTab = $state('servers');

  let serverOptions = $state<{ label: string; value: string }[]>([]);
  let dbOptions = $state<{ label: string; value: string }[]>([]);
  let domainOptions = $state<{ label: string; value: string }[]>([]);
  let serverMap = $derived(Object.fromEntries(serverOptions.map(o => [o.value, o.label])));
  let dbMap = $derived(Object.fromEntries(dbOptions.map(o => [o.value, o.label])));
  let domainMap = $derived(Object.fromEntries(domainOptions.map(o => [o.value, o.label])));

  // 关联资源状态映射
  let serverStatusMap = $state<Record<string, string>>({});
  let databaseStatusMap = $state<Record<string, string>>({});
  let domainStatusMap = $state<Record<string, string>>({});

  // 添加对话框状态
  let addDialogOpen = $state(false);
  let addType = $state<'server' | 'database' | 'domain'>('server');
  let addEntityId = $state('');
  let addRole = $state('');
  let addDomainRole = $state('');
  let addSaving = $state(false);

  // 删除确认状态
  let confirmOpen = $state(false);
  let pendingDeleteType = $state<'server' | 'database' | 'domain'>('server');
  let pendingDeleteId = $state('');

  async function loadAll() {
    const [s, d, dom] = await Promise.all([
      siteRelationsApi.listServers(siteId),
      siteRelationsApi.listDatabases(siteId),
      siteRelationsApi.listDomains(siteId),
    ]);
    servers = s;
    databases = d;
    domains = dom;
  }

  onMount(async () => {
    try {
      const [sOpts, dOpts, domOpts, serverList, dbList, domainList] = await Promise.all([
        getServerOptions(),
        getDatabaseInstanceOptions(),
        getDomainOptions(),
        serversApi.list({ per_page: 200 }),
        databaseInstancesApi.list({ per_page: 200 }),
        domainsApi.list({ per_page: 200 }),
      ]);
      serverOptions = sOpts;
      dbOptions = dOpts;
      domainOptions = domOpts;
      serverStatusMap = Object.fromEntries(serverList.data.map((s: {id: string, status: string}) => [s.id, s.status]));
      databaseStatusMap = Object.fromEntries(dbList.data.map((d: {id: string, status: string}) => [d.id, d.status]));
      domainStatusMap = Object.fromEntries(domainList.data.map((d: {id: string, is_enabled: boolean}) => [d.id, d.is_enabled ? 'enabled' : 'disabled']));
      await loadAll();
    } catch (err) {
      console.error('Failed to load site relations:', err);
    } finally {
      loading = false;
    }
  });

  function openAdd(type: 'server' | 'database' | 'domain') {
    addType = type;
    addEntityId = '';
    addRole = '';
    addDomainRole = '';
    addDialogOpen = true;
  }

  async function handleAdd() {
    if (!addEntityId) return;
    addSaving = true;
    try {
      if (addType === 'server') {
        await siteRelationsApi.createServer({
          site_id: siteId,
          server_id: addEntityId,
          deploy_role: addRole || undefined,
        });
      } else if (addType === 'database') {
        await siteRelationsApi.createDatabase({
          site_id: siteId,
          database_instance_id: addEntityId,
          usage_type: addRole || undefined,
        });
      } else {
        await siteRelationsApi.createDomain({
          site_id: siteId,
          domain_id: addEntityId,
          domain_role: addDomainRole || undefined,
        });
      }
      addDialogOpen = false;
      await loadAll();
    } catch (err) {
      console.error('Failed to add relation:', err);
    } finally {
      addSaving = false;
    }
  }

  function handleDeleteServer(id: string) {
    pendingDeleteType = 'server';
    pendingDeleteId = id;
    confirmOpen = true;
  }

  function handleDeleteDatabase(id: string) {
    pendingDeleteType = 'database';
    pendingDeleteId = id;
    confirmOpen = true;
  }

  function handleDeleteDomain(id: string) {
    pendingDeleteType = 'domain';
    pendingDeleteId = id;
    confirmOpen = true;
  }

  async function doDelete() {
    const id = pendingDeleteId;
    const type = pendingDeleteType;
    if (!id) return;
    try {
      if (type === 'server') {
        await siteRelationsApi.deleteServer(id);
        servers = servers.filter(s => s.id !== id);
      } else if (type === 'database') {
        await siteRelationsApi.deleteDatabase(id);
        databases = databases.filter(d => d.id !== id);
      } else if (type === 'domain') {
        await siteRelationsApi.deleteDomain(id);
        domains = domains.filter(d => d.id !== id);
      }
    } catch (err) {
      console.error('Failed to delete:', err);
    }
  }
</script>

<Card.Root>
  <Card.Header>
    <Card.Title>关联资源</Card.Title>
    <Card.Description>站点与服务器、数据库实例、域名的关联关系</Card.Description>
  </Card.Header>
  <Card.Content>
    {#if loading}
      <div class="py-6 text-center text-sm text-muted-foreground">加载中...</div>
    {:else}
      <Tabs.Root value={activeTab} onValueChange={(v: string) => activeTab = v}>
        <Tabs.List>
          <Tabs.Trigger value="servers">服务器 ({servers.length})</Tabs.Trigger>
          <Tabs.Trigger value="databases">数据库 ({databases.length})</Tabs.Trigger>
          <Tabs.Trigger value="domains">域名 ({domains.length})</Tabs.Trigger>
        </Tabs.List>

        <Tabs.Content value="servers">
          <div class="space-y-3">
            <div class="flex justify-end">
              <Button size="sm" onclick={() => openAdd('server')}>添加服务器</Button>
            </div>
            <Table.Root>
              <Table.Header>
                <Table.Row>
                  <Table.Head>服务器</Table.Head>
                  <Table.Head>部署角色</Table.Head>
                  <Table.Head class="w-[100px]">操作</Table.Head>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {#each servers as rel}
                  <Table.Row>
                    <Table.Cell>
                      <a href="/servers/{rel.server_id}" class="text-primary hover:underline">
                        <span class={getResourceStatusClass(serverStatusMap[rel.server_id], 'server')}>
                          {formatResourceWithStatus(serverMap[rel.server_id] || rel.server_id, serverStatusMap[rel.server_id], 'server')}
                        </span>
                      </a>
                    </Table.Cell>
                    <Table.Cell>{getOptionLabel($siteServerRoleOptions, rel.deploy_role) || '-'}</Table.Cell>
                    <Table.Cell>
                      <Button variant="ghost" size="sm" onclick={() => handleDeleteServer(rel.id)}>
                        删除
                      </Button>
                    </Table.Cell>
                  </Table.Row>
                {:else}
                  <Table.Row>
                    <Table.Cell colspan={3} class="text-center text-muted-foreground">
                      暂无服务器关联
                    </Table.Cell>
                  </Table.Row>
                {/each}
              </Table.Body>
            </Table.Root>
          </div>
        </Tabs.Content>

        <Tabs.Content value="databases">
          <div class="space-y-3">
            <div class="flex justify-end">
              <Button size="sm" onclick={() => openAdd('database')}>添加数据库</Button>
            </div>
            <Table.Root>
              <Table.Header>
                <Table.Row>
                  <Table.Head>数据库实例</Table.Head>
                  <Table.Head>用途</Table.Head>
                  <Table.Head class="w-[100px]">操作</Table.Head>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {#each databases as rel}
                  <Table.Row>
                    <Table.Cell>
                      <a href="/database-instances/{rel.database_instance_id}" class="text-primary hover:underline">
                        <span class={getResourceStatusClass(databaseStatusMap[rel.database_instance_id], 'database')}>
                          {formatResourceWithStatus(dbMap[rel.database_instance_id] || rel.database_instance_id, databaseStatusMap[rel.database_instance_id], 'database')}
                        </span>
                      </a>
                    </Table.Cell>
                    <Table.Cell>{getOptionLabel($siteDatabaseUsageOptions, rel.usage_type) || '-'}</Table.Cell>
                    <Table.Cell>
                      <Button variant="ghost" size="sm" onclick={() => handleDeleteDatabase(rel.id)}>
                        删除
                      </Button>
                    </Table.Cell>
                  </Table.Row>
                {:else}
                  <Table.Row>
                    <Table.Cell colspan={3} class="text-center text-muted-foreground">
                      暂无数据库关联
                    </Table.Cell>
                  </Table.Row>
                {/each}
              </Table.Body>
            </Table.Root>
          </div>
        </Tabs.Content>

        <Tabs.Content value="domains">
          <div class="space-y-3">
            <div class="flex justify-end">
              <Button size="sm" onclick={() => openAdd('domain')}>添加域名</Button>
            </div>
            <Table.Root>
              <Table.Header>
                <Table.Row>
                  <Table.Head>域名</Table.Head>
                  <Table.Head>角色</Table.Head>
                  <Table.Head class="w-[100px]">操作</Table.Head>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {#each domains as rel}
                  <Table.Row>
                    <Table.Cell>
                      <a href="/domains/{rel.domain_id}" class="text-primary hover:underline">
                        <span class={getResourceStatusClass(domainStatusMap[rel.domain_id], 'ip')}>
                          {formatResourceWithStatus(domainMap[rel.domain_id] || rel.domain_id, domainStatusMap[rel.domain_id], 'ip')}
                        </span>
                      </a>
                    </Table.Cell>
                    <Table.Cell>{getOptionLabel($domainRoleOptions, rel.domain_role) || '-'}</Table.Cell>
                    <Table.Cell>
                      <Button variant="ghost" size="sm" onclick={() => handleDeleteDomain(rel.id)}>
                        删除
                      </Button>
                    </Table.Cell>
                  </Table.Row>
                {:else}
                  <Table.Row>
                    <Table.Cell colspan={3} class="text-center text-muted-foreground">
                      暂无域名关联
                    </Table.Cell>
                  </Table.Row>
                {/each}
              </Table.Body>
            </Table.Root>
          </div>
        </Tabs.Content>
      </Tabs.Root>
    {/if}
  </Card.Content>
</Card.Root>

<Dialog.Root bind:open={addDialogOpen}>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>
        {#if addType === 'server'}添加服务器关联
        {:else if addType === 'database'}添加数据库关联
        {:else}添加域名关联{/if}
      </Dialog.Title>
      <Dialog.Description>选择要关联到该站点的资源。</Dialog.Description>
    </Dialog.Header>

    <div class="space-y-4 py-2">
      {#if addType === 'server'}
        <FormSelect
          label="服务器"
          bind:value={addEntityId}
          options={serverOptions}
          placeholder="选择服务器"
        />
        <div class="space-y-2">
          <Label for="add-role">部署角色</Label>
          <Input id="add-role" bind:value={addRole} placeholder="如 web / app / db" />
        </div>
      {:else if addType === 'database'}
        <FormSelect
          label="数据库实例"
          bind:value={addEntityId}
          options={dbOptions}
          placeholder="选择数据库实例"
        />
        <div class="space-y-2">
          <Label for="add-role">用途</Label>
          <Input id="add-role" bind:value={addRole} placeholder="如 主库 / 从库" />
        </div>
      {:else}
        <FormSelect
          label="域名"
          bind:value={addEntityId}
          options={domainOptions}
          placeholder="选择域名"
        />
        <div class="space-y-2">
          <Label for="add-domain-role">域名角色</Label>
          <FormSelect
            id="add-domain-role"
            bind:value={addDomainRole}
            options={$domainRoleOptions}
            placeholder="选择域名角色"
          />
        </div>
      {/if}
    </div>

    <Dialog.Footer>
      <Button variant="outline" onclick={() => (addDialogOpen = false)}>取消</Button>
      <Button onclick={handleAdd} disabled={addSaving || !addEntityId}>
        {addSaving ? '添加中...' : '添加'}
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

<ConfirmDialog
  bind:open={confirmOpen}
  title="确认删除"
  description={`确定要删除此{pendingDeleteType === 'server' ? '服务器' : pendingDeleteType === 'database' ? '数据库' : '域名'}关联吗？`}
  confirmLabel="删除"
  onConfirm={doDelete}
/>
