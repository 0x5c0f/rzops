<script lang="ts">
  import type { CreateCertificateRequest } from '$lib/types/certificate';
import AttachmentFormSection from '$lib/components/shared/AttachmentFormSection.svelte';
  import type { CertificateDomainResponse } from '$lib/types/certificate_domain';
  import { goto } from '$app/navigation';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import DateField from '$lib/components/shared/DateField.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { certificateStatusOptions, certificateTypeOptions } from '$lib/utils/enum-options';
  import { getProviderOptions, getDomainOptions } from '$lib/utils/entity-options';
  import { certificateDomainsApi } from '$lib/api/certificate-domains';
  import { onMount } from 'svelte';

  // 域名绑定草稿行（id 存在 = 已有记录，用于编辑增量同步）
  interface DomainDraft {
    id?: string;
    domain_pattern: string;
    domain_id: string;
    is_primary: boolean;
    // 记录该行最近一次自动带出的域名模式（用于判断是否可再次自动带出，避免覆盖用户手输）
    _lastAuto?: string;
  }

  let {
    initial = {} as CreateCertificateRequest,
    initialDomains = [] as DomainDraft[],
    entityId = '',
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateCertificateRequest;
    initialDomains?: DomainDraft[];
    submitLabel?: string;
    onSubmit: (data: CreateCertificateRequest) => Promise<string | void>;
  } = $props();

  let saving = $state(false);
  let attachmentRef = $state<{ uploadAll: (id: string) => Promise<void> } | null>(null);
  let providerOptions = $state<{ label: string; value: string }[]>([]);
  let domainOptions = $state<{ label: string; value: string }[]>([]);

  let form = $state<CreateCertificateRequest>(createInitial(initial));
  let domains = $state<DomainDraft[]>(initialDomains.length ? JSON.parse(JSON.stringify(initialDomains)) : []);

  function createInitial(initial?: CreateCertificateRequest): CreateCertificateRequest {
    return {
      name: '',
      certificate_type: '',
      status: 'active',
      ...JSON.parse(JSON.stringify(initial ?? {})),
    };
  }

  onMount(async () => {
    const [providers, domains] = await Promise.all([getProviderOptions(), getDomainOptions()]);
    providerOptions = providers;
    domainOptions = domains;
  });

  function emptyDomain(): DomainDraft {
    return { domain_pattern: '', domain_id: '', is_primary: false };
  }
  function addDomainRow() {
    domains = [...domains, emptyDomain()];
  }
  function removeDomainRow(index: number) {
    domains = domains.filter((_, i) => i !== index);
  }

  // 选择已录入域名后自动带出域名模式（避免重复手输；手动填写模式后不再覆盖）
  // 按"行"追踪：每行首次选择（或从自动带出值改为另一域名）时自动带出，用户手输后保持
  $effect(() => {
    for (const d of domains) {
      if (!d.domain_id) continue;
      const opt = domainOptions.find(o => o.value === d.domain_id);
      if (!opt) continue;
      // 仅当该行尚未自动带出过，或当前模式仍是上次自动带出的域名时，才更新
      const isAuto = !d._lastAuto || d.domain_pattern.trim() === d._lastAuto;
      if (isAuto && d.domain_pattern.trim() !== opt.label) {
        d.domain_pattern = opt.label;
        d._lastAuto = opt.label;
      }
    }
  });

  // 增量同步域名绑定：删除已移除的、更新有 id 的、新增无 id 的
  async function syncDomains(certificateId: string) {
    for (const db of initialDomains) {
      if (db.id && !domains.some(r => r.id === db.id)) {
        await certificateDomainsApi.delete(db.id);
      }
    }
    for (const row of domains) {
      const payload = {
        domain_pattern: row.domain_pattern.trim(),
        domain_id: row.domain_id || undefined,
        is_primary: row.is_primary,
      };
      if (row.id) {
        await certificateDomainsApi.update(row.id, payload);
      } else {
        await certificateDomainsApi.create({ certificate_id: certificateId, ...payload });
      }
    }
  }

  async function handleSave() {
    if (domains.some(d => !d.domain_pattern.trim())) {
      alert('域名绑定中"域名/模式"为必填，请填写完整或删除空行');
      return;
    }
    if (
      form.lease_start_date &&
      form.lease_end_date &&
      form.lease_end_date < form.lease_start_date
    ) {
      alert('到期日期不能早于起始日期');
      return;
    }
    saving = true;
    try {
      const certificateId = await onSubmit(form);
      if (certificateId) {
        await attachmentRef?.uploadAll(certificateId);
        await syncDomains(certificateId);
        goto(`/certificates/${certificateId}`);
      }
    } catch (err) {
      console.error('Failed to save certificate:', err);
      alert('保存失败，请重试');
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <div></div>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => history.back()}>取消</Button>
      <Button onclick={handleSave} disabled={saving}>
        {saving ? '保存中...' : submitLabel}
      </Button>
    </div>
  </div>

  <Card.Root>
    <Card.Header>
      <Card.Title>基本信息</Card.Title>
    </Card.Header>
    <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <div class="space-y-2">
        <Label for="name">名称 <span class="text-destructive">*</span></Label>
        <Input id="name" bind:value={form.name} required />
      </div>

      <FormSelect
        label="证书类型"
        bind:value={form.certificate_type}
        options={$certificateTypeOptions}
        placeholder="选择证书类型"
      />

      <FormSelect
        label="供应商"
        bind:value={form.provider_id}
        options={providerOptions}
        placeholder="选择供应商"
      />

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={$certificateStatusOptions}
      />

      <DateField
        id="lease_start_date"
        label="起始日期"
        bind:value={form.lease_start_date}
        max={form.lease_end_date || undefined}
      />

      <DateField
        id="lease_end_date"
        label="到期日期"
        bind:value={form.lease_end_date}
        min={form.lease_start_date || undefined}
      />

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="remarks">备注</Label>
        <TextArea id="remarks" bind:value={form.remarks} rows={3} />
      </div>
    </Card.Content>
  </Card.Root>

  <!-- 绑定域名 -->
  <Card.Root>
    <Card.Header>
      <Card.Title>绑定域名</Card.Title>
      <p class="text-sm text-muted-foreground">证书通常绑定已录入的域名，选择后自动带出域名；也支持填写通配符模式（如 *.example.com）</p>
    </Card.Header>
    <Card.Content class="space-y-3">
      {#each domains as domain, i}
        <div class="grid gap-3 rounded-lg border p-3 md:grid-cols-12">
          <div class="space-y-1 md:col-span-4">
            <FormSelect
              label="关联域名"
              bind:value={domain.domain_id}
              options={domainOptions}
              placeholder="选择已录入域名"
            />
          </div>
          <div class="space-y-1 md:col-span-4">
            <Label>域名 / 模式 <span class="text-destructive">*</span></Label>
            <Input bind:value={domain.domain_pattern} placeholder="*.example.com" />
          </div>
          <div class="flex items-end gap-2 md:col-span-3">
            <label class="flex items-center gap-2 pb-2 text-sm">
              <input type="checkbox" bind:checked={domain.is_primary} class="h-4 w-4" />
              主域名
            </label>
          </div>
          <div class="flex items-end justify-end md:col-span-1">
            <Button variant="ghost" size="sm" type="button" onclick={() => removeDomainRow(i)}>删除</Button>
          </div>
        </div>
      {/each}
      <Button variant="outline" size="sm" type="button" onclick={addDomainRow}>+ 添加域名</Button>
    </Card.Content>
  </Card.Root>

    <AttachmentFormSection bind:this={attachmentRef} targetType="certificate" targetId={entityId} />

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
