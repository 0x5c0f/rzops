<script lang="ts">
  import { goto } from '$app/navigation';
  import { credentialsApi } from '$lib/api/credentials';
  import type { CreateCredentialRequest } from '$lib/types/credential';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import { credentialTypeOptions, commonStatusOptions } from '$lib/utils/enum-options';

  let saving = $state(false);
  let form = $state<CreateCredentialRequest>({
    name: '',
    credential_type: '',
    username: '',
    secret_ref: '',
    remarks: '',
    status: 'active',
  });

  async function handleSave() {
    saving = true;
    try {
      await credentialsApi.create(form);
      goto('/credentials');
    } catch (err) {
      console.error('Failed to create credential:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[
    { label: '凭据', href: '/credentials' },
    { label: '新建' }
  ]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">新建凭据</h1>
    <div class="flex gap-2">
      <Button variant="outline" onclick={() => goto('/credentials')}>取消</Button>
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
        <Label for="name">名称 *</Label>
        <Input id="name" bind:value={form.name} required />
      </div>
      <FormSelect label="类型 *" bind:value={form.credential_type} options={credentialTypeOptions} required />
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
</div>
