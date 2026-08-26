<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import { siteRelationsApi } from '$lib/api/site-relations';
  import type { OpsSiteResponse } from '$lib/types/ops_site';
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
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import {
    getServerOptions,
    getDatabaseInstanceOptions,
    getDomainOptions,
  } from '$lib/utils/entity-options';
  import { onMount } from 'svelte';

  let site = $state<OpsSiteResponse | null>(null);
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

  onMount(async () => {
    const siteId = $page.params.id ?? "";
    if (!siteId) {
      goto('/site-relations');
      return;
    }
    try {
      const [siteData, serverRels, dbRels, domainRels, sOpts, dOpts, domOpts] = await Promise.all([
        opsSitesApi.getById(siteId),
        siteRelationsApi.listServers(siteId),
        siteRelationsApi.listDatabases(siteId),
        siteRelationsApi.listDomains(siteId),
        getServerOptions(),
        getDatabaseInstanceOptions(),
        getDomainOptions(),
      ]);
      site = siteData;
      servers = serverRels;
      databases = dbRels;
      domains = domainRels;
      serverOptions = sOpts;
      dbOptions = dOpts;
      domainOptions = domOpts;
    } catch (err) {
      console.error('Failed to load site relations:', err);
      goto('/site-relations');
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
    if (!site) return;
    if (!addEntityId) return;
    addSaving = true;
    try {
      if (addType === 'server') {
        await siteRelationsApi.createServer({
          site_id: site.id,
          server_id: addEntityId,
          deploy_role: addRole || undefined,
          is_primary: addIsPrimary,
        });
      } else if (addType === 'database') {
        await siteRelationsApi.createDatabase({
          site_id: site.id,
          database_instance_id: addEntityId,
          usage_type: addRole || undefined,
          is_primary: addIsPrimary,
        });
      } else {
        await siteRelationsApi.createDomain({
          site_id: site.id,
          domain_id: addEntityId,
          is_primary: addIsPrimary,
        });
      }
      addDialogOpen = false;
      // 刷新
      const siteId = site.id;
      const [s2, d2, dom2] = await Promise.all([
        siteRelationsApi.listServers(siteId),
        siteRelationsApi.listDatabases(siteId),
        siteRelationsApi.listDomains(siteId),
      ]);
      servers = s2;
      databases = d2;
      domains = dom2;
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

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '站点关联', href: '/site-relations' },
    { label: site?.name || '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if site}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{site.name}</h1>
        <StatusBadge status={site.status} />
      </div>
    </div>

    <Card.Root>
      <Card.Content class="pt-6">
        <div class="grid gap-4 md:grid-cols-3 text-sm">
          <div>
            <span class="text-muted-foreground">URL</span>
            <p>{site.url || '-'}</p>
          </div>
          <div>
            <span class="text-muted-foreground">服务目标</span>
            <p>{site.service_target || '-'}</p>
          </div>
          <div>
            <span class="text-muted-foreground">重要性</span>
            <p>{site.importance || '-'}</p>
          </div>
        </div>
      </Card.Content>
    </Card.Root>

    <Tabs.Root value={activeTab} onValueChange={(v: string) => activeTab = v}>
      <Tabs.List>
        <Tabs.Trigger value="servers">服务器 ({servers.length})</Tabs.Trigger>
        <Tabs.Trigger value="databases">数据库 ({databases.length})</Tabs.Trigger>
        <Tabs.Trigger value="domains">域名 ({domains.length})</Tabs.Trigger>
      </Tabs.List>

      <Tabs.Content value="servers">
        <Card.Root>
          <Card.Header class="flex items-center justify-between">
            <Card.Title>服务器关联</Card.Title>
            <Button size="sm" onclick={() => openAdd('server')}>添加服务器</Button>
          </Card.Header>
          <Card.Content>
            <Table.Root>
              <Table.Header>
                <Table.Row>
                  <Table.Head>服务器</Table.Head>
                  <Table.Head>部署角色</Table.Head>
                  <Table.Head>主用</Table.Head>
                  <Table.Head>操作</Table.Head>
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
          </Card.Content>
        </Card.Root>
      </Tabs.Content>

      <Tabs.Content value="databases">
        <Card.Root>
          <Card.Header class="flex items-center justify-between">
            <Card.Title>数据库关联</Card.Title>
            <Button size="sm" onclick={() => openAdd('database')}>添加数据库</Button>
          </Card.Header>
          <Card.Content>
            <Table.Root>
              <Table.Header>
                <Table.Row>
                  <Table.Head>数据库实例</Table.Head>
                  <Table.Head>用途</Table.Head>
                  <Table.Head>主用</Table.Head>
                  <Table.Head>操作</Table.Head>
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
          </Card.Content>
        </Card.Root>
      </Tabs.Content>

      <Tabs.Content value="domains">
        <Card.Root>
          <Card.Header class="flex items-center justify-between">
            <Card.Title>域名关联</Card.Title>
            <Button size="sm" onclick={() => openAdd('domain')}>添加域名</Button>
          </Card.Header>
          <Card.Content>
            <Table.Root>
              <Table.Header>
                <Table.Row>
                  <Table.Head>域名</Table.Head>
                  <Table.Head>主用</Table.Head>
                  <Table.Head>操作</Table.Head>
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
          </Card.Content>
        </Card.Root>
      </Tabs.Content>
    </Tabs.Root>
  {/if}
</div>

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
