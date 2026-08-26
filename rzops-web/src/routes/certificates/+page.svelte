<script lang="ts">
  import { goto } from '$app/navigation';
  import { certificatesApi } from '$lib/api/certificates';
  import type { CertificateResponse, ListCertificatesQuery } from '$lib/types/certificate';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';
  import { getProviderOptions } from '$lib/utils/entity-options';
  import { certificateStatusOptions, certificateTypeOptions } from '$lib/utils/enum-options';

  let data = $state<CertificateResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListCertificatesQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let providerMap = $state<Record<string, string>>({});

  let certificateTypeMap = $derived(Object.fromEntries($certificateTypeOptions.map(o => [o.value, o.label])));
  let certificateStatusMap = $derived(Object.fromEntries($certificateStatusOptions.map(o => [o.value, o.label])));

  const columns = $derived([
    { key: 'name', label: '名称' , link: (item: CertificateResponse) => `/certificates/${item.id}` },
    { key: 'certificate_type', label: '类型', valueMap: certificateTypeMap },
    { key: 'provider_id', label: '供应商', valueMap: providerMap },
    { key: 'status', label: '状态', valueMap: certificateStatusMap },
    { key: 'lease_end_date', label: '到期日期' },
  ]);

  async function loadData() {
    loading = true;
    try {
      const res = await certificatesApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load certificates:', err);
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    const provOptions = await getProviderOptions();
    Object.assign(providerMap, Object.fromEntries(provOptions.map(o => [o.value, o.label])));
    await loadData();
  });

  function handleSearch(e: Event) {
    const input = e.target as HTMLInputElement;
    query = { ...query, q: input.value, page: 1 };
    loadData();
  }

  function handlePageChange(newPage: number) {
    query = { ...query, page: newPage };
    loadData();
  }

  function handleEdit(item: CertificateResponse) {
    goto(`/certificates/${item.id}/edit`);
  }

  async function handleDelete(item: CertificateResponse) {
    if (!confirm(`确定要删除证书 "${item.name}" 吗？`)) return;
    try {
      await certificatesApi.delete(item.id);
      loadData();
    } catch (err) {
      console.error('Failed to delete certificate:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '证书' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">证书管理</h1>
    <Button onclick={() => goto('/certificates/new')}>新建证书</Button>
  </div>

  <div class="flex gap-2">
    <Input
      placeholder="搜索证书..."
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
    <span>显示 {Math.min((page - 1) * perPage + 1, total)}-{Math.min(page * perPage, total)} / 共 {total} 条</span>
    <div class="flex gap-2">
      <Button
        variant="outline"
        size="sm"
        disabled={page <= 1}
        onclick={() => handlePageChange(page - 1)}
      >
        上一页
      </Button>
      <Button
        variant="outline"
        size="sm"
        disabled={page * perPage >= total}
        onclick={() => handlePageChange(page + 1)}
      >
        下一页
      </Button>
    </div>
  </div>
</div>
