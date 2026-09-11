<script lang="ts">
  import { Label } from '$lib/ui/label';
  import { Input } from '$lib/ui/input';
  import { Badge } from '$lib/ui/badge';
  import { cn } from '$lib/utils';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import Check from '@lucide/svelte/icons/check';
  import X from '@lucide/svelte/icons/x';
  import Search from '@lucide/svelte/icons/search';
  import Loader2 from '@lucide/svelte/icons/loader-2';

  export interface RemoteSelectOption {
    label: string;
    value: string;
  }

  let {
    label,
    value = $bindable(),
    searchFn,
    displayOptions = [],
    placeholder = '请选择',
    searchPlaceholder = '输入关键字搜索...',
    required = false,
    disabled = false,
    multiple = false,
    id = undefined,
    class: className = '',
  }: {
    label?: string;
    value?: string | string[] | undefined;
    /** 远程搜索：输入关键字（空串表示初始加载）后调用，返回匹配选项 */
    searchFn: (keyword: string) => Promise<RemoteSelectOption[]>;
    /** 已选值回显用（编辑时传入当前已关联对象的 label） */
    displayOptions?: RemoteSelectOption[];
    placeholder?: string;
    searchPlaceholder?: string;
    required?: boolean;
    disabled?: boolean;
    multiple?: boolean;
    /** 供外部 <label for> 关联的控件 id */
    id?: string;
    class?: string;
  } = $props();

  let open = $state(false);
  let keyword = $state('');
  let options = $state<RemoteSelectOption[]>([]);
  let loading = $state(false);
  let searchTimer: ReturnType<typeof setTimeout> | undefined;
  let containerRef = $state<HTMLDivElement | null>(null);
  /** 已选值 -> label 缓存（搜索结果变化后仍能正确回显已选项名称） */
  let labelCache = $state<Record<string, string>>({});

  function onDocClick(e: MouseEvent) {
    if (containerRef && !containerRef.contains(e.target as Node)) {
      open = false;
    }
  }

  let selectedValues = $derived(
    multiple ? (Array.isArray(value) ? value : []) : value ? [value] : []
  );

  let allOptions = $derived([...displayOptions, ...options]);

  function labelOf(v: string): string {
    return labelCache[v] || allOptions.find(o => o.value === v)?.label || v;
  }

  let triggerText = $derived(
    multiple
      ? selectedValues.length > 0
        ? `已选择 ${selectedValues.length} 项`
        : placeholder
      : selectedValues.length > 0
        ? labelOf(selectedValues[0])
        : placeholder
  );

  function isSelected(v: string): boolean {
    return selectedValues.includes(v);
  }

  function toggleOpen() {
    if (disabled) return;
    open = !open;
    if (open) {
      keyword = '';
      runSearch('');
    }
  }

  async function runSearch(kw: string) {
    loading = true;
    try {
      const res = await searchFn(kw.trim());
      // 合并已选项，避免已选但不在搜索结果中的项丢失展示
      const known = new Map<string, RemoteSelectOption>();
      for (const o of displayOptions) known.set(o.value, o);
      for (const o of res) known.set(o.value, o);
      options = Array.from(known.values());
    } catch (err) {
      console.error('RemoteSearchSelect search failed:', err);
      options = [];
    } finally {
      loading = false;
    }
  }

  function onKeywordInput() {
    clearTimeout(searchTimer);
    searchTimer = setTimeout(() => {
      runSearch(keyword);
    }, 300);
  }

  function selectOption(opt: RemoteSelectOption) {
    labelCache = { ...labelCache, [opt.value]: opt.label };
    if (multiple) {
      if (selectedValues.includes(opt.value)) {
        value = selectedValues.filter(v => v !== opt.value);
      } else {
        value = [...selectedValues, opt.value];
      }
      // 保持面板打开，可继续多选
    } else {
      value = opt.value;
      open = false;
    }
  }

  function removeValue(v: string) {
    if (multiple && Array.isArray(value)) {
      value = value.filter(item => item !== v);
    } else {
      value = '';
    }
  }

  function clearValue(e: MouseEvent) {
    e.stopPropagation();
    e.preventDefault();
    value = multiple ? [] : '';
  }
</script>

<svelte:window onclick={onDocClick} />

<div bind:this={containerRef} class={cn('space-y-2', className)}>
  {#if label}
    <Label>
      {label}
      {#if required}
        <span class="text-destructive">*</span>
      {/if}
    </Label>
  {/if}

  <!-- 多选已选标签 -->
  {#if multiple && selectedValues.length > 0}
    <div class="flex flex-wrap gap-1">
      {#each selectedValues as v}
        <Badge variant="secondary" class="gap-1">
          {labelOf(v)}
          {#if !disabled}
            <button type="button" aria-label="移除" onclick={() => removeValue(v)} class="ml-1 rounded-full hover:bg-muted">
              <X class="h-3 w-3" />
            </button>
          {/if}
        </Badge>
      {/each}
    </div>
  {/if}

  <div class="relative">
    <button
      type="button"
      {id}
      onclick={toggleOpen}
      disabled={disabled}
      data-placeholder={selectedValues.length === 0 ? true : undefined}
      class="border-input data-[placeholder]:text-muted-foreground [&_svg]:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 flex w-full items-center justify-between gap-2 rounded-md border bg-transparent px-3 py-2 text-sm whitespace-nowrap shadow-xs transition-[color,box-shadow] outline-none select-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 h-9 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg]:size-4"
    >
      <span class="truncate">{triggerText}</span>
      <ChevronDown class="size-4 opacity-50" />
    </button>

    {#if value && !disabled && !required && selectedValues.length > 0}
      <button
        type="button"
        aria-label="清除选择"
        onclick={clearValue}
        class="absolute right-8 top-1/2 -translate-y-1/2 rounded-sm p-0.5 text-muted-foreground hover:bg-muted hover:text-foreground"
      >
        <X class="h-3.5 w-3.5" />
      </button>
    {/if}

    {#if open}
      <div class="absolute z-50 mt-1 w-full">
        <div class="rounded-md border bg-popover text-popover-foreground shadow-md">
          <div class="flex items-center gap-2 border-b px-3 py-2">
            <Search class="h-4 w-4 shrink-0 text-muted-foreground" />
            <input
              type="text"
              bind:value={keyword}
              oninput={onKeywordInput}
              placeholder={searchPlaceholder}
              class="h-8 w-full bg-transparent text-sm outline-none placeholder:text-muted-foreground"
            />
          </div>
          <div class="max-h-64 overflow-auto p-1">
            {#if loading}
              <div class="flex items-center justify-center gap-2 px-3 py-4 text-sm text-muted-foreground">
                <Loader2 class="h-4 w-4 animate-spin" />
                加载中...
              </div>
            {:else if options.length > 0}
              {#each options as opt}
                <button
                  type="button"
                  onclick={() => selectOption(opt)}
                  class="flex w-full items-center justify-between rounded-sm px-2 py-1.5 text-sm outline-none select-none hover:bg-accent hover:text-accent-foreground focus-visible:bg-accent"
                >
                  <span class="truncate">{opt.label}</span>
                  {#if isSelected(opt.value)}
                    <Check class="h-4 w-4 shrink-0" />
                  {/if}
                </button>
              {/each}
            {:else}
              <div class="px-3 py-4 text-center text-sm text-muted-foreground">无匹配结果</div>
            {/if}
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>
