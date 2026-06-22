<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { serverIpsApi } from '$lib/api/server-ips';
  import type { ServerIpResponse, UpdateServerIpRequest } from '$lib/types/server_ip';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { onMount } from 'svelte';

  let serverIp = $state<ServerIpResponse | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let form = $state<UpdateServerIpRequest>({});

  onMount(async () => {
    try {
      serverIp = await serverIpsApi.getById($page.params.id ?? "");
      form = {
        server_id: serverIp.server_id,
        ip_address: serverIp.ip_address,
        ip_type: serverIp.ip_type ?? undefined,
        is_primary: serverIp.is_primary ?? undefined,
        isp_provider_id: serverIp.isp_provider_id ?? undefined,
        description: serverIp.description ?? undefined,
        status: serverIp.status,
      };
    } catch (err) {
      console.error('Failed to load server IP:', err);
      goto('/server-ips');
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    if (!serverIp) return;
    saving = true;
    try {
      await serverIpsApi.update(serverIp.id, form);
      goto('/server-ips');
    } catch (err) {
      console.error('Failed to save server IP:', err);
    } finally {
      saving = false;
    }
  }

  async function handleDelete() {
    if (!serverIp) return;
    if (!confirm(`确定要删除服务器IP "${serverIp.ip_address}" 吗？`)) return;
    try {
      await serverIpsApi.delete(serverIp.id);
      goto('/server-ips');
    } catch (err) {
      console.error('Failed to delete server IP:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '服务器IP', href: '/server-ips' },
    { label: serverIp?.ip_address || '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if serverIp}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{serverIp.ip_address}</h1>
        <StatusBadge status={serverIp.status} />
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
          <Label for="ip_address">IP地址</Label>
          <Input id="ip_address" bind:value={form.ip_address} />
        </div>
        <div class="space-y-2">
          <Label for="ip_type">IP类型</Label>
          <Input id="ip_type" bind:value={form.ip_type} placeholder="公网 / 内网 / ..." />
        </div>
        <div class="space-y-2">
          <Label for="server_id">服务器ID</Label>
          <Input id="server_id" bind:value={form.server_id} />
        </div>
        <div class="space-y-2">
          <Label for="isp_provider_id">ISP供应商ID</Label>
          <Input id="isp_provider_id" bind:value={form.isp_provider_id} />
        </div>
        <div class="space-y-2">
          <Label for="description">描述</Label>
          <Input id="description" bind:value={form.description} />
        </div>
      </Card.Content>
    </Card.Root>
  {/if}
</div>
