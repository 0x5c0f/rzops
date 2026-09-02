<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import { backupPlansApi } from '$lib/api/backup-plans';
  import { monitorTargetsApi } from '$lib/api/monitor-targets';
  import type { CreateOpsSiteRequest, OpsSiteResponse } from '$lib/types/ops_site';
  import type { BackupPlanResponse } from '$lib/types/backup_plan';
  import type { MonitorTargetResponse } from '$lib/types/monitor_target';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import OpsSiteForm from '$lib/components/forms/OpsSiteForm.svelte';
  import SiteRelationsSection from '$lib/components/shared/SiteRelationsSection.svelte';
  import { onMount } from 'svelte';

  let site = $state<OpsSiteResponse | null>(null);
  let backupPlans = $state<BackupPlanResponse[]>([]);
  let monitorTargets = $state<MonitorTargetResponse[]>([]);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/ops-sites'); return; }
    try {
      const [siteData, bpData, mtData] = await Promise.all([
        opsSitesApi.getById(id),
        backupPlansApi.list({ target_type: 'site', target_id: id, per_page: 100 }),
        monitorTargetsApi.list({ target_type: 'site', target_id: id, per_page: 100 }),
      ]);
      site = siteData;
      backupPlans = bpData.data;
      monitorTargets = mtData.data;
    } catch (err) {
      console.error('Failed to load ops site:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  function toForm(s: OpsSiteResponse): CreateOpsSiteRequest {
    return {
      name: s.name,
      url: s.url ?? '',
      service_target: s.service_target ?? '',
      importance: s.importance ?? '',
      purpose: s.purpose ?? '',
      language_runtime: s.language_runtime ?? '',
      web_framework: s.web_framework ?? '',
      code_repo_type: s.code_repo_type ?? '',
      code_repo_url: s.code_repo_url ?? '',
      function_summary: s.function_summary ?? '',
      remarks: s.remarks ?? '',
      status: s.status ?? 'active',
      is_test_site: s.is_test_site ?? false,
    };
  }

  function toBackupDraft(b: BackupPlanResponse) {
    return {
      id: b.id,
      name: b.name,
      schedule: b.schedule ?? '',
      retention_days: b.retention_days != null ? String(b.retention_days) : '',
      status: b.status,
    };
  }

  function toMonitorDraft(m: MonitorTargetResponse) {
    return {
      id: m.id,
      name: m.name,
      monitor_type: m.monitor_type ?? '',
      endpoint: m.endpoint ?? '',
      interval_seconds: m.interval_seconds != null ? String(m.interval_seconds) : '',
      status: m.status,
    };
  }

  async function handleUpdate(data: CreateOpsSiteRequest) {
    const id = $page.params.id;
    if (!id) return;
    await opsSitesApi.update(id, data);
    goto(`/ops-sites/${id}`);
    return id;
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '站点', href: '/ops-sites' },
    { label: site?.name || '详情', href: site ? `/ops-sites/${site.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑站点</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !site}
    <p class="text-sm text-muted-foreground">加载失败，站点可能不存在。</p>
  {:else}
    <OpsSiteForm
      initial={toForm(site)}
      initialBackupPlans={backupPlans.map(toBackupDraft)}
      initialMonitorTargets={monitorTargets.map(toMonitorDraft)}
      entityId={site.id}
      submitLabel="保存"
      onSubmit={handleUpdate}
    />
    <SiteRelationsSection siteId={site.id} />
  {/if}
</div>
