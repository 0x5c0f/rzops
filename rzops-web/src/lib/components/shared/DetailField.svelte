<script lang="ts">
  let {
    label,
    value,
    href,
    mono = false,
  }: {
    label: string;
    value: string | null | undefined;
    href?: string | null;
    mono?: boolean;
  } = $props();
</script>

<div class="flex justify-between py-1.5">
  <dt class="text-sm text-muted-foreground">{label}</dt>
  <dd class="text-sm {mono ? 'font-mono' : ''}">
    {#if href}
      <a {href} class="text-primary hover:underline">{value || '-'}</a>
    {:else}
      {value || '-'}
    {/if}
  </dd>
</div>
