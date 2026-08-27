<script lang="ts">
  import { siteRelationsApi } from '$lib/api/site-relations';
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
  import { onMount } from 'svelte';

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

  // 添加对话框状态
  let addDialogOpen = $state(false);
  let addType = $state<'server' | 'database' | 'domain'>('server');
  let addEntityId = $state('');
  let addRole = $state('');
  let addIsPrimary = $state(false);
  let addSaving = $state(false);

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
      const [sOpts, dOpts, domOpts] = await Promise.all([
        getServerOptions(),
        getDatabaseInstanceOptions(),
        getDomainOptions(),
      ]);
      serverOptions = sOpts;
      dbOptions = dOpts;
      domainOptions = domOpts;
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
    addIsPrimary = false;
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
          is_primary: addIsPrimary,
        });
      } else if (addType === 'database') {
        await siteRelationsApi.createDatabase({
          site_id: siteId,
          database_instance_id: addEntityId,
          usage_type: addRole || undefined,
          is_primary: addIsPrimary,
        });
      } else {
        await siteRelationsApi.createDomain({
          site_id: siteId,
          domain_id: addEntityId,
          is_primary: addIsPrimary,
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

  async function handleDeleteServer(id: string) {
    if (!confirm('确定要删除此服务器关联吗？')) return;
    try {
      await siteRelationsApi.deleteServer(id);
      servers = servers.filter(s => s.id !== id);
    } catch (err) {
      console.error('Failed to delete:', err);
    }
  }

  async function handleDeleteDatabase(id: string) {
    if (!confirm('确定要删除此数据库关联吗？')) return;
    try {
      await siteRelationsApi.deleteDatabase(id);
      databases = databases.filter(d => d.id !== id);
    } catch (err) {
      console.error('Failed to delete:', err);
    }
  }

  async function handleDeleteDomain(id: string) {
    if (!confirm('确定要删除此域名关联吗？')) return;
    try {
      await siteRelationsApi.deleteDomain(id);
      domains = domains.filter(d => d.id !== id);
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
                  <Table.Head>主用</Table.Head>
                  <Table.Head class="w-[100px]">操作</Table.Head>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {#each servers as rel}
                  <Table.Row>
                    <Table.Cell>
                      <a href="/servers/{rel.server_id}" class="text-primary hover:underline">
                        {serverMap[rel.server_id] || rel.server_id}
                      </a>
                    </Table.Cell>
                    <Table.Cell>{rel.deploy_role || '-'}</Table.Cell>
                    <Table.Cell>{rel.is_primary ? '是' : '-'}</Table.Cell>
                    <Table.Cell>
                      <Button variant="ghost" size="sm" onclick={() => handleDeleteServer(rel.id)}>
                        删除
                      </Button>
                    </Table.Cell>
                  </Table.Row>
                {:else}
                  <Table.Row>
                    <Table.Cell colspan={4} class="text-center text-muted-foreground">
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
                  <Table.Head>主用</Table.Head>
                  <Table.Head class="w-[100px]">操作</Table.Head>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {#each databases as rel}
                  <Table.Row>
                    <Table.Cell>
                      <a href="/database-instances/{rel.database_instance_id}" class="text-primary hover:underline">
                        {dbMap[rel.database_instance_id] || rel.database_instance_id}
                      </a>
                    </Table.Cell>
                    <Table.Cell>{rel.usage_type || '-'}</Table.Cell>
                    <Table.Cell>{rel.is_primary ? '是' : '-'}</Table.Cell>
                    <Table.Cell>
                      <Button variant="ghost" size="sm" onclick={() => handleDeleteDatabase(rel.id)}>
                        删除
                      </Button>
                    </Table.Cell>
                  </Table.Row>
                {:else}
                  <Table.Row>
                    <Table.Cell colspan={4} class="text-center text-muted-foreground">
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
                  <Table.Head>主用</Table.Head>
                  <Table.Head class="w-[100px]">操作</Table.Head>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {#each domains as rel}
                  <Table.Row>
                    <Table.Cell>
                      <a href="/domains/{rel.domain_id}" class="text-primary hover:underline">
                        {domainMap[rel.domain_id] || rel.domain_id}
                      </a>
                    </Table.Cell>
                    <Table.Cell>{rel.is_primary ? '是' : '-'}</Table.Cell>
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
      {/if}

      <div class="flex items-center gap-2">
        <input type="checkbox" id="add-primary" bind:checked={addIsPrimary} class="h-4 w-4" />
        <Label for="add-primary">主用</Label>
      </div>
    </div>

    <Dialog.Footer>
      <Button variant="outline" onclick={() => (addDialogOpen = false)}>取消</Button>
      <Button onclick={handleAdd} disabled={addSaving || !addEntityId}>
        {addSaving ? '添加中...' : '添加'}
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
