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
  import * as Card from '$lib/ui/card';
  import * as Table from '$lib/ui/table';
  import * as Tabs from '$lib/ui/tabs';
  import {
    siteServerRoleOptions,
    siteDatabaseUsageOptions,
    domainRoleOptions,
    getOptionLabel,
  } from '$lib/utils/enum-options';
  import { formatResourceWithStatus, getResourceStatusClass } from '$lib/utils/resource-status';
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

  // 名称映射：id -> name，用于展示关联资源名称
  let serverMap = $state<Record<string, string>>({});
  let dbMap = $state<Record<string, string>>({});
  let domainMap = $state<Record<string, string>>({});
  // 关联资源状态映射
  let serverStatusMap = $state<Record<string, string>>({});
  let databaseStatusMap = $state<Record<string, string>>({});
  let domainStatusMap = $state<Record<string, string>>({});

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
      const [serverList, dbList, domainList] = await Promise.all([
        serversApi.list({ per_page: 200 }),
        databaseInstancesApi.list({ per_page: 200 }),
        domainsApi.list({ per_page: 200 }),
        loadAll(),
      ]);
      serverMap = Object.fromEntries(serverList.data.map((s: {id: string, name: string}) => [s.id, s.name]));
      dbMap = Object.fromEntries(dbList.data.map((d: {id: string, name: string}) => [d.id, d.name]));
      domainMap = Object.fromEntries(domainList.data.map((d: {id: string, domain_name: string}) => [d.id, d.domain_name]));
      serverStatusMap = Object.fromEntries(serverList.data.map((s: {id: string, status: string}) => [s.id, s.status]));
      databaseStatusMap = Object.fromEntries(dbList.data.map((d: {id: string, status: string}) => [d.id, d.status]));
      domainStatusMap = Object.fromEntries(domainList.data.map((d: {id: string, is_enabled: boolean}) => [d.id, d.is_enabled ? 'enabled' : 'disabled']));
    } catch (err) {
      console.error('Failed to load site relations:', err);
    } finally {
      loading = false;
    }
  });
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
          <Table.Root>
            <Table.Header>
              <Table.Row>
                <Table.Head>服务器</Table.Head>
                <Table.Head>部署角色</Table.Head>
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
                </Table.Row>
              {:else}
                <Table.Row>
                  <Table.Cell colspan={2} class="text-center text-muted-foreground">
                    暂无服务器关联
                  </Table.Cell>
                </Table.Row>
              {/each}
            </Table.Body>
          </Table.Root>
        </Tabs.Content>

        <Tabs.Content value="databases">
          <Table.Root>
            <Table.Header>
              <Table.Row>
                <Table.Head>数据库实例</Table.Head>
                <Table.Head>用途</Table.Head>
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
                </Table.Row>
              {:else}
                <Table.Row>
                  <Table.Cell colspan={2} class="text-center text-muted-foreground">
                    暂无数据库关联
                  </Table.Cell>
                </Table.Row>
              {/each}
            </Table.Body>
          </Table.Root>
        </Tabs.Content>

        <Tabs.Content value="domains">
          <Table.Root>
            <Table.Header>
              <Table.Row>
                <Table.Head>域名</Table.Head>
                <Table.Head>角色</Table.Head>
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
                </Table.Row>
              {:else}
                <Table.Row>
                  <Table.Cell colspan={2} class="text-center text-muted-foreground">
                    暂无域名关联
                  </Table.Cell>
                </Table.Row>
              {/each}
            </Table.Body>
          </Table.Root>
        </Tabs.Content>
      </Tabs.Root>
    {/if}
  </Card.Content>
</Card.Root>
