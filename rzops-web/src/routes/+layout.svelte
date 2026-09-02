<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { auth } from '$lib/stores/auth';
  import { authApi } from '$lib/api/auth';
  import { loadAllDicts } from '$lib/utils/enum-options';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import Header from '$lib/components/layout/Header.svelte';
  import '../app.css';

  let { children } = $props();

  let loading = $state(true);
  let initialized = $state(false);
  let mobileSidebarOpen = $state(false);

  function toggleMobileSidebar() {
    mobileSidebarOpen = !mobileSidebarOpen;
  }

  function closeMobileSidebar() {
    mobileSidebarOpen = false;
  }

  onMount(async () => {
    // 登录页不加载字典（字典接口需认证，未登录请求会 401 导致整页跳转死循环）
    if ($page.url.pathname === '/login') {
      loading = false;
      initialized = true;
      return;
    }

    // Initialize auth from localStorage
    auth.init();

    // Check if we have a token
    const token = localStorage.getItem('token');
    if (!token) {
      goto('/login');
      return;
    }

    // Try to get user info
    try {
      const user = await authApi.me();
      auth.setUser(user);
      // 认证通过后再预加载数据字典选项（幂等）
      loadAllDicts();
      loading = false;
      initialized = true;
    } catch (err) {
      // Token might be invalid, clear and redirect
      console.error('Auth check failed:', err);
      auth.logout();
      goto('/login');
    }
  });
</script>

{#if !initialized}
  <div class="flex min-h-screen items-center justify-center">
    <div class="flex flex-col items-center gap-2">
      <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
      <p class="text-sm text-muted-foreground">加载中...</p>
    </div>
  </div>
{:else if $page.url.pathname === '/login'}
  {@render children()}
{:else}
  <div class="flex h-screen">
    <Sidebar mobileOpen={mobileSidebarOpen} onMobileClose={closeMobileSidebar} />
    <!-- 移动端遮罩层 -->
    {#if mobileSidebarOpen}
      <div
        class="fixed inset-0 z-30 bg-black/50 md:hidden"
        onclick={closeMobileSidebar}
      ></div>
    {/if}
    <div class="flex flex-1 flex-col overflow-hidden">
      <Header onMenuToggle={toggleMobileSidebar} />
      <main class="flex-1 overflow-auto p-4 md:p-6">
        {@render children()}
      </main>
    </div>
  </div>
{/if}
