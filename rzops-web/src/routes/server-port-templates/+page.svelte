<script lang="ts">
  import { goto } from '$app/navigation';
  import { serverPortTemplatesApi } from '$lib/api/server-port-templates';
  import type { ServerPortTemplateResponse, ListServerPortTemplatesQuery } from '$lib/types/server_port_template';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';
  import { protocolOptions } from '$lib/utils/enum-options';
import { canCreate, canUpdate, canDelete } from '$lib/utils/permissions';

  let data = $state<ServerPortTemplateResponse[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let query = $state<ListServerPortTemplatesQuery>({ page: 1, per_page: 20 });
  let page = $derived(query.page ?? 1);
  let perPage = $derived(query.per_page ?? 20);
  let protocolMap = $derived(Object.fromEntries($protocolOptions.map(o => [o.value, o.label])));

  const columns = $derived([
    { key: 'name', label: '模板名称', link: (item: ServerPortTemplateResponse) => `/server-port-templates/${item.id}`, lockVisible: true },
    { key: 'protocol', label: '协议', hideBelow: 'sm', valueMap: protocolMap },
    { key: 'port', label: '端口' },
    { key: 'service_name', label: '服务名称', hideBelow: 'lg' },
    { key: 'access_scope', label: '访问范围', hideBelow: 'lg' },
    { key: 'is_enabled', label: '启用', badge: (item: ServerPortTemplateResponse) =>
      item.is_enabled
        ? { label: '启用', className: 'bg-green-100 text-green-700 border-transparent' }
        : { label: '停用', className: 'bg-slate-100 text-slate-500 border-transparent' }
    },
  ]);

  async function loadData() {
    loading = true;
    try {
      const res = await serverPortTemplatesApi.list(query);
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load port templates:', err);
    } finally {
      loading = false;
    }
  }

  onMount(() => loadData());

  function handleSearch(e: Event) {
    const input = e.target as HTMLInputElement;
    query = { ...query, q: input.value, page: 1 };
    loadData();
  }

  function handlePageChange(newPage: number) {
    query = { ...query, page: newPage };
    loadData();
  }

  function handlePerPageChange(newPerPage: number) {
    query = { ...query, per_page: newPerPage, page: 1 };
    loadData();
  }

  function handleEdit(item: ServerPortTemplateResponse) {
    goto(`/server-port-templates/${item.id}/edit`);
  }

  async function handleDelete(item: ServerPortTemplateResponse) {
    try {
      await serverPortTemplatesApi.delete(item.id);
      data = data.filter(s => s.id !== item.id);
      total = total - 1;
      if (data.length === 0 && page > 1) {
        handlePageChange(page - 1);
      }
    } catch (err) {
      console.error('Failed to delete port template:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '端口模板' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">端口模板管理</h1>
    {#if canCreate('server_port_template')}
      <Button onclick={() => goto('/server-port-templates/new')}>新建端口模板</Button>
    {/if}
  </div>

  <p class="text-sm text-muted-foreground">
    端口模板用于批量录入：在服务器编辑页的"服务端口"卡片中可选择模板快速添加端口，每台服务器将生成独立的端口记录。
  </p>

  <div class="flex flex-wrap items-center gap-2">
    <Input
      placeholder="搜索模板名 / 服务名 / 端口..."
      class="max-w-sm"
      value={query.q ?? ''}
      oninput={handleSearch}
    />
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={canUpdate('server_port_template') ? handleEdit : undefined}
    onDelete={canDelete('server_port_template') ? handleDelete : undefined}
    storageKey="server-port-templates" />

  <Pagination
    {page}
    {perPage}
    {total}
    onPageChange={handlePageChange}
    onPerPageChange={handlePerPageChange}
  />
</div>
