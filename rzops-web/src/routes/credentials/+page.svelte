<script lang="ts">
  import { goto } from '$app/navigation';
  import { credentialsApi } from '$lib/api/credentials';
  import type { CredentialResponse, ListCredentialsQuery } from '$lib/types/credential';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';
  import { commonStatusOptions, credentialTypeOptions } from '$lib/utils/enum-options';

  let data = $state<CredentialResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListCredentialsQuery>({ limit: 20, offset: 0 });
  let offset = $derived(query.offset ?? 0);
  let limit = $derived(query.limit ?? 20);

  let statusMap = $derived(Object.fromEntries($commonStatusOptions.map(o => [o.value, o.label])));
  let credentialTypeMap = $derived(Object.fromEntries($credentialTypeOptions.map(o => [o.value, o.label])));

  const columns = [
    { key: 'name', label: '名称' },
    { key: 'credential_type', label: '类型', valueMap: credentialTypeMap },
    { key: 'username', label: '用户名' },
    { key: 'status', label: '状态', valueMap: statusMap },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string) },
  ];

  async function loadData() {
    loading = true;
    try {
      const res = await credentialsApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load credentials:', err);
    } finally {
      loading = false;
    }
  }

  onMount(loadData);

  function handleSearch(e: Event) {
    const input = e.target as HTMLInputElement;
    query = { ...query, q: input.value, offset: 0 };
    loadData();
  }

  function handlePageChange(newOffset: number) {
    query = { ...query, offset: newOffset };
    loadData();
  }

  function handleEdit(item: CredentialResponse) {
    goto(`/credentials/${item.id}`);
  }

  async function handleDelete(item: CredentialResponse) {
    if (!confirm(`确定要删除凭据 "${item.name}" 吗？`)) return;
    try {
      await credentialsApi.delete(item.id);
      loadData();
    } catch (err) {
      console.error('Failed to delete credential:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '凭据' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">凭据管理</h1>
    <Button onclick={() => goto('/credentials/new')}>新建凭据</Button>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索凭据..."
      class="max-w-sm"
      oninput={handleSearch}
    />
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={handleEdit}
    onDelete={handleDelete}
  />

  <div class="flex items-center justify-between text-sm text-muted-foreground">
    <span>显示 {Math.min(offset + 1, total)}-{Math.min(offset + limit, total)} / 共 {total} 条</span>
    <div class="flex gap-2">
      <Button
        variant="outline"
        size="sm"
        disabled={offset === 0}
        onclick={() => handlePageChange(Math.max(0, offset - limit))}
      >
        上一页
      </Button>
      <Button
        variant="outline"
        size="sm"
        disabled={offset + limit >= total}
        onclick={() => handlePageChange(offset + limit)}
      >
        下一页
      </Button>
    </div>
  </div>
</div>
