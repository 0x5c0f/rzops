<script lang="ts">
  import { goto } from '$app/navigation';
  import { serverIpsApi } from '$lib/api/server-ips';
  import type { CreateServerIpRequest } from '$lib/types/server_ip';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';

  let saving = $state(false);
  let form = $state<CreateServerIpRequest>({
    server_id: '',
    ip_address: '',
    ip_type: '',
    status: 'active',
    isp_provider_id: '',
    description: '',
  });

  async function handleSave() {
    saving = true;
    try {
      await serverIpsApi.create(form);
      goto('/server-ips');
    } catch (err) {
      console.error('Failed to create server IP:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '服务器IP', href: '/server-ips' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建服务器IP</h1>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => goto('/server-ips')}>取消</Button>
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
        <Label for="ip_address">IP地址 *</Label>
        <Input id="ip_address" bind:value={form.ip_address} required />
      </div>
      <div class="space-y-2">
        <Label for="ip_type">IP类型</Label>
        <Input id="ip_type" bind:value={form.ip_type} placeholder="公网 / 内网 / ..." />
      </div>
      <div class="space-y-2">
        <Label for="server_id">服务器ID *</Label>
        <Input id="server_id" bind:value={form.server_id} required />
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
</div>
