<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { serversApi } from '$lib/api/servers';
  import { serverIpsApi } from '$lib/api/server-ips';
  import { serverPortsApi } from '$lib/api/server-ports';
  import { databaseInstancesApi } from '$lib/api/database-instances';
  import type { CreateServerRequest, ServerResponse } from '$lib/types/server';
  import type { ServerIpResponse } from '$lib/types/server_ip';
  import type { ServerPortResponse } from '$lib/types/server_port';
  import type { DatabaseInstanceResponse } from '$lib/types/database_instance';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import ServerForm from '$lib/components/forms/ServerForm.svelte';
  import { onMount } from 'svelte';

  let server = $state<ServerResponse | null>(null);
  let ips = $state<ServerIpResponse[]>([]);
  let ports = $state<ServerPortResponse[]>([]);
  let dbInstances = $state<DatabaseInstanceResponse[]>([]);
  let loading = $state(true);
  let loadError = $state(false);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/servers'); return; }
    try {
      const [serverData, ipData, portData, dbData] = await Promise.all([
        serversApi.getById(id),
        serverIpsApi.list({ server_id: id, per_page: 100 }),
        serverPortsApi.list({ server_id: id, per_page: 100 }),
        databaseInstancesApi.list({ server_id: id, per_page: 100 }),
      ]);
      server = serverData;
      ips = ipData.data;
      ports = portData.data;
      dbInstances = dbData.data;
    } catch (err) {
      console.error('Failed to load server:', err);
      loadError = true;
    } finally {
      loading = false;
    }
  });

  // 详情 -> 表单数据：null 转空字符串，确保编辑时能清空字段
  function toForm(s: ServerResponse): CreateServerRequest {
    return {
      name: s.name,
      asset_code: s.asset_code ?? '',
      primary_ip: s.primary_ip ?? '',
      location: s.location ?? '',
      isp_provider_id: s.isp_provider_id ?? '',
      data_center_id: s.data_center_id ?? '',
      hosting_type: s.hosting_type ?? '',
      is_dual_line: s.is_dual_line ?? false,
      lease_start_date: s.lease_start_date ?? '',
      lease_end_date: s.lease_end_date ?? '',
      price: s.price ?? '',
      price_currency: s.price_currency ?? 'CNY',
      server_type: s.server_type ?? '',
      role_tags: [...(s.role_tags ?? [])],
      is_database_server: s.is_database_server ?? false,
      cpu: s.cpu ?? '',
      memory_gb: s.memory_gb ?? undefined,
      is_raid: s.is_raid ?? false,
      raid_level: s.raid_level ?? '',
      disk_layout: s.disk_layout ?? '',
      hardware_config: s.hardware_config ?? '',
      architecture: s.architecture ?? '',
      maintainer_id: s.maintainer_id ?? '',
      brand: s.brand ?? '',
      warranty_info: s.warranty_info ?? '',
      operating_system: s.operating_system ?? '',
      web_server_type: [...(s.web_server_type ?? [])],
      server_provider_id: s.server_provider_id ?? '',
      software_provider_id: s.software_provider_id ?? '',
      status: s.status ?? 'active',
      offline_time: s.offline_time ?? '',
      offline_reason: s.offline_reason ?? '',
      remarks: s.remarks ?? '',
    };
  }

  function toIpDraft(ip: ServerIpResponse) {
    return {
      id: ip.id,
      ip_address: ip.ip_address,
      nic_name: ip.nic_name ?? '',
      ip_type: ip.ip_type ?? '',
      is_primary: ip.is_primary,
      isp_provider_id: ip.isp_provider_id ?? '',
      description: ip.description ?? '',
    };
  }

  function toPortDraft(p: ServerPortResponse) {
    return {
      id: p.id,
      protocol: p.protocol,
      port: String(p.port),
      service_name: p.service_name,
      access_scope: p.access_scope ?? '',
      is_enabled: p.is_enabled,
      description: p.description ?? '',
    };
  }

  function toDbDraft(d: DatabaseInstanceResponse) {
    return {
      id: d.id,
      name: d.name,
      db_type: d.db_type,
      port: d.port != null ? String(d.port) : '',
      instance_name: d.instance_name ?? '',
      importance: d.importance ?? '',
      description: d.description ?? '',
    };
  }

  async function handleUpdate(data: CreateServerRequest) {
    const id = $page.params.id;
    if (!id) return;
    await serversApi.update(id, data);
    return id;
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '服务器', href: '/servers' },
    { label: server?.name || '详情', href: server ? `/servers/${server.id}` : undefined },
    { label: '编辑' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">编辑服务器</h1>
  </div>

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if loadError || !server}
    <p class="text-sm text-muted-foreground">加载失败，服务器可能不存在。</p>
  {:else}
    <ServerForm
      initial={toForm(server)}
      initialIps={ips.map(toIpDraft)}
      initialPorts={ports.map(toPortDraft)}
      initialDbInstances={dbInstances.map(toDbDraft)}
      entityId={server.id}
      submitLabel="保存"
      onSubmit={handleUpdate}
    />
  {/if}
</div>
