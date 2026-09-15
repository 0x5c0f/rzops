<script lang="ts">
  import { Label } from '$lib/ui/label';
  import CalendarIcon from '@lucide/svelte/icons/calendar';

  let {
    id = `date-${Math.random().toString(36).slice(2, 8)}`,
    label,
    value = $bindable(),
    min,
    max,
    required = false,
    hint,
    class: className,
  }: {
    id?: string;
    label?: string;
    value?: string;
    min?: string;
    max?: string;
    required?: boolean;
    hint?: string;
    class?: string;
  } = $props();

  let inputRef = $state<HTMLInputElement | null>(null);

  function openPicker() {
    const el = inputRef;
    if (!el) return;
    if (typeof el.showPicker === 'function') {
      try {
        el.showPicker();
      } catch {
        el.focus();
      }
    } else {
      el.focus();
    }
  }
</script>

<div class="space-y-2 {className ?? ''}">
  {#if label}
    <Label for={id}>{label}{#if required}<span class="text-destructive">*</span>{/if}</Label>
  {/if}
  <div class="relative">
    <input
      bind:this={inputRef}
      id={id}
      type="date"
      bind:value
      {min}
      {max}
      {required}
      class="border-input bg-background selection:bg-primary dark:bg-input/30 selection:text-primary-foreground ring-offset-background placeholder:text-muted-foreground flex h-9 w-full min-w-0 rounded-md border px-3 py-1 text-sm shadow-xs transition-[color,box-shadow] outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 [&::-webkit-calendar-picker-indicator]:m-0 [&::-webkit-calendar-picker-indicator]:p-0 [&::-webkit-calendar-picker-indicator]:opacity-0"
    />
    <button
      type="button"
      onclick={openPicker}
      tabindex="-1"
      aria-label="选择日期"
      class="absolute inset-y-0 right-0 flex w-9 cursor-pointer items-center justify-center border-l border-input/60 text-muted-foreground transition-colors hover:text-foreground"
    >
      <CalendarIcon class="h-4 w-4" />
    </button>
  </div>
  {#if hint}
    <p class="text-xs text-muted-foreground">{hint}</p>
  {/if}
</div>
