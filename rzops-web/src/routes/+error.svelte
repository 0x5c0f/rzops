<script lang="ts">
  import { page } from '$app/stores';
</script>

<div class="flex min-h-[60vh] flex-col items-center justify-center gap-2">
  <div class="text-6xl font-bold text-muted-foreground/30">
    {$page.status ?? 404}
  </div>
  <h1 class="text-xl font-semibold">
    {#if $page.status === 404}
      页面不存在
    {:else if $page.status === 403}
      无访问权限
    {:else}
      页面出错了
    {/if}
  </h1>
  <p class="text-sm text-muted-foreground">
    {#if $page.status === 404}
      您访问的页面不存在或已被移除，请检查地址是否正确。
    {:else if $page.status === 403}
      您没有访问该页面的权限，请联系管理员。
    {:else}
      抱歉，页面加载时发生了错误，请稍后重试。
    {/if}
  </p>
  <a
    href="/"
    class="mt-2 rounded-md border px-4 py-2 text-sm hover:bg-accent">返回首页</a
  >
</div>
