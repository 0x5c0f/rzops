<script lang="ts" generics="T">
  import { onMount } from 'svelte';
  import { browser } from '$app/environment';
  import * as Table from '$lib/ui/table';
  import { Button } from '$lib/ui/button';
  import * as Dialog from '$lib/ui/dialog';
  import { Badge } from '$lib/ui/badge';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
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
    /** 视口宽度低于该断点时自动隐藏（响应式列） */
    hideBelow?: 'sm' | 'md' | 'lg' | 'xl' | '2xl';
    /** Optional value map for resolving IDs to display names */
    valueMap?: Record<string, string>;
    /** Optional render function for custom display */
    render?: (value: unknown, item: T) => string;
    /** Optional custom display text (takes precedence) */
    display?: (item: T) => string;
    /** Optional link href; when present the cell renders as a link (null = plain text) */
    link?: (item: T) => string | null;
    /** 状态徽章：根据 item 返回徽章标签与样式类；返回 null 时不渲染徽章 */
    badge?: (item: T) => { label: string; className: string } | null;
    /** 状态徽章（StatusBadge 组件）：根据 item 返回状态值或 {status, color, label}；返回空时不渲染徽章 */
    statusBadge?: (item: T) => { status: string; color?: string | null; label?: string } | string | null | undefined;
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
    showIndex = true,
    page = 1,
    perPage = 20,
    children,
    extraActions,
    actionsWidth = 'w-[120px]',
    expandContent,
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
    /** 是否显示序号列，默认显示 */
    showIndex?: boolean;
    /** 当前页码，用于计算序号 */
    page?: number;
    /** 每页条数，用于计算序号 */
    perPage?: number;
    /** 默认插槽 */
    children?: import('svelte').Snippet;
    /** 额外操作 snippet（接收行数据），如重置密码按钮 */
    extraActions?: import('svelte').Snippet<[T]>;
    /** 操作列宽度，默认 w-[90px]；操作按钮较多时可覆盖为更宽 */
    actionsWidth?: string;
    /** 行展开内容 snippet（接收行数据）；传入后首列出现展开箭头，点击展开详情 */
    expandContent?: import('svelte').Snippet<[T]>;
  } = $props();

  let hasActions = $derived(!!(onEdit || onDelete || extraActions));
  let hasExpand = $derived(!!expandContent);
  let expandedKey = $state<string | null>(null);
  let confirmOpen = $state(false);
  let pendingDelete = $state<T | null>(null);

  // 列设置面板
  let columnSettingOpen = $state(false);
  let columnVisibility = $state<Record<string, boolean>>({});
  let storageInitialized = $state(false);

  /** 列的默认可见性：窄屏时 hideBelow 列默认隐藏，但用户显式勾选后优先显示 */
  function defaultVisible(col: Column): boolean {
    if (col.lockVisible) return true;
    if (col.hideInTable) return false;
    if (col.hideBelow && windowWidth < BREAKPOINTS[col.hideBelow]) return false;
    return true;
  }

  function loadVisibility() {
    if (!storageKey) {
      const vis: Record<string, boolean> = {};
      for (const col of columns) {
        vis[col.key] = defaultVisible(col);
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
          if (col.key in saved) {
            vis[col.key] = saved[col.key];
          } else {
            vis[col.key] = defaultVisible(col);
          }
        }
        columnVisibility = vis;
      } else {
        const vis: Record<string, boolean> = {};
        for (const col of columns) {
          vis[col.key] = defaultVisible(col);
        }
        columnVisibility = vis;
      }
    } catch {
      const vis: Record<string, boolean> = {};
      for (const col of columns) {
        vis[col.key] = defaultVisible(col);
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
      vis[col.key] = defaultVisible(col);
    }
    columnVisibility = vis;
    saveVisibility();
  }

  // 响应式列隐藏：视口宽度低于断点时自动隐藏
  const BREAKPOINTS: Record<string, number> = { sm: 640, md: 768, lg: 1024, xl: 1280, '2xl': 1536 };
  let windowWidth = $state(browser ? window.innerWidth : 9999);

  onMount(() => {
    loadVisibility();
    const onResize = () => {
      windowWidth = window.innerWidth;
    };
    window.addEventListener('resize', onResize);
    return () => window.removeEventListener('resize', onResize);
  });

  let visibleColumns = $derived(columns.filter((c) => columnVisibility[c.key] !== false));
  let visibleCount = $derived(visibleColumns.length);

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

  // —— 单元格截断 tooltip ——
  let tooltip = $state({ visible: false, x: 0, y: 0, text: '' });
  let tooltipTimer: ReturnType<typeof setTimeout> | null = null;

  function handleCellEnter(e: MouseEvent) {
    const td = e.currentTarget as HTMLElement;
    // 仅内容被截断（省略号）的单元格显示完整内容提示
    if (td.scrollWidth <= td.clientWidth) return;
    const text = td.textContent?.trim() ?? '';
    if (!text) return;
    clearTimeout(tooltipTimer ?? undefined);
    tooltipTimer = setTimeout(() => {
      tooltip = { visible: true, x: e.clientX + 12, y: e.clientY + 14, text };
    }, 250);
  }

  function handleCellMove(e: MouseEvent) {
    if (!tooltip.visible) return;
    tooltip.x = Math.min(e.clientX + 12, window.innerWidth - 260);
    tooltip.y = e.clientY + 14;
  }

  function handleCellLeave() {
    if (tooltipTimer) clearTimeout(tooltipTimer);
    tooltipTimer = null;
    tooltip.visible = false;
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

  <div class="rounded-md border">
    <Table.Root containerClass="overflow-x-clip" class="table-fixed">
      <Table.Header>
        <Table.Row>
          {#if hasExpand}
            <Table.Head class="sticky left-0 top-0 z-30 w-10 bg-background shadow-[1px_0_0_0_var(--border),0_1px_0_0_var(--border)]"></Table.Head>
          {/if}
          {#if showIndex}
            <Table.Head class={cn('w-[60px] sticky top-0 bg-background shadow-[0_1px_0_0_var(--border)] text-center', hasExpand ? 'left-10 z-20' : 'left-0 z-20')}>#</Table.Head>
          {/if}
          {#each visibleColumns as col}
            <Table.Head class={cn('sticky top-0 z-10 bg-background shadow-[0_1px_0_0_var(--border)]', col.class)}>{col.label}</Table.Head>
          {/each}
          {#if hasActions}
            <Table.Head class={cn('sticky top-0 z-10 bg-background text-center shadow-[0_1px_0_0_var(--border)]', actionsWidth)}>操作</Table.Head>
          {/if}
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {#if loading || !storageInitialized}
          <Table.Row>
            <Table.Cell colspan={visibleColumns.length + (hasActions ? 1 : 0) + (showIndex ? 1 : 0) + (hasExpand ? 1 : 0)} class="h-24 text-center text-muted-foreground">
              加载中...
            </Table.Cell>
          </Table.Row>
        {:else if data.length === 0}
          <Table.Row>
            <Table.Cell colspan={visibleColumns.length + (hasActions ? 1 : 0) + (showIndex ? 1 : 0) + (hasExpand ? 1 : 0)} class="h-24 text-center text-muted-foreground">
              暂无数据
            </Table.Cell>
          </Table.Row>
        {:else}
          {#each data as item, index}
            {@const rk = String((item as Record<string, unknown>).id ?? index)}
            {@const isOpen = expandedKey === rk}
            <Table.Row class={getRowClass ? getRowClass(item) : ''}>
              {#if hasExpand}
                <Table.Cell class="sticky left-0 z-20 w-10 bg-background text-center shadow-[1px_0_0_0_var(--border)]">
                  <Button variant="ghost" size="sm" class="h-7 w-7 p-0" onclick={() => (expandedKey = isOpen ? null : rk)} aria-label={isOpen ? '收起' : '展开'}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class={cn('transition-transform', isOpen && 'rotate-90')}><path d="m9 18 6-6-6-6"/></svg>
                  </Button>
                </Table.Cell>
              {/if}
              {#if showIndex}
                <Table.Cell class={cn('sticky bg-background text-center text-muted-foreground shadow-[1px_0_0_0_var(--border)]', hasExpand ? 'left-10 z-10' : 'left-0 z-20')}>{(page - 1) * perPage + index + 1}</Table.Cell>
              {/if}
              {#each visibleColumns as col}
                <Table.Cell
                  class={cn('truncate', col.class)}
                  onmouseenter={handleCellEnter}
                  onmousemove={handleCellMove}
                  onmouseleave={handleCellLeave}
                >
                  {#if col.statusBadge}
                    {@const sb = col.statusBadge(item)}
                    {#if sb}
                      {#if typeof sb === 'string'}
                        <StatusBadge status={sb} />
                      {:else}
                        <StatusBadge status={sb.status} color={sb.color} label={sb.label} />
                      {/if}
                    {:else}
                      {col.display ? col.display(item) : displayValue(item, col)}
                    {/if}
                  {:else if col.badge}
                    {@const badge = col.badge(item)}
                    {#if badge}
                      <Badge class={badge.className}>{badge.label}</Badge>
                    {:else}
                      {col.display ? col.display(item) : displayValue(item, col)}
                    {/if}
                  {:else if col.link}
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
                <Table.Cell class="text-center">
                  <div class="flex flex-nowrap justify-center gap-0.5">
                    {#if extraActions}
                      {@render extraActions(item)}
                    {/if}
                    {#if onEdit}
                      <Button variant="ghost" size="sm" class="px-1.5" onclick={() => onEdit(item)}>{editLabel}</Button>
                    {/if}
                    {#if onDelete}
                      <Button variant="ghost" size="sm" class="px-1.5" onclick={() => handleDeleteClick(item)}>删除</Button>
                    {/if}
                  </div>
                </Table.Cell>
              {/if}
            </Table.Row>
            {#if isOpen && expandContent}
              <Table.Row class="bg-muted/30">
                <Table.Cell colspan={visibleColumns.length + (hasActions ? 1 : 0) + (showIndex ? 1 : 0) + (hasExpand ? 1 : 0)}>
                  {@render expandContent(item)}
                </Table.Cell>
              </Table.Row>
            {/if}
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
            <span>{col.label} {#if col.hideBelow}<span class="text-xs text-muted-foreground">(窄屏默认隐藏)</span>{/if}</span>
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

{#if tooltip.visible}
  <div
    class="pointer-events-none fixed z-[100] max-w-[260px] truncate rounded-md border bg-popover px-2.5 py-1.5 text-xs text-popover-foreground shadow-md"
    style="left: {tooltip.x}px; top: {tooltip.y}px;"
  >
    {tooltip.text}
  </div>
{/if}
