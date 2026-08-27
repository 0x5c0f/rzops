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
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';

  let domain = $state<DomainResponse | null>(null);
  let loading = $state(true);
  let providerMap = $state<Record<string, string>>({});

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/domains'); return; }
    try {
      const [d, provOptions] = await Promise.all([
        domainsApi.getById(id),
        getProviderOptions(),
      ]);
      domain = d;
      providerMap = Object.fromEntries(provOptions.map(o => [o.value, o.label]));
    } catch (err) {
      console.error('Failed to load domain:', err);
      goto('/domains');
    } finally {
      loading = false;
    }
  });

  async function handleDelete() {
    if (!domain) return;
    if (!confirm(`确定要删除域名 "${domain.domain_name}" 吗？`)) return;
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
        <Button onclick={() => goto(`/domains/${domain?.id}/edit`)}>编辑</Button>
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
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
                  <a href="/providers/{domain.provider_id}" class="text-primary hover:underline">
                    {providerMap[domain.provider_id] || domain.provider_id}
                  </a>
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
              <dd>{domain.privacy_status || '-'}</dd>
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
</div>
