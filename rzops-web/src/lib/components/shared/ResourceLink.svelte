<script lang="ts">
  interface Props {
    id?: string | null;
    nameMap: Record<string, string>;
    href: string;
    label?: string;
  }

  let { id, nameMap, href, label = '已删除' }: Props = $props();
</script>

{#if id}
  {#if nameMap[id]}
    <a href={href} class="text-primary hover:underline">{nameMap[id]}</a>
  {:else}
    <span class="text-muted-foreground italic">{label}</span>
  {/if}
{:else}
  -
{/if}
