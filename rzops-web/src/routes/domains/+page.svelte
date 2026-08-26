<script lang="ts">
  import { goto } from '$app/navigation';
  import { domainsApi } from '$lib/api/domains';
  import type { DomainResponse, ListDomainsQuery } from '$lib/types/domain';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { formatDate } from '$lib/utils/format';
  import { onMount } from 'svelte';

  let data = $state<DomainResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListDomainsQuery>({ limit: 20, offset: 0 });
  let offset = $derived(query.offset ?? 0);
  let limit = $derived(query.limit ?? 20);

  const columns = [
    { key: 'domain_name', label: '域名' },
    { key: 'provider_id', label: '注册商' },
    { key: 'expiry_date', label: '到期日期', render: (v: unknown) => formatDate(v as string) },
    { key: 'is_enabled', label: '启用状态' },
    { key: 'created_at', label: '创建时间', render: (v: unknown) => formatDate(v as string) },
  ];

  async function loadData() {
    loading = true;
    try {
      const res = await domainsApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load domains:', err);
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

  function handleEdit(item: DomainResponse) {
    goto(`/domains/${item.id}/edit`);
  }

  async function handleDelete(item: DomainResponse) {
    if (!confirm(`确定要删除域名 "${item.domain_name}" 吗？`)) return;
    try {
      await domainsApi.delete(item.id);
      loadData();
    } catch (err) {
      console.error('Failed to delete domain:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '域名' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">域名管理</h1>
    <Button onclick={() => goto('/domains/new')}>新建域名</Button>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索域名..."
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
