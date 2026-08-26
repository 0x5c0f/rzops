<script lang="ts">
  import { goto } from '$app/navigation';
  import { auth } from '$lib/stores/auth';
  import { Button } from '$lib/ui/button';

  let { children }: { children?: import('svelte').Snippet } = $props();
</script>

<header class="flex h-14 items-center justify-between border-b border-border bg-card/70 px-6 backdrop-blur">
  <div class="text-sm font-medium text-muted-foreground">
    {#if children}
      {@render children()}
    {/if}
  </div>

  <div class="flex items-center gap-4">
    <div class="text-right">
      <p class="text-sm font-medium">{$auth.user?.full_name || '用户'}</p>
      <p class="text-xs text-muted-foreground">{$auth.user?.email || ''}</p>
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
