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
    class: className = '',
  }: {
    label: string;
    value?: string[] | undefined;
    options: SelectOption[];
    placeholder?: string;
    required?: boolean;
    disabled?: boolean;
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

  let selectedLabels = $derived(
    currentValue.map(v => options.find(opt => opt.value === v)?.label || v)
  );
</script>

<div class={cn('space-y-2', className)}>
  <Label>
    {label}
    {#if required}
      <span class="text-destructive">*</span>
    {/if}
  </Label>

  <!-- Selected items -->
  {#if currentValue.length > 0}
    <div class="flex flex-wrap gap-1">
      {#each currentValue as v, i}
        <Badge variant="secondary" class="gap-1">
          {selectedLabels[i]}
          {#if !disabled}
            <button
              type="button"
              onclick={() => removeValue(v)}
              class="ml-1 rounded-full hover:bg-muted"
            >
              <X class="h-3 w-3" />
            </button>
          {/if}
        </Badge>
      {/each}
    </div>
  {/if}

  <!-- Select dropdown -->
  <Select.Root type="single" value="" onValueChange={(v) => { if (v) addValue(v); }} {disabled}>
    <Select.Trigger class="w-full">
      {currentValue.length > 0 ? `已选择 ${currentValue.length} 项` : placeholder}
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
