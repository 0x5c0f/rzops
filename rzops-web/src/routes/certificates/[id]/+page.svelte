<script lang="ts">
  import AttachmentSection from '$lib/components/shared/AttachmentSection.svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { certificatesApi } from '$lib/api/certificates';
  import { certificateDomainsApi } from '$lib/api/certificate-domains';
  import type { CertificateResponse } from '$lib/types/certificate';
  import type { CertificateDomainResponse } from '$lib/types/certificate_domain';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import * as Table from '$lib/ui/table';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { certificateStatusOptions, certificateTypeOptions, getOptionLabel } from '$lib/utils/enum-options';
  import { getProviderOptions, getCredentialOptions, getDomainOptions } from '$lib/utils/entity-options';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';

  let certificate = $state<CertificateResponse | null>(null);
  let domains = $state<CertificateDomainResponse[]>([]);
  let loading = $state(true);
  let providerMap = $state<Record<string, string>>({});
  let credentialMap = $state<Record<string, string>>({});
  let domainMap = $state<Record<string, string>>({});

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/certificates'); return; }
    try {
      const [cert, providers, credentials, domainList, domainsData] = await Promise.all([
        certificatesApi.getById(id),
        getProviderOptions(),
        getCredentialOptions(),
        getDomainOptions(),
        certificateDomainsApi.list({ certificate_id: id, per_page: 100 }),
      ]);
      certificate = cert;
      providerMap = Object.fromEntries(providers.map(o => [o.value, o.label]));
      credentialMap = Object.fromEntries(credentials.map(o => [o.value, o.label]));
      domainMap = Object.fromEntries(domainList.map(o => [o.value, o.label]));
      domains = domainsData.data;
    } catch (err) {
      console.error('Failed to load certificate:', err);
      goto('/certificates');
    } finally {
      loading = false;
    }
  });

  async function handleDelete() {
    if (!certificate) return;
    if (!confirm(`确定要删除证书 "${certificate.name}" 吗？`)) return;
    try {
      await certificatesApi.delete(certificate.id);
      goto('/certificates');
    } catch (err) {
      console.error('Failed to delete certificate:', err);
    }
  }
</script>

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '证书', href: '/certificates' },
    { label: certificate?.name || '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if certificate}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold">{certificate.name}</h1>
        <StatusBadge status={certificate.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/certificates')}>返回列表</Button>
        <Button onclick={() => goto(`/certificates/${certificate?.id}/edit`)}>编辑</Button>
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
              <dt class="text-muted-foreground">名称</dt>
              <dd>{certificate.name}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">证书类型</dt>
              <dd>{getOptionLabel($certificateTypeOptions, certificate.certificate_type)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">状态</dt>
              <dd>{getOptionLabel($certificateStatusOptions, certificate.status)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">供应商</dt>
              <dd>
                {#if certificate.provider_id}
                  <a href="/providers/{certificate.provider_id}" class="text-primary hover:underline">
                    {providerMap[certificate.provider_id] || certificate.provider_id}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">起始日期</dt>
              <dd>{formatDate(certificate.lease_start_date)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">到期日期</dt>
              <dd>{formatDate(certificate.lease_end_date)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">密钥凭证</dt>
              <dd>
                {#if certificate.private_key_credential_id}
                  <a href="/credentials/{certificate.private_key_credential_id}" class="text-primary hover:underline">
                    {credentialMap[certificate.private_key_credential_id] || certificate.private_key_credential_id}
                  </a>
                {:else}
                  -
                {/if}
              </dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">备注</dt>
              <dd>{certificate.remarks || '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>
    </div>

    <!-- 绑定域名 -->
    <Card.Root>
      <Card.Header>
        <Card.Title>绑定域名 ({domains.length})</Card.Title>
      </Card.Header>
      <Card.Content>
        {#if domains.length === 0}
          <p class="text-sm text-muted-foreground">暂无绑定域名</p>
        {:else}
          <Table.Root>
            <Table.Header>
              <Table.Row>
                <Table.Head>域名 / 模式</Table.Head>
                <Table.Head>关联域名</Table.Head>
                <Table.Head>主域名</Table.Head>
              </Table.Row>
            </Table.Header>
            <Table.Body>
              {#each domains as d}
                <Table.Row>
                  <Table.Cell class="font-mono">{d.domain_pattern}</Table.Cell>
                  <Table.Cell>
                    {#if d.domain_id}
                      <a href="/domains/{d.domain_id}" class="text-primary hover:underline">
                        {domainMap[d.domain_id] || d.domain_id}
                      </a>
                    {:else}
                      -
                    {/if}
                  </Table.Cell>
                  <Table.Cell>{d.is_primary ? '是' : '-'}</Table.Cell>
                </Table.Row>
              {/each}
            </Table.Body>
          </Table.Root>
        {/if}
      </Card.Content>
    </Card.Root>
    <AttachmentSection targetType="certificate" targetId={certificate.id} />
  {/if}
</div>
