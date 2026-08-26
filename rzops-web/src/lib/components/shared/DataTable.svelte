<script lang="ts" generics="T">
  import * as Table from '$lib/ui/table';
  import { Button } from '$lib/ui/button';

  interface Column {
    key: string;
    label: string;
    class?: string;
    /** Optional value map for resolving IDs to display names */
    valueMap?: Record<string, string>;
    /** Optional render function for custom display */
    render?: (value: unknown, item: T) => string;
    /** Optional custom display text (takes precedence) */
    display?: (item: T) => string;
    /** Optional link href; when present the cell renders as a link (null = plain text) */
    link?: (item: T) => string | null;
  }

  let {
    columns,
    data,
    loading = false,
    onEdit,
    onDelete,
    editLabel = '编辑',
  }: {
    columns: Column[];
    data: T[];
    loading?: boolean;
    onEdit?: (item: T) => void;
    onDelete?: (item: T) => void;
    editLabel?: string;
  } = $props();

  let hasActions = $derived(onEdit || onDelete);

  function getValue(item: T, key: string): unknown {
    return (item as Record<string, unknown>)[key];
  }

  function displayValue(item: T, col: Column): string {
    const raw = getValue(item, col.key);

    // Use custom render function if provided
    if (col.render) {
      return col.render(raw, item);
    }

    // Use value map if provided
    if (col.valueMap && raw) {
      return col.valueMap[String(raw)] || String(raw);
    }

    // Handle arrays
    if (Array.isArray(raw)) {
      return raw.join(', ') || '-';
    }

    // Handle null/undefined
    if (raw === null || raw === undefined) {
      return '-';
    }

    return String(raw);
  }
</script>

<div class="rounded-md border">
  <Table.Root>
    <Table.Header>
      <Table.Row>
        {#each columns as col}
          <Table.Head class={col.class}>{col.label}</Table.Head>
        {/each}
        {#if hasActions}
          <Table.Head class="w-[100px]">操作</Table.Head>
        {/if}
      </Table.Row>
    </Table.Header>
    <Table.Body>
      {#if loading}
        <Table.Row>
          <Table.Cell colspan={columns.length + (hasActions ? 1 : 0)} class="h-24 text-center text-muted-foreground">
            加载中...
          </Table.Cell>
        </Table.Row>
      {:else if data.length === 0}
        <Table.Row>
          <Table.Cell colspan={columns.length + (hasActions ? 1 : 0)} class="h-24 text-center text-muted-foreground">
            暂无数据
          </Table.Cell>
        </Table.Row>
      {:else}
        {#each data as item}
          <Table.Row>
            {#each columns as col}
              <Table.Cell class={col.class}>
                {#if col.link}
                  {@const href = col.link(item)}
                  {#if href}
                    <a
                      href={href}
                      class="text-primary hover:underline"
                      onclick={(e) => e.stopPropagation()}
                    >
                      {col.display ? col.display(item) : displayValue(item, col)}
                    </a>
                  {:else}
                    {col.display ? col.display(item) : displayValue(item, col)}
                  {/if}
                {:else}
                  {col.display ? col.display(item) : displayValue(item, col)}
                {/if}
              </Table.Cell>
            {/each}
            {#if hasActions}
              <Table.Cell>
                <div class="flex gap-1">
                  {#if onEdit}
                    <Button variant="ghost" size="sm" onclick={() => onEdit(item)}>
                      {editLabel}
                    </Button>
                  {/if}
                  {#if onDelete}
                    <Button variant="ghost" size="sm" onclick={() => onDelete(item)}>
                      删除
                    </Button>
                  {/if}
                </div>
              </Table.Cell>
            {/if}
          </Table.Row>
        {/each}
      {/if}
    </Table.Body>
  </Table.Root>
</div>
