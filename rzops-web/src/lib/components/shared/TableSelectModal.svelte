<script lang="ts">
  import { Button } from '$lib/ui/button';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import { Badge } from '$lib/ui/badge';
  import * as Dialog from '$lib/ui/dialog';
  import { cn } from '$lib/utils';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import X from '@lucide/svelte/icons/x';
  import Search from '@lucide/svelte/icons/search';
  import Loader2 from '@lucide/svelte/icons/loader-2';
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
    <Dialog.Content class="sm:max-w-3xl">
      <Dialog.Header>
        <Dialog.Title>{modalTitle}</Dialog.Title>
        <Dialog.Description>
          共 {total} 条，已选 {tempSelected.size} 项
        </Dialog.Description>
      </Dialog.Header>

      <!-- 搜索框 -->
      <div class="flex items-center gap-2 border-b px-4 py-3">
        <Search class="h-4 w-4 shrink-0 text-muted-foreground" />
        <input
          type="text"
          bind:value={keyword}
          oninput={onKeywordInput}
          placeholder={searchPlaceholder}
          class="h-8 w-full bg-transparent text-sm outline-none placeholder:text-muted-foreground"
        />
      </div>

      <!-- 表格 -->
      <div class="max-h-96 overflow-auto">
        {#if loading}
          <div class="flex items-center justify-center gap-2 py-12 text-sm text-muted-foreground">
            <Loader2 class="h-4 w-4 animate-spin" />
            加载中...
          </div>
        {:else if tableData.length === 0}
          <div class="py-12 text-center text-sm text-muted-foreground">无匹配结果</div>
        {:else}
          <table class="w-full text-sm">
            <thead>
              <tr>
                {#if multiple}
                  <th class="sticky top-0 z-10 w-10 bg-background px-2 py-2 text-left font-medium">
                    <input
                      type="checkbox"
                      class="h-4 w-4"
                      checked={tableData.length > 0 && tableData.every(item => isRowSelected(item))}
                      onchange={toggleAllOnPage}
                    />
                  </th>
                {/if}
                {#each columns as col}
                  <th class={cn('sticky top-0 z-10 bg-background px-2 py-2 text-left font-medium whitespace-nowrap', col.width)}>{col.label}</th>
                {/each}
              </tr>
            </thead>
            <tbody>
              {#each tableData as item (String(item[rowKey]))}
                <tr
                  class={isRowSelected(item) ? 'bg-accent/50 cursor-pointer' : 'cursor-pointer hover:bg-accent/30'}
                  onclick={() => toggleRow(item)}
                >
                  {#if multiple}
                    <td class="px-2 py-2">
                      <input
                        type="checkbox"
                        class="h-4 w-4"
                        checked={isRowSelected(item)}
                        onchange={(e) => { e.stopPropagation(); toggleRow(item); }}
                      />
                    </td>
                  {/if}
                  {#each columns as col}
                    <td class="px-2 py-2">{cellValue(item, col)}</td>
                  {/each}
                </tr>
              {/each}
            </tbody>
          </table>
        {/if}
      </div>

      <!-- 分页 -->
      {#if total > 0}
        <div class="flex items-center justify-between border-t px-4 py-3 text-sm">
          <span class="text-muted-foreground">
            第 {page} / {totalPages} 页，共 {total} 条
          </span>
          <div class="flex gap-1">
            <Button variant="outline" size="sm" disabled={page <= 1} onclick={() => goPage(page - 1)}>
              上一页
            </Button>
            <Button variant="outline" size="sm" disabled={page >= totalPages} onclick={() => goPage(page + 1)}>
              下一页
            </Button>
          </div>
        </div>
      {/if}

      <Dialog.Footer>
        <Button variant="outline" onclick={closeModal}>取消</Button>
        <Button onclick={confirmSelect} disabled={tempSelected.size === 0}>
          确认（已选 {tempSelected.size} 项）
        </Button>
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>
</div>
