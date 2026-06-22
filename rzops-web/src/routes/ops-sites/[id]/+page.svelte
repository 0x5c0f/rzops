<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { opsSitesApi } from '$lib/api/ops-sites';
  import type { OpsSiteResponse, UpdateOpsSiteRequest } from '$lib/types/ops_site';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { siteStatusOptions, importanceOptions, serviceTargetOptions } from '$lib/utils/enum-options';
  import { onMount } from 'svelte';

  let site = $state<OpsSiteResponse | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let form = $state<UpdateOpsSiteRequest>({});

  onMount(async () => {
    try {
      site = await opsSitesApi.getById($page.params.id ?? "");
      form = {
        name: site.name,
        url: site.url ?? undefined,
        service_target: site.service_target ?? undefined,
        importance: site.importance ?? undefined,
        purpose: site.purpose ?? undefined,
        language_runtime: site.language_runtime ?? undefined,
        web_framework: site.web_framework ?? undefined,
        code_repo_type: site.code_repo_type ?? undefined,
        code_repo_url: site.code_repo_url ?? undefined,
        function_summary: site.function_summary ?? undefined,
        remarks: site.remarks ?? undefined,
        status: site.status,
      };
    } catch (err) {
      console.error('Failed to load ops-site:', err);
      goto('/ops-sites');
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    if (!site) return;
    saving = true;
    try {
      await opsSitesApi.update(site.id, form);
      goto('/ops-sites');
    } catch (err) {
      console.error('Failed to save ops-site:', err);
    } finally {
      saving = false;
    }
  }

  async function handleDelete() {
    if (!site) return;
    if (!confirm(`确定要删除站点 "${site.name}" 吗？`)) return;
    try {
      await opsSitesApi.delete(site.id);
      goto('/ops-sites');
    } catch (err) {
      console.error('Failed to delete ops-site:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '站点', href: '/ops-sites' },
    { label: site?.name || '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if site}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{site.name}</h1>
        <StatusBadge status={site.status} />
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
          <Label for="name">名称</Label>
          <Input id="name" bind:value={form.name} />
        </div>
        <FormSelect label="状态" bind:value={form.status} options={siteStatusOptions} required />
        <div class="space-y-2">
          <Label for="url">URL</Label>
          <Input id="url" bind:value={form.url} placeholder="https://..." />
        </div>
        <FormSelect label="服务目标" bind:value={form.service_target} options={serviceTargetOptions} />
        <FormSelect label="重要性" bind:value={form.importance} options={importanceOptions} />
        <div class="space-y-2">
          <Label for="purpose">用途</Label>
          <Input id="purpose" bind:value={form.purpose} />
        </div>
        <div class="space-y-2">
          <Label for="language_runtime">语言/运行时</Label>
          <Input id="language_runtime" bind:value={form.language_runtime} />
        </div>
        <div class="space-y-2">
          <Label for="web_framework">Web框架</Label>
          <Input id="web_framework" bind:value={form.web_framework} />
        </div>
        <div class="space-y-2">
          <Label for="code_repo_type">代码仓库类型</Label>
          <Input id="code_repo_type" bind:value={form.code_repo_type} />
        </div>
        <div class="space-y-2">
          <Label for="code_repo_url">代码仓库地址</Label>
          <Input id="code_repo_url" bind:value={form.code_repo_url} />
        </div>
        <div class="space-y-2 md:col-span-2 lg:col-span-3">
          <Label for="function_summary">功能概述</Label>
          <Input id="function_summary" bind:value={form.function_summary} />
        </div>
        <div class="space-y-2 md:col-span-2 lg:col-span-3">
          <Label for="remarks">备注</Label>
          <Input id="remarks" bind:value={form.remarks} />
        </div>
      </Card.Content>
    </Card.Root>
  {/if}
</div>
