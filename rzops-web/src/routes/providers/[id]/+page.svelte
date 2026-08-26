<script lang="ts">
  import AttachmentSection from '$lib/components/shared/AttachmentSection.svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { providersApi } from '$lib/api/providers';
  import { serversApi } from '$lib/api/servers';
  import type { ProviderResponse } from '$lib/types/provider';
  import type { ServerResponse } from '$lib/types/server';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import * as Table from '$lib/ui/table';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { getOptionLabel, getOptionLabels, providerTypeOptions, commonStatusOptions } from '$lib/utils/enum-options';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';

  let provider = $state<ProviderResponse | null>(null);
  let servers = $state<ServerResponse[]>([]);
  let loading = $state(true);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/providers'); return; }

    try {
      const [provData, serverData] = await Promise.all([
        providersApi.getById(id),
        serversApi.list({ isp_provider_id: id, per_page: 100 }),
      ]);
      provider = provData;
      servers = serverData.data;
    } catch (err) {
      console.error('Failed to load provider:', err);
      goto('/providers');
    } finally {
      loading = false;
    }
  });

  async function handleDelete() {
    if (!provider) return;
    if (!confirm(`确定要删除供应商 "${provider.name}" 吗？`)) return;
    try {
      await providersApi.delete(provider.id);
      goto('/providers');
    } catch (err) {
      console.error('Failed to delete provider:', err);
    }
  }
</script>

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '供应商', href: '/providers' },
    { label: provider?.name || '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if provider}
    <!-- 页面标题 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold">{provider.name}</h1>
        <StatusBadge status={provider.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/providers')}>返回列表</Button>
        <Button onclick={() => goto(`/providers/${provider?.id}/edit`)}>编辑</Button>
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
      </div>
    </div>

    <div class="grid gap-6 lg:grid-cols-2">
      <!-- 基本信息 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>基本信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">名称</dt>
              <dd>{provider.name}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">供应商类型</dt>
              <dd>{getOptionLabels($providerTypeOptions, provider.provider_types).join(', ') || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">国家</dt>
              <dd>{provider.country || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">地址</dt>
              <dd>{provider.address || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">网站</dt>
              <dd>
                {#if provider.website}
                  <a href={provider.website} target="_blank" rel="noopener noreferrer" class="text-primary hover:underline">
                    {provider.website}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">状态</dt>
              <dd>{getOptionLabel($commonStatusOptions, provider.status)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">描述</dt>
              <dd>{provider.description || '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>

      <!-- 联系信息 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>联系信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">联系人</dt>
              <dd>{provider.contact_name || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">联系电话</dt>
              <dd>{provider.contact_phone || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">QQ</dt>
              <dd>{provider.contact_qq || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">传真</dt>
              <dd>{provider.fax || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">创建时间</dt>
              <dd>{formatDate(provider.created_at)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">更新时间</dt>
              <dd>{formatDate(provider.updated_at)}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>
    </div>

    <!-- 关联服务器 -->
    <Card.Root>
      <Card.Header>
        <Card.Title>关联服务器 ({servers.length})</Card.Title>
      </Card.Header>
      <Card.Content>
        {#if servers.length === 0}
          <p class="text-sm text-muted-foreground">暂无关联服务器</p>
        {:else}
          <Table.Root>
            <Table.Header>
              <Table.Row>
                <Table.Head>名称</Table.Head>
                <Table.Head>资产编号</Table.Head>
                <Table.Head>主IP</Table.Head>
                <Table.Head>服务器类型</Table.Head>
                <Table.Head>状态</Table.Head>
              </Table.Row>
            </Table.Header>
            <Table.Body>
              {#each servers as server}
                <Table.Row>
                  <Table.Cell>
                    <a href="/servers/{server.id}" class="text-primary hover:underline">
                      {server.name}
                    </a>
                  </Table.Cell>
                  <Table.Cell class="font-mono">{server.asset_code || '-'}</Table.Cell>
                  <Table.Cell class="font-mono">{server.primary_ip || '-'}</Table.Cell>
                  <Table.Cell>{server.server_type || '-'}</Table.Cell>
                  <Table.Cell><StatusBadge status={server.status} /></Table.Cell>
                </Table.Row>
              {/each}
            </Table.Body>
          </Table.Root>
        {/if}
      </Card.Content>
    </Card.Root>
    <AttachmentSection targetType="provider" targetId={provider.id} />
  {/if}
</div>
