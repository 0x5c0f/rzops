<script lang="ts" generics="T">
  import { onMount } from 'svelte';
  import * as Table from '$lib/ui/table';
  import { Button } from '$lib/ui/button';
  import * as Dialog from '$lib/ui/dialog';
  import ConfirmDialog from '$lib/components/shared/ConfirmDialog.svelte';
  import { cn } from '$lib/utils';

  interface Column {
    key: string;
    label: string;
    class?: string;
    /** 默认隐藏，用户可在列设置中开启 */
    hideInTable?: boolean;
    /** 锁定可见，不允许在列设置中隐藏 */
    lockVisible?: boolean;
    /** Optional value map for resolving IDs to display names */
    valueMap?: Record<string, string>;
    /** Optional render function for custom display */
    render?: (value: unknown, item: T) => string;
    /** Optional custom display text (takes precedence) */
    display?: (item: T) => string;
    /** Optional link href; when present the cell renders as a link (null = plain text) */
    link?: (item: T) => string | null;
  }

  let {
    columns,
    data,
    loading = false,
    onEdit,
    onDelete,
    editLabel = '编辑',
    getDeleteLabel,
    deleteTitle = '确认删除',
    storageKey = '',
    getRowClass,
  }: {
    columns: Column[];
    data: T[];
    loading?: boolean;
    onEdit?: (item: T) => void;
    onDelete?: (item: T) => void;
    editLabel?: string;
    getDeleteLabel?: (item: T) => string;
    deleteTitle?: string;
    storageKey?: string;
    /** 自定义行样式类名，根据行数据返回 */
    getRowClass?: (item: T) => string;
  } = $props();

  let hasActions = $derived(onEdit || onDelete);
  let confirmOpen = $state(false);
  let pendingDelete = $state<T | null>(null);

  // 列设置面板
  let columnSettingOpen = $state(false);
  let columnVisibility = $state<Record<string, boolean>>({});
  let storageInitialized = $state(false);

  function loadVisibility() {
    if (!storageKey) {
      const vis: Record<string, boolean> = {};
      for (const col of columns) {
        vis[col.key] = !col.hideInTable;
      }
      columnVisibility = vis;
      storageInitialized = true;
      return;
    }
    try {
      const raw = localStorage.getItem(`dt_cols:${storageKey}`);
      if (raw) {
        const saved = JSON.parse(raw) as Record<string, boolean>;
        const vis: Record<string, boolean> = {};
        for (const col of columns) {
          if (col.lockVisible) {
            vis[col.key] = true;
          } else if (col.key in saved) {
            vis[col.key] = saved[col.key];
          } else {
            vis[col.key] = !col.hideInTable;
          }
        }
        columnVisibility = vis;
      } else {
        const vis: Record<string, boolean> = {};
        for (const col of columns) {
          vis[col.key] = !col.hideInTable;
        }
        columnVisibility = vis;
      }
    } catch {
      const vis: Record<string, boolean> = {};
      for (const col of columns) {
        vis[col.key] = !col.hideInTable;
      }
      columnVisibility = vis;
    }
    storageInitialized = true;
  }

  function saveVisibility() {
    if (!storageKey) return;
    try {
      const toSave: Record<string, boolean> = {};
      for (const col of columns) {
        if (!col.lockVisible) {
          toSave[col.key] = columnVisibility[col.key] ?? true;
        }
      }
      localStorage.setItem(`dt_cols:${storageKey}`, JSON.stringify(toSave));
    } catch {
      // ignore
    }
  }

  function toggleColumn(key: string) {
    columnVisibility = { ...columnVisibility, [key]: !columnVisibility[key] };
    saveVisibility();
  }

  function resetColumns() {
    const vis: Record<string, boolean> = {};
    for (const col of columns) {
      vis[col.key] = col.lockVisible ? true : !col.hideInTable;
    }
    columnVisibility = vis;
    saveVisibility();
  }

  let visibleColumns = $derived(columns.filter((c) => columnVisibility[c.key] !== false));
  let visibleCount = $derived(visibleColumns.length);

  onMount(() => {
    loadVisibility();
  });

  function getValue(item: T, key: string): unknown {
    return (item as Record<string, unknown>)[key];
  }

  function displayValue(item: T, col: Column): string {
    const raw = getValue(item, col.key);
    if (col.render) return col.render(raw, item);
    if (col.valueMap && raw) return col.valueMap[String(raw)] || '-';
    if (Array.isArray(raw)) return raw.join(', ') || '-';
    if (raw === null || raw === undefined) return '-';
    return String(raw);
  }

  function handleDeleteClick(item: T) {
    pendingDelete = item;
    confirmOpen = true;
  }

  function confirmDelete() {
    if (pendingDelete && onDelete) onDelete(pendingDelete);
    pendingDelete = null;
    confirmOpen = false;
  }

  function deleteLabel(): string {
    if (!pendingDelete) return '';
    if (getDeleteLabel) return getDeleteLabel(pendingDelete);
    const name = (pendingDelete as Record<string, unknown>).name;
    return name ? String(name) : '该项';
  }
</script>

<div class="relative">
  <!-- 列设置按钮 -->
  <div class="mb-2 flex justify-end">
    <Button variant="outline" size="sm" onclick={() => (columnSettingOpen = true)}>
      <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="mr-1"><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M3 9h18"/><path d="M9 21V9"/></svg>
      列设置 ({visibleCount}/{columns.length})
    </Button>
  </div>

  <div class="rounded-md border overflow-auto max-h-[calc(100vh-280px)]">
    <Table.Root>
      <Table.Header>
        <Table.Row>
          {#each visibleColumns as col}
            <Table.Head class={cn('sticky top-0 z-10 bg-background shadow-[0_1px_0_0_var(--border)]', col.class)}>{col.label}</Table.Head>
          {/each}
          {#if hasActions}
            <Table.Head class="w-[100px] sticky top-0 z-10 bg-background shadow-[0_1px_0_0_var(--border)]">操作</Table.Head>
          {/if}
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {#if loading || !storageInitialized}
          <Table.Row>
            <Table.Cell colspan={visibleColumns.length + (hasActions ? 1 : 0)} class="h-24 text-center text-muted-foreground">
              加载中...
            </Table.Cell>
          </Table.Row>
        {:else if data.length === 0}
          <Table.Row>
            <Table.Cell colspan={visibleColumns.length + (hasActions ? 1 : 0)} class="h-24 text-center text-muted-foreground">
              暂无数据
            </Table.Cell>
          </Table.Row>
        {:else}
          {#each data as item}
            <Table.Row class={getRowClass ? getRowClass(item) : ''}>
              {#each visibleColumns as col}
                <Table.Cell class={col.class}>
                  {#if col.link}
                    {@const href = col.link(item)}
                    {#if href}
                      <a href={href} class="text-primary hover:underline">
                        {col.display ? col.display(item) : displayValue(item, col)}
                      </a>
                    {:else}
                      {col.display ? col.display(item) : displayValue(item, col)}
                    {/if}
                  {:else}
                    {col.display ? col.display(item) : displayValue(item, col)}
                  {/if}
                </Table.Cell>
              {/each}
              {#if hasActions}
                <Table.Cell>
                  <div class="flex gap-1">
                    {#if onEdit}
                      <Button variant="ghost" size="sm" onclick={() => onEdit(item)}>{editLabel}</Button>
                    {/if}
                    {#if onDelete}
                      <Button variant="ghost" size="sm" onclick={() => handleDeleteClick(item)}>删除</Button>
                    {/if}
                  </div>
                </Table.Cell>
              {/if}
            </Table.Row>
          {/each}
        {/if}
      </Table.Body>
    </Table.Root>
  </div>
</div>

<!-- 列设置对话框 -->
<Dialog.Root bind:open={columnSettingOpen}>
  <Dialog.Content class="sm:max-w-md">
    <Dialog.Header>
      <Dialog.Title>列设置</Dialog.Title>
      <Dialog.Description>选择要在表格中显示的列，配置自动保存。</Dialog.Description>
    </Dialog.Header>
    <div class="max-h-80 space-y-1 overflow-y-auto py-2">
      {#each columns as col}
        <label class="flex cursor-pointer items-center gap-3 rounded-md px-3 py-2 text-sm hover:bg-accent">
          {#if col.lockVisible}
            <input type="checkbox" checked disabled class="h-4 w-4" />
            <span class="text-muted-foreground">{col.label} <span class="text-xs">(固定)</span></span>
          {:else}
            <input
              type="checkbox"
              checked={columnVisibility[col.key] !== false}
              onchange={() => toggleColumn(col.key)}
              class="h-4 w-4"
            />
            <span>{col.label}</span>
          {/if}
        </label>
      {/each}
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={resetColumns}>恢复默认</Button>
      <Button onclick={() => (columnSettingOpen = false)}>完成</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>

{#if onDelete}
  <ConfirmDialog
    bind:open={confirmOpen}
    title={deleteTitle}
    description={`确定要删除「${deleteLabel()}」吗？此操作可在回收站恢复。`}
    confirmLabel="删除"
    onConfirm={confirmDelete}
  />
{/if}
