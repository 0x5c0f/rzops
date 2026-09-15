<script lang="ts">
  import AttachmentSection from '$lib/components/shared/AttachmentSection.svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { domainsApi } from '$lib/api/domains';
  import type { DomainResponse } from '$lib/types/domain';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { domainPrivacyStatusOptions, getOptionLabel } from '$lib/utils/enum-options';
  import { formatResourceWithStatus, getResourceStatusClass } from '$lib/utils/resource-status';
  import { providersApi } from '$lib/api/providers';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';
  import ConfirmDialog from '$lib/components/shared/ConfirmDialog.svelte';
import { canUpdate, canDelete } from '$lib/utils/permissions';

  let domain = $state<DomainResponse | null>(null);

  let confirmOpen = $state(false);
  let loading = $state(true);
  let providerMap = $state<Record<string, string>>({});
  let providerStatusMap = $state<Record<string, string>>({});

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/domains'); return; }
    try {
      const [d, provOptions, providerList] = await Promise.all([
        domainsApi.getById(id),
        getProviderOptions(),
        providersApi.list({ per_page: 200 }),
      ]);
      domain = d;
      providerMap = Object.fromEntries(provOptions.map(o => [o.value, o.label]));
      providerStatusMap = Object.fromEntries(providerList.data.map((p: {id: string, status?: string}) => [p.id, p.status || 'active']));
    } catch (err) {
      console.error('Failed to load domain:', err);
      goto('/domains');
    } finally {
      loading = false;
    }
  });

  function handleDelete() {
    if (!domain) return;
    confirmOpen = true;
  }

  async function doDelete() {
    if (!domain) return;
    try {
      await domainsApi.delete(domain.id);
      goto('/domains');
      } catch (err) {
      console.error('Failed to delete domain:', err);
    }
  }
</script>

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '域名', href: '/domains' },
    { label: domain?.domain_name || '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if domain}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold">{domain.domain_name}</h1>
        <StatusBadge status={domain.is_enabled ? 'active' : 'inactive'} />
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/domains')}>返回列表</Button>
        {#if canUpdate('domain')}
        <Button onclick={() => goto(`/domains/${domain?.id}/edit`)}>编辑</Button>
      {/if}
        {#if canDelete('domain')}
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
      {/if}
      </div>
    </div>

    <div class="grid gap-6 lg:grid-cols-2">
      <Card.Root>
        <Card.Header>
          <Card.Title>基本信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">域名</dt>
              <dd class="font-mono">{domain.domain_name}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">注册日期</dt>
              <dd>{formatDate(domain.registered_date)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">到期日期</dt>
              <dd>{formatDate(domain.expiry_date)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">续费金额</dt>
              <dd>{domain.renewal_amount ? `${domain.renewal_amount} ${domain.renewal_currency || ''}` : '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">注册商</dt>
              <dd>
                {#if domain.provider_id}
                  {#if providerMap[domain.provider_id]}
                    <a href="/providers/{domain.provider_id}" class="text-primary hover:underline">
                      <span class={getResourceStatusClass(providerStatusMap[domain.provider_id], 'provider')}>
                        {formatResourceWithStatus(providerMap[domain.provider_id], providerStatusMap[domain.provider_id], 'provider')}
                      </span>
                    </a>
                  {:else}
                    <span class="text-muted-foreground italic">已删除</span>
                  {/if}
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">启用状态</dt>
              <dd>{domain.is_enabled ? '是' : '否'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>

      <Card.Root>
        <Card.Header>
          <Card.Title>联系信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">平台电话</dt>
              <dd>{domain.platform_phone || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">域名邮箱</dt>
              <dd>{domain.domain_email || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">隐私状态</dt>
              <dd>{getOptionLabel($domainPrivacyStatusOptions, domain.privacy_status)}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>
    </div>

    <Card.Root>
      <Card.Header>
        <Card.Title>其他</Card.Title>
      </Card.Header>
      <Card.Content>
        <dl class="grid gap-3 text-sm">
          <div class="flex justify-between">
            <dt class="text-muted-foreground">备注</dt>
            <dd>{domain.remarks || '-'}</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-muted-foreground">创建时间</dt>
            <dd>{formatDate(domain.created_at)}</dd>
          </div>
          <div class="flex justify-between">
            <dt class="text-muted-foreground">更新时间</dt>
            <dd>{formatDate(domain.updated_at)}</dd>
          </div>
        </dl>
      </Card.Content>
    </Card.Root>
    <AttachmentSection targetType="domain" targetId={domain.id} />
  {/if}

    <ConfirmDialog
      bind:open={confirmOpen}
      title="确认删除"
      description={`确定要删除域名「${domain?.domain_name}」吗？此操作可在回收站恢复。`}
      confirmLabel="删除"
      onConfirm={doDelete}
    />
</div>
