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

  onMount(async () => {
    // 预加载数据字典选项（幂等，浏览器端执行）
    loadAllDicts();
    // Skip auth check for login page
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
    <Sidebar />
    <div class="flex flex-1 flex-col overflow-hidden">
      <Header />
      <main class="flex-1 overflow-auto p-6">
        {@render children()}
      </main>
    </div>
  </div>
{/if}
