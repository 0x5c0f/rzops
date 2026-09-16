<script lang="ts">
  import * as Select from '$lib/ui/select';
  import { Label } from '$lib/ui/label';
  import { Badge } from '$lib/ui/badge';
  import { cn } from '$lib/utils';
  import X from '@lucide/svelte/icons/x';

  interface SelectOption {
    label: string;
    value: string;
  }

  let {
    label,
    value = $bindable(),
    options,
    placeholder = '请选择',
    required = false,
    disabled = false,
    /** trigger 内最多直接展示的已选标签数，超出折叠为 +N */
    maxDisplay = 2,
    class: className = '',
  }: {
    label: string;
    value?: string[] | undefined;
    options: SelectOption[];
    placeholder?: string;
    required?: boolean;
    disabled?: boolean;
    maxDisplay?: number;
    class?: string;
  } = $props();

  let currentValue = $derived(value ?? []);

  // 上次由 pointerdown/keydown 主动移除的时间戳。
  // pointerdown 移除后 DOM 会立即重排（Badge 左移），随后合成的 click 会落在
  // 下一个"×"上，若不加抑制会导致一次点击连续删除多个值。
  const REMOVE_SUPPRESS_MS = 350;
  let lastBadgeRemoveTs = 0;

  function badgeRemove(v: string) {
    lastBadgeRemoveTs = Date.now();
    removeValue(v);
  }

  function badgeClickRemove(v: string) {
    if (Date.now() - lastBadgeRemoveTs < REMOVE_SUPPRESS_MS) return; // pointerdown 已处理，忽略合成 click
    removeValue(v);
  }

  function removeValue(v: string) {
    value = currentValue.filter(item => item !== v);
  }

  // 前 maxDisplay 个已选项（解析 label），其余折叠为 +N
  let displayItems = $derived(
    currentValue.slice(0, Math.max(0, maxDisplay)).map(v => ({
      value: v,
      label: options.find(opt => opt.value === v)?.label || v,
    }))
  );
  let hiddenCount = $derived(Math.max(0, currentValue.length - maxDisplay));
</script>

<div class={cn('space-y-2', className)}>
  <Label>
    {label}
    {#if required}
      <span class="text-destructive">*</span>
    {/if}
  </Label>

  <Select.Root
    type="multiple"
    value={currentValue}
    onValueChange={(v) => { value = v; }}
    {disabled}
  >
    <Select.Trigger class="w-full flex-wrap whitespace-normal">
      {#if currentValue.length > 0}
        <span class="flex flex-wrap items-center gap-1">
          {#each displayItems as item}
            <Badge variant="secondary" class="gap-1 px-1.5 py-0 text-xs">
              {item.label}
              {#if !disabled}
                <span
                  role="button"
                  tabindex="-1"
                  aria-label={`移除 ${item.label}`}
                  onpointerdown={(e) => {
                    // bits-ui Select.Trigger 在 pointerdown 阶段即打开浮层，
                    // 若不在此拦截，popover 弹出会覆盖/吞掉后续 click，导致移除按钮失效。
                    e.stopPropagation();
                    e.preventDefault();
                    badgeRemove(item.value);
                  }}
                  onclick={(e) => {
                    e.stopPropagation();
                    e.preventDefault();
                    badgeClickRemove(item.value);
                  }}
                  onkeydown={(e) => {
                    if (e.key === 'Enter' || e.key === ' ') {
                      e.stopPropagation();
                      e.preventDefault();
                      badgeRemove(item.value);
                    }
                  }}
                  class="ml-0.5 cursor-pointer rounded-full hover:bg-muted"
                >
                  <X class="h-3 w-3" />
                </span>
              {/if}
            </Badge>
          {/each}
          {#if hiddenCount > 0}
            <span class="text-xs text-muted-foreground">+{hiddenCount}</span>
          {/if}
        </span>
      {:else}
        {placeholder}
      {/if}
    </Select.Trigger>
    <Select.Content>
      {#each options as option}
        <Select.Item
          value={option.value}
          label={option.label}
          class={currentValue.includes(option.value) ? 'text-primary' : ''}
        >
          <span class="flex w-full items-center justify-between gap-2">
            {option.label}
            {#if currentValue.includes(option.value)}
              <span class="text-xs text-muted-foreground">已选（点击取消）</span>
            {/if}
          </span>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>
