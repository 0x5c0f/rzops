<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import { siteRelationsApi } from '$lib/api/site-relations';
  import type { OpsSiteResponse } from '$lib/types/ops_site';
  import type { SiteServerRelationResponse, SiteDatabaseRelationResponse, SiteDomainRelationResponse } from '$lib/types/site_relation';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import * as Table from '$lib/ui/table';
  import * as Tabs from '$lib/ui/tabs';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { onMount } from 'svelte';

  let site = $state<OpsSiteResponse | null>(null);
  let servers = $state<SiteServerRelationResponse[]>([]);
  let databases = $state<SiteDatabaseRelationResponse[]>([]);
  let domains = $state<SiteDomainRelationResponse[]>([]);
  let loading = $state(true);
  let activeTab = $state('servers');

  onMount(async () => {
    const siteId = $page.params.id ?? "";
    if (!siteId) {
      goto('/site-relations');
      return;
    }
    try {
      site = await opsSitesApi.getById(siteId);
      [servers, databases, domains] = await Promise.all([
        siteRelationsApi.listServers(siteId),
        siteRelationsApi.listDatabases(siteId),
        siteRelationsApi.listDomains(siteId),
      ]);
    } catch (err) {
      console.error('Failed to load site relations:', err);
      goto('/site-relations');
    } finally {
      loading = false;
    }
  });

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
    <div class="text-muted-foreground">加载中...</div>
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
          <Card.Content>
            <Table.Root>
              <Table.Header>
                <Table.Row>
                  <Table.Head>服务器ID</Table.Head>
                  <Table.Head>角色</Table.Head>
                  <Table.Head>备注</Table.Head>
                  <Table.Head>操作</Table.Head>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {#each servers as rel}
                  <Table.Row>
                    <Table.Cell>{rel.server_id}</Table.Cell>
                    <Table.Cell>{rel.role || '-'}</Table.Cell>
                    <Table.Cell>{rel.remarks || '-'}</Table.Cell>
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
          <Card.Content>
            <Table.Root>
              <Table.Header>
                <Table.Row>
                  <Table.Head>数据库实例ID</Table.Head>
                  <Table.Head>角色</Table.Head>
                  <Table.Head>备注</Table.Head>
                  <Table.Head>操作</Table.Head>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {#each databases as rel}
                  <Table.Row>
                    <Table.Cell>{rel.database_instance_id}</Table.Cell>
                    <Table.Cell>{rel.role || '-'}</Table.Cell>
                    <Table.Cell>{rel.remarks || '-'}</Table.Cell>
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
          <Card.Content>
            <Table.Root>
              <Table.Header>
                <Table.Row>
                  <Table.Head>域名ID</Table.Head>
                  <Table.Head>角色</Table.Head>
                  <Table.Head>备注</Table.Head>
                  <Table.Head>操作</Table.Head>
                </Table.Row>
              </Table.Header>
              <Table.Body>
                {#each domains as rel}
                  <Table.Row>
                    <Table.Cell>{rel.domain_id}</Table.Cell>
                    <Table.Cell>{rel.role || '-'}</Table.Cell>
                    <Table.Cell>{rel.remarks || '-'}</Table.Cell>
                    <Table.Cell>
                      <Button variant="ghost" size="sm" onclick={() => handleDeleteDomain(rel.id)}>
                        删除
                      </Button>
                    </Table.Cell>
                  </Table.Row>
                {:else}
                  <Table.Row>
                    <Table.Cell colspan={4} class="text-center text-muted-foreground">
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
