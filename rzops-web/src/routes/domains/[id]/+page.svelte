<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { domainsApi } from '$lib/api/domains';
  import type { DomainResponse, UpdateDomainRequest } from '$lib/types/domain';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { onMount } from 'svelte';

  let domain = $state<DomainResponse | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let form = $state<UpdateDomainRequest>({});

  onMount(async () => {
    try {
      domain = await domainsApi.getById($page.params.id ?? "");
      form = {
        domain_name: domain.domain_name,
        registrar: domain.registrar ?? undefined,
        registration_date: domain.registration_date ?? undefined,
        expiration_date: domain.expiration_date ?? undefined,
        dns_provider: domain.dns_provider ?? undefined,
        icp_filing_no: domain.icp_filing_no ?? undefined,
        icp_filing_status: domain.icp_filing_status ?? undefined,
        purpose: domain.purpose ?? undefined,
        owner_id: domain.owner_id ?? undefined,
        is_enabled: domain.is_enabled ?? undefined,
        business_unit_id: domain.business_unit_id ?? undefined,
        company_id: domain.company_id ?? undefined,
        renewal_amount: domain.renewal_amount ?? undefined,
        renewal_currency: domain.renewal_currency ?? undefined,
        account_credential_id: domain.account_credential_id ?? undefined,
        platform_phone: domain.platform_phone ?? undefined,
        domain_email: domain.domain_email ?? undefined,
        privacy_status: domain.privacy_status ?? undefined,
        remarks: domain.remarks ?? undefined,
      };
    } catch (err) {
      console.error('Failed to load domain:', err);
      goto('/domains');
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    if (!domain) return;
    saving = true;
    try {
      await domainsApi.update(domain.id, form);
      goto('/domains');
    } catch (err) {
      console.error('Failed to save domain:', err);
    } finally {
      saving = false;
    }
  }

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

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '域名', href: '/domains' },
    { label: domain?.domain_name || '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if domain}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{domain.domain_name}</h1>
        <StatusBadge status={domain.is_enabled ? 'active' : 'inactive'} />
      </div>
      <div class="flex gap-2">
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
        <Button onclick={handleSave} disabled={saving}>
          {saving ? '保存中...' : '保存'}
        </Button>
      </div>
    </div>

    <Card.Root>
      <Card.Header>
        <Card.Title>基本信息</Card.Title>
      </Card.Header>
      <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        <div class="space-y-2">
          <Label for="domain_name">域名</Label>
          <Input id="domain_name" bind:value={form.domain_name} />
        </div>
        <div class="space-y-2">
          <Label for="registrar">注册商</Label>
          <Input id="registrar" bind:value={form.registrar} />
        </div>
        <div class="space-y-2">
          <Label for="registration_date">注册日期</Label>
          <Input id="registration_date" type="date" bind:value={form.registration_date} />
        </div>
        <div class="space-y-2">
          <Label for="expiration_date">到期日期</Label>
          <Input id="expiration_date" type="date" bind:value={form.expiration_date} />
        </div>
        <div class="space-y-2">
          <Label for="dns_provider">DNS提供商</Label>
          <Input id="dns_provider" bind:value={form.dns_provider} />
        </div>
        <div class="space-y-2">
          <Label for="icp_filing_no">ICP备案号</Label>
          <Input id="icp_filing_no" bind:value={form.icp_filing_no} />
        </div>
        <div class="space-y-2">
          <Label for="icp_filing_status">ICP备案状态</Label>
          <Input id="icp_filing_status" bind:value={form.icp_filing_status} />
        </div>
        <div class="space-y-2">
          <Label for="purpose">用途</Label>
          <Input id="purpose" bind:value={form.purpose} />
        </div>
        <div class="space-y-2">
          <Label for="owner_id">所有者ID</Label>
          <Input id="owner_id" bind:value={form.owner_id} />
        </div>
        <div class="space-y-2">
          <Label for="is_enabled">启用状态</Label>
          <Input id="is_enabled" bind:value={form.is_enabled} placeholder="true / false" />
        </div>
        <div class="space-y-2">
          <Label for="business_unit_id">业务单元ID</Label>
          <Input id="business_unit_id" bind:value={form.business_unit_id} />
        </div>
        <div class="space-y-2">
          <Label for="company_id">公司ID</Label>
          <Input id="company_id" bind:value={form.company_id} />
        </div>
        <div class="space-y-2">
          <Label for="renewal_amount">续费金额</Label>
          <Input id="renewal_amount" bind:value={form.renewal_amount} />
        </div>
        <div class="space-y-2">
          <Label for="renewal_currency">续费币种</Label>
          <Input id="renewal_currency" bind:value={form.renewal_currency} placeholder="CNY / USD / ..." />
        </div>
        <div class="space-y-2">
          <Label for="account_credential_id">账户凭证ID</Label>
          <Input id="account_credential_id" bind:value={form.account_credential_id} />
        </div>
        <div class="space-y-2">
          <Label for="platform_phone">平台电话</Label>
          <Input id="platform_phone" bind:value={form.platform_phone} />
        </div>
        <div class="space-y-2">
          <Label for="domain_email">域名邮箱</Label>
          <Input id="domain_email" bind:value={form.domain_email} />
        </div>
        <div class="space-y-2">
          <Label for="privacy_status">隐私状态</Label>
          <Input id="privacy_status" bind:value={form.privacy_status} />
        </div>
        <div class="space-y-2 md:col-span-2 lg:col-span-3">
          <Label for="remarks">备注</Label>
          <Input id="remarks" bind:value={form.remarks} />
        </div>
      </Card.Content>
    </Card.Root>
  {/if}
</div>
