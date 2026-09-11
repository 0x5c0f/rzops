<script lang="ts" generics="T">
  import * as Card from '$lib/ui/card';
  import Pagination from '$lib/components/shared/Pagination.svelte';
  import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
  import { cn } from '$lib/utils';
  import type { Snippet } from 'svelte';

  interface Column<T> {
    key: string;
    label: string;
    class?: string;
    /** 单元格自定义渲染：返回文本（无转义风险由调用方保证），或 null 显示 - */
    render?: (item: T) => string | null;
    /** 渲染为链接（优先级高于 render） */
    link?: (item: T) => string | null;
    /** 状态徽章：status 为状态值（读字典色），label 可选覆盖 */
    badge?: (item: T) => { status: string; label?: string } | null;
    /** 完全自定义的单元格 snippet（最高优先级），可通过 {item} 渲染任意内容 */
    cell?: Snippet<[T]>;
  }

  let {
    title,
    description,
    items,
    columns,
    emptyText = '暂无关联数据',
    defaultPerPage = 10,
    pageSizeOptions = [10, 20, 50],
    children,
  }: {
    title: string;
    description?: string;
    items: T[];
    columns: Column<T>[];
    emptyText?: string;
    defaultPerPage?: number;
    pageSizeOptions?: number[];
    /** 标题右侧的自定义内容（如操作按钮） */
    children?: Snippet;
  } = $props();

  let page = $state(1);
  let perPage = $state(defaultPerPage);

  let total = $derived(items.length);
  let totalPages = $derived(Math.max(1, Math.ceil(total / perPage)));
  let pagedItems = $derived(items.slice((page - 1) * perPage, page * perPage));

  $effect(() => {
    if (page > totalPages) page = totalPages;
  });
</script>

<Card.Root>
  <Card.Header class="flex items-start justify-between gap-4">
    <div>
      <Card.Title>{title}</Card.Title>
      {#if description}
        <Card.Description class="mt-1">{description}</Card.Description>
      {/if}
    </div>
    {#if children}
      <div class="shrink-0">
        {@render children()}
      </div>
    {/if}
  </Card.Header>
  <Card.Content>
    {#if total === 0}
      <div class="flex items-center justify-center rounded-lg border border-dashed py-10 text-sm text-muted-foreground">
        {emptyText}
      </div>
    {:else}
      <div class="overflow-x-auto">
        <table class="w-full text-sm">
          <thead>
            <tr class="border-b bg-muted/50">
              {#each columns as col}
                <th class={cn('px-3 py-2 text-left font-medium text-muted-foreground', col.class)}>{col.label}</th>
              {/each}
            </tr>
          </thead>
          <tbody>
            {#each pagedItems as item (item)}
              <tr class="border-b last:border-0 hover:bg-muted/40">
                {#each columns as col}
                  <td class={cn('px-3 py-2 align-middle', col.class)}>
                    {#if col.cell}
                      {@render col.cell(item)}
                    {:else if col.link && col.link(item)}
                      <div class="flex items-center gap-2">
                        <a href={col.link(item)} class="text-primary hover:underline">
                          {col.render ? col.render(item) : item[col.key as keyof T]?.toString()}
                        </a>
                        {#if col.badge && col.badge(item)}
                          <StatusBadge status={col.badge(item)!.status} label={col.badge(item)!.label} />
                        {/if}
                      </div>
                    {:else if col.badge && col.badge(item)}
                      <StatusBadge status={col.badge(item)!.status} label={col.badge(item)!.label} />
                    {:else if col.render}
                      {col.render(item)}
                    {:else if item[col.key as keyof T] !== undefined && item[col.key as keyof T] !== null}
                      {item[col.key as keyof T]?.toString()}
                    {:else}
                      -
                    {/if}
                  </td>
                {/each}
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
      {#if total > perPage}
        <Pagination
          {page}
          {perPage}
          {total}
          pageSizeOptions={pageSizeOptions}
          onPageChange={(p) => (page = p)}
          onPerPageChange={(p) => (perPage = p)}
          class="mt-4"
        />
      {/if}
    {/if}
  </Card.Content>
</Card.Root>
