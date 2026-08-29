<script lang="ts">
  import * as Select from '$lib/ui/select';
  import { Label } from '$lib/ui/label';
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
    class: className = '',
  }: {
    label: string;
    value?: string | undefined;
    options: SelectOption[];
    placeholder?: string;
    required?: boolean;
    disabled?: boolean;
    class?: string;
  } = $props();

  let selectedLabel = $derived(
    value ? (options.find(opt => opt.value === value)?.label || placeholder) : placeholder
  );

  // 非必填且已有值时允许一键清除选择
  function clearValue(e: MouseEvent) {
    e.stopPropagation();
    e.preventDefault();
    value = '';
  }
</script>

<div class={cn('space-y-2', className)}>
  {#if label}
    <Label>
      {label}
      {#if required}
        <span class="text-destructive">*</span>
      {/if}
    </Label>
  {/if}
  <div class="relative">
    <Select.Root type="single" bind:value={value} {disabled}>
      <Select.Trigger class={cn('w-full', value && !required ? 'pr-9' : '')}>
        {selectedLabel}
      </Select.Trigger>
      <Select.Content>
        {#each options as option}
          <Select.Item value={option.value} label={option.label}>
            {option.label}
          </Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>
    {#if value && !disabled && !required}
      <button
        type="button"
        aria-label="清除选择"
        onclick={clearValue}
        class="absolute right-8 top-1/2 -translate-y-1/2 rounded-sm p-0.5 text-muted-foreground hover:bg-muted hover:text-foreground"
      >
        <X class="h-3.5 w-3.5" />
      </button>
    {/if}
  </div>
</div>
