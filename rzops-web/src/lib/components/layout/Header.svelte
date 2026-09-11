<script lang="ts">
  import { goto } from '$app/navigation';
  import { auth } from '$lib/stores/auth';
  import { Button } from '$lib/ui/button';
  import Menu from '@lucide/svelte/icons/menu';

  let {
    children,
    onMenuToggle = () => {},
  }: {
    children?: import('svelte').Snippet;
    onMenuToggle?: () => void;
  } = $props();
</script>

<header class="flex h-14 items-center justify-between border-b border-border bg-card/70 px-4 backdrop-blur md:px-6">
  <div class="flex items-center gap-3">
    <!-- 移动端汉堡菜单按钮 -->
    <button
      class="rounded-md p-2 text-muted-foreground hover:bg-accent hover:text-accent-foreground md:hidden"
      onclick={onMenuToggle}
      aria-label="打开菜单"
    >
      <Menu class="h-5 w-5" />
    </button>
    <div class="text-sm font-medium text-muted-foreground">
      {#if children}
        {@render children()}
      {/if}
    </div>
  </div>

  <div class="flex items-center gap-2 md:gap-4">
    <div class="text-right">
      <p class="text-sm font-medium">{$auth.user?.full_name || '用户'}</p>
      <p class="hidden text-xs text-muted-foreground sm:block">{$auth.user?.email || ''}</p>
    </div>
    <Button
      variant="ghost"
      size="sm"
      onclick={() => { auth.logout(); goto('/login'); }}
    >
      退出
    </Button>
  </div>
</header>
