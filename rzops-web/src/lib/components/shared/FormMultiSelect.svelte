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

  function addValue(v: string) {
    if (!currentValue.includes(v)) {
      value = [...currentValue, v];
    }
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

  <Select.Root type="single" value="" onValueChange={(v) => { if (v) addValue(v); }} {disabled}>
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
                  onclick={(e) => {
                    e.stopPropagation();
                    e.preventDefault();
                    removeValue(item.value);
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
          disabled={currentValue.includes(option.value)}
        >
          {option.label}
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Root>
</div>
