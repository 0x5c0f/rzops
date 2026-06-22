<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { serverPortsApi } from '$lib/api/server-ports';
  import type { ServerPortResponse, UpdateServerPortRequest } from '$lib/types/server_port';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';

  let serverPort = $state<ServerPortResponse | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let form = $state<UpdateServerPortRequest>({});

  onMount(async () => {
    try {
      serverPort = await serverPortsApi.getById($page.params.id ?? "");
      form = {
        server_id: serverPort.server_id,
        protocol: serverPort.protocol,
        port: serverPort.port,
        service_name: serverPort.service_name ?? undefined,
        access_scope: serverPort.access_scope ?? undefined,
        is_enabled: serverPort.is_enabled ?? undefined,
        description: serverPort.description ?? undefined,
      };
    } catch (err) {
      console.error('Failed to load server port:', err);
      goto('/server-ports');
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    if (!serverPort) return;
    saving = true;
    try {
      await serverPortsApi.update(serverPort.id, form);
      goto('/server-ports');
    } catch (err) {
      console.error('Failed to save server port:', err);
    } finally {
      saving = false;
    }
  }

  async function handleDelete() {
    if (!serverPort) return;
    if (!confirm(`确定要删除端口 "${serverPort.port}" 吗？`)) return;
    try {
      await serverPortsApi.delete(serverPort.id);
      goto('/server-ports');
    } catch (err) {
      console.error('Failed to delete server port:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '服务器端口', href: '/server-ports' },
    { label: serverPort ? `${serverPort.protocol}:${serverPort.port}` : '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if serverPort}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{serverPort.protocol}:{serverPort.port}</h1>
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
          <Label for="server_id">服务器ID</Label>
          <Input id="server_id" bind:value={form.server_id} />
        </div>
        <div class="space-y-2">
          <Label for="protocol">协议</Label>
          <Input id="protocol" bind:value={form.protocol} placeholder="TCP / UDP / ..." />
        </div>
        <div class="space-y-2">
          <Label for="port">端口</Label>
          <Input id="port" type="number" bind:value={form.port} />
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
  {/if}
</div>
