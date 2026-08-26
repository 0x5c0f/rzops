<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { credentialsApi } from '$lib/api/credentials';
  import type { CredentialResponse } from '$lib/types/credential';
  import { Button } from '$lib/ui/button';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { credentialTypeOptions, getOptionLabel } from '$lib/utils/enum-options';
  import { onMount } from 'svelte';

  let credential = $state<CredentialResponse | null>(null);
  let loading = $state(true);

  onMount(async () => {
    const id = $page.params.id;
    if (!id) { goto('/credentials'); return; }

    try {
      credential = await credentialsApi.getById(id);
    } catch (err) {
      console.error('Failed to load credential:', err);
      goto('/credentials');
    } finally {
      loading = false;
    }
  });

  async function handleDelete() {
    if (!credential) return;
    if (!confirm(`确定要删除凭据 "${credential.name}" 吗？`)) return;
    try {
      await credentialsApi.delete(credential.id);
      goto('/credentials');
    } catch (err) {
      console.error('Failed to delete credential:', err);
    }
  }
</script>

<div class="space-y-6">
  <Breadcrumb items={[
    { label: '凭据', href: '/credentials' },
    { label: credential?.name || '详情' }
  ]} />

  {#if loading}
    <div class="flex items-center justify-center py-12">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
    </div>
  {:else if credential}
    <!-- 页面标题 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <h1 class="text-2xl font-semibold">{credential.name}</h1>
        <StatusBadge status={credential.status} />
      </div>
      <div class="flex gap-2">
        <Button variant="outline" onclick={() => goto('/credentials')}>返回列表</Button>
        <Button onclick={() => goto(`/credentials/${credential?.id}/edit`)}>编辑</Button>
        <Button variant="destructive" onclick={handleDelete}>删除</Button>
      </div>
    </div>

    <div class="grid gap-6 lg:grid-cols-2">
      <!-- 基本信息 -->
      <Card.Root>
        <Card.Header>
          <Card.Title>基本信息</Card.Title>
        </Card.Header>
        <Card.Content>
          <dl class="grid gap-3 text-sm">
            <div class="flex justify-between">
              <dt class="text-muted-foreground">名称</dt>
              <dd>{credential.name}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">类型</dt>
              <dd>{getOptionLabel($credentialTypeOptions, credential.credential_type)}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">用户名</dt>
              <dd class="font-mono">{credential.username || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">密钥引用</dt>
              <dd class="font-mono">{credential.secret_ref || '-'}</dd>
            </div>
            <div class="flex justify-between">
              <dt class="text-muted-foreground">所有者ID</dt>
              <dd class="font-mono">{credential.owner_id || '-'}</dd>
            </div>
          </dl>
        </Card.Content>
      </Card.Root>
    </div>
  {/if}
</div>
