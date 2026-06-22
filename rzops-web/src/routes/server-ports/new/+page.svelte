<script lang="ts">
  import { goto } from '$app/navigation';
  import { serverPortsApi } from '$lib/api/server-ports';
  import type { CreateServerPortRequest } from '$lib/types/server_port';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';

  let saving = $state(false);
  let form = $state<CreateServerPortRequest>({
    server_id: '',
    protocol: '',
    port: 0,
    service_name: '',
    access_scope: '',
    description: '',
  });

  async function handleSave() {
    saving = true;
    try {
      await serverPortsApi.create(form);
      goto('/server-ports');
    } catch (err) {
      console.error('Failed to create server port:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '服务器端口', href: '/server-ports' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建服务器端口</h1>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => goto('/server-ports')}>取消</Button>
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
        <Label for="server_id">服务器ID *</Label>
        <Input id="server_id" bind:value={form.server_id} required />
      </div>
      <div class="space-y-2">
        <Label for="protocol">协议 *</Label>
        <Input id="protocol" bind:value={form.protocol} placeholder="TCP / UDP / ..." required />
      </div>
      <div class="space-y-2">
        <Label for="port">端口 *</Label>
        <Input id="port" type="number" bind:value={form.port} required />
      </div>
      <div class="space-y-2">
        <Label for="service_name">服务名称</Label>
        <Input id="service_name" bind:value={form.service_name} />
      </div>
      <div class="space-y-2">
        <Label for="access_scope">访问范围</Label>
        <Input id="access_scope" bind:value={form.access_scope} placeholder="公网 / 内网 / ..." />
      </div>
      <div class="space-y-2">
        <Label for="description">描述</Label>
        <Input id="description" bind:value={form.description} />
      </div>
    </Card.Content>
  </Card.Root>
</div>
