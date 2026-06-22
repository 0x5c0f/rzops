<script lang="ts">
  import { goto } from '$app/navigation';
  import { authApi } from '$lib/api/auth';
  import { auth } from '$lib/stores/auth';
  import { Button } from '$lib/ui/button';
  import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '$lib/ui/card';
  import { Input } from '$lib/ui/input';
  import { Label } from '$lib/ui/label';

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

<div class="flex min-h-screen items-center justify-center bg-muted/50">
  <Card class="w-full max-w-md">
    <CardHeader class="text-center">
      <CardTitle class="text-2xl font-bold">RzOps</CardTitle>
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
