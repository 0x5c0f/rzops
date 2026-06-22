<script lang="ts">
  import { goto } from '$app/navigation';
  import { domainsApi } from '$lib/api/domains';
  import type { CreateDomainRequest } from '$lib/types/domain';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';

  let saving = $state(false);
  let form = $state<CreateDomainRequest>({
    domain_name: '',
    is_enabled: true,
    business_unit_id: '',
    company_id: '',
    renewal_amount: '',
    renewal_currency: '',
    account_credential_id: '',
    platform_phone: '',
    domain_email: '',
    privacy_status: '',
  });

  async function handleSave() {
    saving = true;
    try {
      await domainsApi.create(form);
      goto('/domains');
    } catch (err) {
      console.error('Failed to create domain:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '域名', href: '/domains' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建域名</h1>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => goto('/domains')}>取消</Button>
      <Button onclick={handleSave} disabled={saving}>
        {saving ? '创建中...' : '创建'}
      </Button>
    </div>
  </div>

  <Card.Root>
    <Card.Header>
      <Card.Title>基本信息</Card.Title>
    </Card.Header>
    <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <div class="space-y-2">
        <Label for="domain_name">域名 *</Label>
        <Input id="domain_name" bind:value={form.domain_name} required />
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
</div>
