<script lang="ts">
  import { goto } from '$app/navigation';
  import { authApi } from '$lib/api/auth';
  import { auth } from '$lib/stores/auth';
  import { Button } from '$lib/ui/button';
  import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/ui/card';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';
  import LayoutGrid from '@lucide/svelte/icons/layout-grid';

  let email = $state('');
  let password = $state('');
  let error = $state('');
  let loading = $state(false);

  async function handleSubmit(e: Event) {
    e.preventDefault();
    error = '';
    loading = true;

    try {
      const response = await authApi.login({ email, password });
      auth.login(response.access_token, response.user);
      goto('/');
    } catch (err) {
      error = err instanceof Error ? err.message : '登录失败';
    } finally {
      loading = false;
    }
  }
</script>

<div class="relative flex min-h-screen items-center justify-center overflow-hidden bg-slate-50 p-4">
  <!-- 背景装饰 -->
  <div class="pointer-events-none absolute inset-0">
    <div class="absolute -left-24 -top-24 h-72 w-72 rounded-full bg-sky-200/50 blur-3xl"></div>
    <div class="absolute -bottom-32 -right-24 h-80 w-80 rounded-full bg-blue-300/40 blur-3xl"></div>
    <div class="absolute left-1/2 top-1/3 h-56 w-56 -translate-x-1/2 rounded-full bg-indigo-200/30 blur-3xl"></div>
  </div>

  <Card class="relative w-full max-w-md border-slate-200/80 shadow-2xl shadow-slate-300/40">
    <CardHeader class="text-center">
      <div class="mx-auto mb-3 flex h-14 w-14 items-center justify-center rounded-2xl bg-gradient-to-br from-sky-500 to-blue-700 shadow-lg shadow-blue-500/30">
        <LayoutGrid class="h-7 w-7 text-white" />
      </div>
      <CardTitle class="text-2xl font-bold tracking-tight">RzOps</CardTitle>
      <CardDescription>CMDB 运维资产管理平台</CardDescription>
    </CardHeader>
    <CardContent>
      <form onsubmit={handleSubmit} class="space-y-4">
        {#if error}
          <div class="rounded-md bg-destructive/10 p-3 text-sm text-destructive">
            {error}
          </div>
        {/if}

        <div class="space-y-2">
          <Label for="email">邮箱</Label>
          <Input
            id="email"
            type="email"
            placeholder="admin@rzops.local"
            bind:value={email}
            required
            disabled={loading}
          />
        </div>

        <div class="space-y-2">
          <Label for="password">密码</Label>
          <Input
            id="password"
            type="password"
            placeholder="请输入密码"
            bind:value={password}
            required
            disabled={loading}
          />
        </div>

        <Button type="submit" class="w-full" disabled={loading}>
          {loading ? '登录中...' : '登录'}
        </Button>
      </form>
    </CardContent>
  </Card>
</div>
