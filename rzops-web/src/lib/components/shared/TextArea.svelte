<script lang="ts">
  import { Label } from '$lib/ui/label';

  let {
    id = `text-${Math.random().toString(36).slice(2, 8)}`,
    label,
    value = $bindable(),
    rows = 3,
    required = false,
    placeholder,
    hint,
    maxlength,
    class: className,
  }: {
    id?: string;
    label?: string;
    value?: string;
    rows?: number;
    required?: boolean;
    placeholder?: string;
    hint?: string;
    maxlength?: number;
    class?: string;
  } = $props();
</script>

<div class="space-y-2 {className ?? ''}">
  {#if label}
    <Label for={id}>{label}{required ? ' *' : ''}</Label>
  {/if}
  <textarea
    id={id}
    rows={rows}
    bind:value
    {required}
    {placeholder}
    {maxlength}
    class="border-input bg-background selection:bg-primary dark:bg-input/30 selection:text-primary-foreground ring-offset-background placeholder:text-muted-foreground flex w-full min-w-0 resize-y rounded-md border px-3 py-2 text-sm leading-6 shadow-xs transition-[color,box-shadow] outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive disabled:cursor-not-allowed disabled:opacity-50"
  ></textarea>
  {#if hint}
    <p class="text-xs text-muted-foreground">{hint}</p>
  {/if}
</div>
