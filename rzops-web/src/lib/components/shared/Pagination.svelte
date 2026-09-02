<script lang="ts">
  import { Button } from '$lib/ui/button';
  import { cn } from '$lib/utils';

  let {
    page,
    perPage,
    total,
    onPageChange,
    onPerPageChange,
    pageSizeOptions = [10, 20, 50, 100],
    class: className = '',
  }: {
    page: number;
    perPage: number;
    total: number;
    onPageChange: (page: number) => void;
    onPerPageChange: (perPage: number) => void;
    pageSizeOptions?: number[];
    class?: string;
  } = $props();

  let totalPages = $derived(Math.max(1, Math.ceil(total / perPage)));
  let start = $derived(total === 0 ? 0 : (page - 1) * perPage + 1);
  let end = $derived(Math.min(page * perPage, total));
</script>

<div class={cn('flex items-center justify-between text-sm text-muted-foreground', className)}>
  <span>显示 {start}-{end} / 共 {total} 条</span>
  <div class="flex items-center gap-4">
    <div class="flex items-center gap-2">
      <span>每页</span>
      <select
        value={perPage}
        onchange={(e) => onPerPageChange(Number((e.target as HTMLSelectElement).value))}
        class="rounded-md border border-input bg-background px-2 py-1 text-sm focus:outline-none focus:ring-2 focus:ring-ring"
      >
        {#each pageSizeOptions as size}
          <option value={size}>{size}</option>
        {/each}
      </select>
      <span>条</span>
    </div>
    <div class="flex gap-2">
      <Button
        variant="outline"
        size="sm"
        disabled={page <= 1}
        onclick={() => onPageChange(page - 1)}
      >
        上一页
      </Button>
      <span class="flex items-center px-2">{page} / {totalPages}</span>
      <Button
        variant="outline"
        size="sm"
        disabled={page >= totalPages}
        onclick={() => onPageChange(page + 1)}
      >
        下一页
      </Button>
    </div>
  </div>
</div>
