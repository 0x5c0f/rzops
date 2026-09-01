<script lang="ts">
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import { Badge } from '$lib/ui/badge';
  import * as Dialog from '$lib/ui/dialog';
  import * as Table from '$lib/ui/table';
  import { cn } from '$lib/utils';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import X from '@lucide/svelte/icons/x';
  import Search from '@lucide/svelte/icons/search';
  import Loader2 from '@lucide/svelte/icons/loader-2';
  import ChevronLeft from '@lucide/svelte/icons/chevron-left';
  import ChevronRight from '@lucide/svelte/icons/chevron-right';
  import Inbox from '@lucide/svelte/icons/inbox';
  import Check from '@lucide/svelte/icons/check';
  import { onMount } from 'svelte';

  export interface TableSelectColumn {
    key: string;
    label: string;
    render?: (item: Record<string, unknown>) => string;
    width?: string;
  }

  export interface TableSelectPageResult {
    data: Record<string, unknown>[];
    total: number;
  }

  let {
    label,
    value = $bindable(),
    searchFn,
    columns,
    displayOptions = [] as { label: string; value: string }[],
    placeholder = '请选择',
    searchPlaceholder = '输入关键字搜索...',
    modalTitle = '选择数据',
    multiple = true,
    required = false,
    disabled = false,
    rowKey = 'id',
    labelKey = 'name',
    perPage = 10,
  }: {
    label?: string;
    value?: string[];
    searchFn: (keyword: string, page: number, perPage: number) => Promise<TableSelectPageResult>;
    columns: TableSelectColumn[];
    displayOptions?: { label: string; value: string }[];
    placeholder?: string;
    searchPlaceholder?: string;
    modalTitle?: string;
    multiple?: boolean;
    required?: boolean;
    disabled?: boolean;
    rowKey?: string;
    labelKey?: string;
    perPage?: number;
  } = $props();

  let modalOpen = $state(false);
  let keyword = $state('');
  let page = $state(1);
  let total = $state(0);
  let tableData = $state<Record<string, unknown>[]>([]);
  let loading = $state(false);
  /** 弹窗内临时选中：id -> 完整行对象，跨页保留 */
  let tempSelected = $state<Map<string, Record<string, unknown>>>(new Map());
  /** value -> label 缓存，用于触发器回显 */
  let labelCache = $state<Record<string, string>>({});
  let searchTimer: ReturnType<typeof setTimeout> | undefined;

  let selectedValues = $derived(Array.isArray(value) ? value : value ? [value] : []);

  let totalPages = $derived(Math.max(1, Math.ceil(total / perPage)));

  function labelOf(v: string): string {
    if (labelCache[v]) return labelCache[v];
    const fromDisplay = displayOptions.find(o => o.value === v);
    if (fromDisplay) return fromDisplay.label;
    const fromTemp = tempSelected.get(v);
    if (fromTemp) return String(fromTemp[labelKey] ?? v);
    return v;
  }

  let triggerText = $derived(
    selectedValues.length > 0 ? `已选择 ${selectedValues.length} 项` : placeholder
  );

  async function loadData() {
    loading = true;
    try {
      const res = await searchFn(keyword.trim(), page, perPage);
      tableData = res.data;
      total = res.total;
    } catch (err) {
      console.error('TableSelectModal load failed:', err);
      tableData = [];
      total = 0;
    } finally {
      loading = false;
    }
  }

  function openModal() {
    if (disabled) return;
    // 初始化临时选中：从当前 value 和 displayOptions 构建
    tempSelected = new Map();
    for (const v of selectedValues) {
      const lbl = labelOf(v);
      tempSelected.set(v, { [rowKey]: v, [labelKey]: lbl });
    }
    keyword = '';
    page = 1;
    modalOpen = true;
    loadData();
  }

  function closeModal() {
    modalOpen = false;
  }

  function onKeywordInput() {
    clearTimeout(searchTimer);
    searchTimer = setTimeout(() => {
      page = 1;
      loadData();
    }, 300);
  }

  function goPage(p: number) {
    if (p < 1 || p > totalPages) return;
    page = p;
    loadData();
  }

  function isRowSelected(item: Record<string, unknown>): boolean {
    const id = String(item[rowKey]);
    return tempSelected.has(id);
  }

  function toggleRow(item: Record<string, unknown>) {
    const id = String(item[rowKey]);
    if (tempSelected.has(id)) {
      tempSelected.delete(id);
    } else {
      if (!multiple) {
        tempSelected.clear();
      }
      tempSelected.set(id, item);
    }
    // 触发响应式更新
    tempSelected = new Map(tempSelected);
  }

  function toggleAllOnPage() {
    const allSelected = tableData.every(item => isRowSelected(item));
    if (allSelected) {
      for (const item of tableData) {
        tempSelected.delete(String(item[rowKey]));
      }
    } else {
      for (const item of tableData) {
        tempSelected.set(String(item[rowKey]), item);
      }
    }
    tempSelected = new Map(tempSelected);
  }

  function removeValue(v: string, e: MouseEvent) {
    e.stopPropagation();
    if (disabled) return;
    if (multiple && Array.isArray(value)) {
      value = value.filter(item => item !== v);
    } else {
      value = [];
    }
  }

  function confirmSelect() {
    const ids = Array.from(tempSelected.keys());
    value = multiple ? ids : ids.slice(0, 1);
    // 更新 labelCache
    for (const [id, item] of tempSelected) {
      labelCache[id] = String(item[labelKey] ?? id);
    }
    modalOpen = false;
  }

  function cellValue(item: Record<string, unknown>, col: TableSelectColumn): string {
    if (col.render) return col.render(item);
    const raw = item[col.key];
    if (raw === null || raw === undefined) return '-';
    return String(raw);
  }

  onMount(() => {
    // 从 displayOptions 初始化 labelCache
    for (const opt of displayOptions) {
      labelCache[opt.value] = opt.label;
    }
  });
</script>

<div class={cn('space-y-2')}>
  {#if label}
    <Label>
      {label}
      {#if required}
        <span class="text-destructive">*</span>
      {/if}
    </Label>
  {/if}

  <!-- 已选标签 -->
  {#if selectedValues.length > 0}
    <div class="flex flex-wrap gap-1">
      {#each selectedValues as v}
        <Badge variant="secondary" class="gap-1">
          {labelOf(v)}
          {#if !disabled}
            <button
              type="button"
              aria-label="移除"
              onclick={(e) => removeValue(v, e)}
              class="ml-1 rounded-full hover:bg-muted"
            >
              <X class="h-3 w-3" />
            </button>
          {/if}
        </Badge>
      {/each}
    </div>
  {/if}

  <!-- 触发器 -->
  <button
    type="button"
    onclick={openModal}
    disabled={disabled}
    data-placeholder={selectedValues.length === 0 ? true : undefined}
    class="border-input data-[placeholder]:text-muted-foreground [&_svg]:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 flex w-full items-center justify-between gap-2 rounded-md border bg-transparent px-3 py-2 text-sm whitespace-nowrap shadow-xs transition-[color,box-shadow] outline-none select-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 h-9 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg]:size-4"
  >
    <span class="truncate">{triggerText}</span>
    <ChevronDown class="size-4 opacity-50" />
  </button>

  <!-- 选择弹窗 -->
  <Dialog.Root bind:open={modalOpen}>
    <Dialog.Content class="sm:max-w-4xl p-0 overflow-hidden">
      <!-- 头部 -->
      <div class="border-b bg-gradient-to-r from-primary/5 to-transparent px-6 py-4">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-lg font-semibold">{modalTitle}</h2>
            <p class="mt-0.5 text-sm text-muted-foreground">
              共 <span class="font-medium text-foreground">{total}</span> 条数据
              {#if tempSelected.size > 0}
                · 已选 <span class="font-medium text-primary">{tempSelected.size}</span> 项
              {/if}
            </p>
          </div>
          <button
            type="button"
            onclick={closeModal}
            class="rounded-md p-1.5 text-muted-foreground transition-colors hover:bg-muted hover:text-foreground"
            aria-label="关闭"
          >
            <X class="h-4 w-4" />
          </button>
        </div>
      </div>

      <!-- 搜索框 -->
      <div class="border-b px-6 py-3">
        <div class="relative">
          <Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <input
            type="text"
            bind:value={keyword}
            oninput={onKeywordInput}
            placeholder={searchPlaceholder}
            class="h-9 w-full rounded-lg border border-input bg-background pl-9 pr-4 text-sm outline-none transition-colors placeholder:text-muted-foreground focus:border-ring focus:ring-2 focus:ring-ring/30"
          />
          {#if keyword}
            <button
              type="button"
              onclick={() => { keyword = ''; page = 1; loadData(); }}
              class="absolute right-2 top-1/2 -translate-y-1/2 rounded p-1 text-muted-foreground hover:bg-muted hover:text-foreground"
              aria-label="清除搜索"
            >
              <X class="h-3.5 w-3.5" />
            </button>
          {/if}
        </div>
      </div>

      <!-- 已选标签区 -->
      {#if tempSelected.size > 0}
        <div class="flex flex-wrap items-center gap-1.5 border-b bg-muted/30 px-6 py-2.5">
          <span class="text-xs font-medium text-muted-foreground">已选：</span>
          {#each Array.from(tempSelected.entries()) as [id, item]}
            <Badge variant="secondary" class="gap-1 border-primary/20 bg-primary/10 text-primary-foreground">
              <span class="text-primary">{String(item[labelKey] ?? id)}</span>
              <button
                type="button"
                onclick={() => { tempSelected.delete(id); tempSelected = new Map(tempSelected); }}
                class="ml-0.5 rounded-full hover:bg-primary/20"
                aria-label="移除"
              >
                <X class="h-3 w-3" />
              </button>
            </Badge>
          {/each}
          <button
            type="button"
            onclick={() => { tempSelected.clear(); tempSelected = new Map(); }}
            class="ml-auto text-xs text-muted-foreground underline-offset-2 hover:text-destructive hover:underline"
          >
            清空全部
          </button>
        </div>
      {/if}

      <!-- 表格 -->
      <div class="max-h-[420px] overflow-auto">
        {#if loading}
          <div class="flex flex-col items-center justify-center gap-3 py-16">
            <Loader2 class="h-8 w-8 animate-spin text-primary" />
            <span class="text-sm text-muted-foreground">加载中...</span>
          </div>
        {:else if tableData.length === 0}
          <div class="flex flex-col items-center justify-center gap-3 py-16">
            <div class="rounded-full bg-muted p-4">
              <Inbox class="h-8 w-8 text-muted-foreground" />
            </div>
            <div class="text-center">
              <p class="text-sm font-medium">无匹配结果</p>
              <p class="mt-1 text-xs text-muted-foreground">
                {#if keyword}
                  没有找到与「{keyword}」匹配的数据，试试其他关键词
                {:else}
                  暂无数据
                {/if}
              </p>
            </div>
          </div>
        {:else}
          <Table.Root>
            <Table.Header class="sticky top-0 z-10 bg-muted/80 backdrop-blur">
              <Table.Row>
                <Table.Head class="w-12 px-4">
                  <input
                    type="checkbox"
                    class="h-4 w-4 rounded border-input accent-primary"
                    checked={tableData.length > 0 && tableData.every(item => isRowSelected(item))}
                    onchange={toggleAllOnPage}
                  />
                </Table.Head>
                {#each columns as col}
                  <Table.Head class={col.width ? `${col.width} px-4 py-3 text-xs font-semibold uppercase tracking-wide` : 'px-4 py-3 text-xs font-semibold uppercase tracking-wide'}>
                    {col.label}
                  </Table.Head>
                {/each}
              </Table.Row>
            </Table.Header>
            <Table.Body>
              {#each tableData as item, index (String(item[rowKey]))}
                <Table.Row
                  class={cn(
                    'cursor-pointer transition-colors',
                    index % 2 === 1 ? 'bg-muted/20' : '',
                    isRowSelected(item) ? 'bg-primary/10 hover:bg-primary/15' : 'hover:bg-muted/40'
                  )}
                  onclick={() => toggleRow(item)}
                >
                  <Table.Cell class="px-4 py-3">
                    <div class={cn(
                      'flex h-5 w-5 items-center justify-center rounded border transition-colors',
                      isRowSelected(item) ? 'border-primary bg-primary' : 'border-input bg-background'
                    )}>
                      {#if isRowSelected(item)}
                        <Check class="h-3.5 w-3.5 text-primary-foreground" />
                      {/if}
                    </div>
                  </Table.Cell>
                  {#each columns as col}
                    <Table.Cell class="px-4 py-3 text-sm">
                      {#if col.key === labelKey && isRowSelected(item)}
                        <span class="font-medium text-primary">{cellValue(item, col)}</span>
                      {:else}
                        {cellValue(item, col)}
                      {/if}
                    </Table.Cell>
                  {/each}
                </Table.Row>
              {/each}
            </Table.Body>
          </Table.Root>
        {/if}
      </div>

      <!-- 分页 -->
      {#if total > 0}
        <div class="flex items-center justify-between border-t bg-muted/20 px-6 py-3">
          <span class="text-xs text-muted-foreground">
            显示第 {(page - 1) * perPage + 1}-{Math.min(page * perPage, total)} 条，共 {total} 条
          </span>
          <div class="flex items-center gap-1">
            <button
              type="button"
              disabled={page <= 1}
              onclick={() => goPage(page - 1)}
              class="flex h-8 w-8 items-center justify-center rounded-md border border-input text-muted-foreground transition-colors hover:bg-muted hover:text-foreground disabled:cursor-not-allowed disabled:opacity-40"
              aria-label="上一页"
            >
              <ChevronLeft class="h-4 w-4" />
            </button>
            <span class="px-3 text-sm font-medium">
              {page} <span class="text-muted-foreground">/ {totalPages}</span>
            </span>
            <button
              type="button"
              disabled={page >= totalPages}
              onclick={() => goPage(page + 1)}
              class="flex h-8 w-8 items-center justify-center rounded-md border border-input text-muted-foreground transition-colors hover:bg-muted hover:text-foreground disabled:cursor-not-allowed disabled:opacity-40"
              aria-label="下一页"
            >
              <ChevronRight class="h-4 w-4" />
            </button>
          </div>
        </div>
      {/if}

      <!-- 底部操作 -->
      <div class="flex items-center justify-between border-t px-6 py-4">
        <span class="text-sm text-muted-foreground">
          {#if tempSelected.size > 0}
            已选择 <span class="font-semibold text-primary">{tempSelected.size}</span> 项
          {:else}
            请从列表中选择
          {/if}
        </span>
        <div class="flex gap-2">
          <Button variant="outline" onclick={closeModal}>取消</Button>
          <Button onclick={confirmSelect} disabled={tempSelected.size === 0} class="gap-1.5">
            <Check class="h-4 w-4" />
            确认选择
          </Button>
        </div>
      </div>
    </Dialog.Content>
  </Dialog.Root>
</div>
