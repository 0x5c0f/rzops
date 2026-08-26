<script lang="ts">
  import * as Select from '$lib/ui/select';
  import { Label } from '$lib/ui/label';
  import { cn } from '$lib/utils';

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
</script>

<div class={cn('space-y-2', className)}>
  <Label>
    {label}
    {#if required}
      <span class="text-destructive">*</span>
    {/if}
  </Label>
  <Select.Root type="single" bind:value={value} {disabled}>
    <Select.Trigger class="w-full">
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
</div>
