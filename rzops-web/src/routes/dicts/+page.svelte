<script lang="ts">
  import { dictsApi, type DictItem, type CreateDictRequest, type UpdateDictRequest } from '$lib/api/dicts';
  import { loadAllDicts } from '$lib/utils/enum-options';
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import * as Dialog from '$lib/ui/dialog';
  import * as Select from '$lib/ui/select';
  import DataTable from '$lib/components/shared/DataTable.svelte';
  import Breadcrumb from '$lib/components/layout/Breadcrumb.svelte';
  import { onMount } from 'svelte';

  let data = $state<DictItem[]>([]);
  let total = $state(0);
  let loading = $state(true);
  let q = $state('');
  let dictType = $state('');
  let types = $state<string[]>([]);
  let typeSearch = $state('');
  let page = $state(1);
  const perPage = 20;
  let offset = $derived((page - 1) * perPage);

  // 新建 / 编辑对话框状态
  let dialogOpen = $state(false);
  let editing = $state<DictItem | null>(null);
  let form = $state<CreateDictRequest>({ dict_type: '', dict_code: '', dict_label: '', sort_order: 0, enabled: true, remark: '' });
  let saving = $state(false);
  let error = $state('');

  const columns = $derived([
    { key: 'dict_type', label: '字典类型' },
    { key: 'dict_code', label: '编码' },
    { key: 'dict_label', label: '显示名称' },
    { key: 'sort_order', label: '排序' },
    {
      key: 'enabled', label: '状态',
      display: (item: DictItem) => (item.enabled ? '启用' : '停用'),
    },
    { key: 'remark', label: '备注' },
  ]);

  async function loadData() {
    loading = true;
    try {
      const res = await dictsApi.list({ dict_type: dictType || undefined, q: q || undefined, page, per_page: perPage });
      data = res.data;
      total = res.count;
    } catch (err) {
      console.error('Failed to load dicts:', err);
    } finally {
      loading = false;
    }
  }

  async function loadTypes() {
    try {
      const res = await dictsApi.list();
      const set = new Set<string>();
      res.data.forEach((i) => set.add(i.dict_type));
      types = [...set].sort();
    } catch (err) {
      console.error('Failed to load dict types:', err);
    }
  }

  onMount(() => {
    loadData();
    loadTypes();
  });

  function openCreate() {
    editing = null;
    form = { dict_type: dictType || '', dict_code: '', dict_label: '', sort_order: 0, enabled: true, remark: '' };
    error = '';
    dialogOpen = true;
  }

  function openEdit(item: DictItem) {
    editing = item;
    form = {
      dict_type: item.dict_type,
      dict_code: item.dict_code,
      dict_label: item.dict_label,
      sort_order: item.sort_order,
      enabled: item.enabled,
      remark: item.remark ?? '',
    };
    error = '';
    dialogOpen = true;
  }

  async function handleSave() {
    if (!form.dict_type.trim() || !form.dict_code.trim() || !form.dict_label.trim()) {
      error = '字典类型、编码和显示名称均为必填项';
      return;
    }
    saving = true;
    error = '';
    try {
      if (editing) {
        const payload: UpdateDictRequest = {
          dict_label: form.dict_label,
          sort_order: form.sort_order ?? 0,
          enabled: form.enabled,
          remark: form.remark || undefined,
        };
        await dictsApi.update(editing.id, payload);
      } else {
        await dictsApi.create({ ...form, dict_type: form.dict_type.trim(), dict_code: form.dict_code.trim(), dict_label: form.dict_label.trim() });
      }
      dialogOpen = false;
      await Promise.all([loadData(), loadTypes()]);
      loadAllDicts(); // 刷新前端各表单下拉
    } catch (err) {
      error = err instanceof Error ? err.message : '保存失败';
    } finally {
      saving = false;
    }
  }

  async function handleDelete(item: DictItem) {
    if (!confirm(`确定要停用字典项 "${item.dict_label}" (${item.dict_code}) 吗？停用后表单中将不再展示。`)) return;
    try {
      await dictsApi.delete(item.id);
      await loadData();
      loadAllDicts();
    } catch (err) {
      console.error('Failed to delete dict:', err);
    }
  }
</script>

<div class="space-y-4">
  <Breadcrumb items={[{ label: '字典管理' }]} />

  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">字典管理</h1>
    <Button onclick={openCreate}>新建字典项</Button>
  </div>

  <div class="flex gap-2">
    <Select.Root type="single" bind:value={dictType} onValueChange={() => { page = 1; loadData(); }}>
      <Select.Trigger class="w-48">
        {dictType || '全部类型'}
      </Select.Trigger>
      <Select.Content>
        <div class="px-2 pt-2" onclick={(e) => e.stopPropagation()}>
          <Input
            placeholder="搜索类型..."
            value={typeSearch}
            oninput={(e) => (typeSearch = (e.target as HTMLInputElement).value)}
          />
        </div>
        <Select.Item value="">全部类型</Select.Item>
        {#each types.filter((t) => !typeSearch || t.toLowerCase().includes(typeSearch.toLowerCase())) as t}
          <Select.Item value={t}>{t}</Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>
    <Input
      placeholder="搜索编码 / 名称 / 类型..."
      class="max-w-sm"
      value={q}
      oninput={(e) => { q = (e.target as HTMLInputElement).value; page = 1; loadData(); }}
    />
  </div>

  <DataTable
    {columns}
    {data}
    {loading}
    onEdit={openEdit}
    onDelete={handleDelete}
  />

  <div class="flex items-center justify-between text-sm text-muted-foreground">
    <span>显示 {total === 0 ? 0 : offset + 1}-{Math.min(offset + perPage, total)} / 共 {total} 条（停用的项不再出现在业务表单下拉中，历史数据名称不受影响）</span>
    <div class="flex gap-2">
      <Button
        variant="outline"
        size="sm"
        disabled={page <= 1}
        onclick={() => { page -= 1; loadData(); }}
      >
        上一页
      </Button>
      <Button
        variant="outline"
        size="sm"
        disabled={offset + perPage >= total}
        onclick={() => { page += 1; loadData(); }}
      >
        下一页
      </Button>
    </div>
  </div>

  <!-- 新建 / 编辑对话框 -->
  <Dialog.Root bind:open={dialogOpen}>
    <Dialog.Content class="sm:max-w-md">
      <Dialog.Header>
        <Dialog.Title>{editing ? '编辑字典项' : '新建字典项'}</Dialog.Title>
        <Dialog.Description>
          {editing ? '修改显示名称、排序或启用状态' : '为业务表单新增一个可选项'}
        </Dialog.Description>
      </Dialog.Header>
      <div class="space-y-4 py-2">
        <div class="space-y-2">
          <Label for="dt">字典类型 *</Label>
          <Input id="dt" bind:value={form.dict_type} placeholder="如 server_type / server_status" disabled={!!editing} />
        </div>
        <div class="grid grid-cols-2 gap-3">
          <div class="space-y-2">
            <Label for="dc">编码 *</Label>
            <Input id="dc" bind:value={form.dict_code} placeholder="如 physical / active" disabled={!!editing} />
          </div>
          <div class="space-y-2">
            <Label for="dl">显示名称 *</Label>
            <Input id="dl" bind:value={form.dict_label} placeholder="如 物理机 / 运行中" />
          </div>
        </div>
        <div class="grid grid-cols-2 gap-3">
          <div class="space-y-2">
            <Label for="so">排序</Label>
            <Input id="so" type="number" bind:value={form.sort_order} />
          </div>
          <div class="flex items-center gap-2 pt-6">
            <input type="checkbox" id="en" bind:checked={form.enabled} class="h-4 w-4" />
            <Label for="en">启用</Label>
          </div>
        </div>
        <div class="space-y-2">
          <Label for="rm">备注</Label>
          <Input id="rm" bind:value={form.remark} placeholder="可选" />
        </div>
        {#if error}
          <p class="text-sm text-red-600">{error}</p>
        {/if}
      </div>
      <Dialog.Footer>
        <Button variant="outline" onclick={() => (dialogOpen = false)}>取消</Button>
        <Button onclick={handleSave} disabled={saving}>{saving ? '保存中...' : '保存'}</Button>
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>
</div>
