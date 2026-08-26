<script lang="ts">
  import type { CreateCredentialRequest } from '$lib/types/credential';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Card from '$lib/ui/card';
  import FormSelect from '$lib/components/shared/FormSelect.svelte';
  import TextArea from '$lib/components/shared/TextArea.svelte';
  import { credentialTypeOptions, commonStatusOptions } from '$lib/utils/enum-options';

  let {
    initial = {} as CreateCredentialRequest,
    submitLabel = '保存',
    onSubmit,
  }: {
    initial?: CreateCredentialRequest;
    submitLabel?: string;
    onSubmit: (data: CreateCredentialRequest) => Promise<void>;
  } = $props();

  let saving = $state(false);

  let form = $state<CreateCredentialRequest>(createInitial(initial));

  function createInitial(initial?: CreateCredentialRequest): CreateCredentialRequest {
    return {
      name: '',
      credential_type: '',
      username: '',
      secret_ref: '',
      status: 'active',
      ...structuredClone(initial ?? {}),
    };
  }

  async function handleSave() {
    saving = true;
    try {
      await onSubmit(form);
    } catch (err) {
      console.error('Failed to save credential:', err);
    } finally {
      saving = false;
    }
  }
</script>

<div class="space-y-4">
  <Card.Root>
    <Card.Header>
      <Card.Title>基本信息</Card.Title>
    </Card.Header>
    <Card.Content class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <div class="space-y-2">
        <Label for="name">名称 *</Label>
        <Input id="name" bind:value={form.name} required />
      </div>

      <FormSelect
        label="类型 *"
        bind:value={form.credential_type}
        options={credentialTypeOptions}
        required
      />

      <FormSelect
        label="状态"
        bind:value={form.status}
        options={commonStatusOptions}
      />

      <div class="space-y-2">
        <Label for="username">用户名</Label>
        <Input id="username" bind:value={form.username} />
      </div>

      <div class="space-y-2">
        <Label for="secret_ref">密钥引用</Label>
        <Input id="secret_ref" bind:value={form.secret_ref} />
      </div>

      <div class="space-y-2 md:col-span-2 lg:col-span-3">
        <Label for="remarks">备注</Label>
        <TextArea id="remarks" bind:value={form.remarks} rows={3} />
      </div>
    </Card.Content>
  </Card.Root>

  <div class="flex justify-end gap-2 pb-4">
    <Button variant="outline" onclick={() => history.back()}>取消</Button>
    <Button onclick={handleSave} disabled={saving}>
      {saving ? '保存中...' : submitLabel}
    </Button>
  </div>
</div>
