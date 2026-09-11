<script lang="ts">
  interface BreadcrumbItem {
    label: string;
    href?: string;
  }

  let { items }: { items: BreadcrumbItem[] } = $props();
</script>

<nav class="flex items-center space-x-1 text-sm text-muted-foreground">
  <a href="/" class="hover:text-foreground">首页</a>
  {#each items as item, i}
    <span>/</span>
    {#if item.href && i < items.length - 1}
      <a href={item.href} class="hover:text-foreground">{item.label}</a>
    {:else}
      <span class="text-foreground">{item.label}</span>
    {/if}
  {/each}
</nav>
