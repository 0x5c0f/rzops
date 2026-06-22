<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { credentialsApi } from '$lib/api/credentials';
  import type { CredentialResponse, UpdateCredentialRequest } from '$lib/types/credential';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { onMount } from 'svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { credentialTypeOptions, commonStatusOptions } from '$lib/utils/enum-options';

  let credential = $state<CredentialResponse | null>(null);
  let loading = $state(true);
  let saving = $state(false);
  let form = $state<UpdateCredentialRequest>({});

  onMount(async () => {
    try {
      credential = await credentialsApi.getById($page.params.id ?? "");
      form = {
        name: credential.name,
        credential_type: credential.credential_type,
        username: credential.username ?? undefined,
        secret_ref: credential.secret_ref ?? undefined,
        remarks: credential.remarks ?? undefined,
        status: credential.status,
      };
    } catch (err) {
      console.error('Failed to load credential:', err);
      goto('/credentials');
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    if (!credential) return;
    saving = true;
    try {
      await credentialsApi.update(credential.id, form);
      goto('/credentials');
    } catch (err) {
      console.error('Failed to save credential:', err);
    } finally {
      saving = false;
    }
  }

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

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '凭据', href: '/credentials' },
    { label: credential?.name || '详情' }
  ]} />

  {#if loading}
    <div class="text-muted-foreground">加载中...</div>
  {:else if credential}
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-2">
        <h1 class="text-2xl font-semibold">{credential.name}</h1>
        <StatusBadge status={credential.status} />
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
        <FormSelect label="类型" bind:value={form.credential_type} options={credentialTypeOptions} />
        <div class="space-y-2">
          <Label for="username">用户名</Label>
          <Input id="username" bind:value={form.username} />
        </div>
        <div class="space-y-2">
          <Label for="secret_ref">密钥引用</Label>
          <Input id="secret_ref" bind:value={form.secret_ref} />
        </div>
        <FormSelect label="状态" bind:value={form.status} options={commonStatusOptions} />
        <div class="space-y-2 md:col-span-2 lg:col-span-3">
          <Label for="remarks">备注</Label>
          <Input id="remarks" bind:value={form.remarks} />
        </div>
      </Card.Content>
    </Card.Root>
  {/if}
</div>
