<script lang="ts">
  import { page } from '$app/stores';
  import { browser } from '$app/environment';
  import { cn } from '$lib/utils';
  import { auth } from '$lib/stores/auth';
  import * as Collapsible from '$lib/ui/collapsible';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';
  import LayoutGrid from '@lucide/svelte/icons/layout-grid';
  import Boxes from '@lucide/svelte/icons/boxes';
  import Globe from '@lucide/svelte/icons/globe';
  import AppWindow from '@lucide/svelte/icons/app-window';
  import Wrench from '@lucide/svelte/icons/wrench';
  import Settings from '@lucide/svelte/icons/settings';
  import ShieldCheck from '@lucide/svelte/icons/shield-check';
  import ChevronsLeft from '@lucide/svelte/icons/chevrons-left';
  import ChevronsRight from '@lucide/svelte/icons/chevrons-right';
  import X from '@lucide/svelte/icons/x';
  import { onMount } from 'svelte';

  let {
    mobileOpen = false,
    onMobileClose = () => {},
  }: {
    mobileOpen?: boolean;
    onMobileClose?: () => void;
  } = $props();

  const groupIcons: Record<string, typeof Boxes> = {
    基础设施: Boxes,
    网络: Globe,
    应用: AppWindow,
    运维: Wrench,
    管理: Settings,
    审计: ShieldCheck,
  };

  interface NavItem {
    href: string;
    label: string;
  }

  interface NavGroup {
    label: string;
    items: NavItem[];
  }

  const navGroups: NavGroup[] = [
    {
      label: '基础设施',
      items: [
        { href: '/servers', label: '服务器' },
        { href: '/datacenters', label: '数据中心' },
        { href: '/providers', label: '供应商' },
      ],
    },
    {
      label: '网络',
      items: [
        { href: '/domains', label: '域名' },
        { href: '/certificates', label: '证书' },
        { href: '/server-ips', label: '服务器IP' },
        { href: '/server-ports', label: '服务器端口' },
      ],
    },
    {
      label: '应用',
      items: [
        { href: '/ops-sites', label: '站点' },
        { href: '/database-instances', label: '数据库实例' },
      ],
    },
    {
      label: '运维',
      items: [
        { href: '/backup-plans', label: '备份计划' },
        { href: '/monitor-targets', label: '监控目标' },
      ],
    },
    {
      label: '管理',
      items: [
        { href: '/attachments', label: '附件' },
        { href: '/dicts', label: '字典管理' },
      ],
    },
    {
      label: '审计',
      items: [
        { href: '/audit-logs', label: '审计日志' },
        { href: '/change-records', label: '变更记录' },
      ],
    },
  ];

  // Track which groups are expanded (persisted)
  let expandedGroups = $state<Record<string, boolean>>({});
  // Sidebar collapsed state (persisted)
  let collapsed = $state(false);
  let groupsInitialized = $state(false);

  onMount(() => {
    collapsed = localStorage.getItem('rzops-sidebar-collapsed') === '1';
    let saved: Record<string, boolean> = {};
    try {
      const raw = localStorage.getItem('rzops-sidebar-groups');
      if (raw) saved = JSON.parse(raw);
    } catch { /* ignore */ }
    // 初始化所有分组状态，默认展开当前页面所在分组
    const currentPath = $page.url.pathname;
    navGroups.forEach(group => {
      if (saved[group.label] !== undefined) {
        expandedGroups[group.label] = saved[group.label];
      } else {
        expandedGroups[group.label] = group.items.some(item => currentPath.startsWith(item.href));
      }
    });
    groupsInitialized = true;
    saveGroups();
  });

  function toggleCollapsed() {
    collapsed = !collapsed;
    if (browser) {
      localStorage.setItem('rzops-sidebar-collapsed', collapsed ? '1' : '0');
    }
  }

  function saveGroups() {
    if (browser && groupsInitialized) {
      localStorage.setItem('rzops-sidebar-groups', JSON.stringify(expandedGroups));
    }
  }

  // Audit group (and any future admin-only entries) visible only to superusers.
  const visibleGroups = $derived(
    $auth.user?.is_superuser ? navGroups : navGroups.filter((g) => g.label !== '审计')
  );

  function isActive(href: string, currentPath: string) {
    if (href === '/') return currentPath === '/';
    return currentPath.startsWith(href);
  }

  function isGroupActive(group: NavGroup, currentPath: string) {
    return group.items.some(item => isActive(item.href, currentPath));
  }

  // 分组展开状态已在 onMount 中初始化并持久化，无需自动展开
</script>

<aside
  class={cn(
    'flex h-screen flex-col border-r border-sidebar-border bg-sidebar-background text-sidebar-foreground transition-[width] duration-300 ease-in-out',
    'fixed inset-y-0 left-0 z-40 w-64 transform transition-transform duration-300 ease-in-out md:static md:translate-x-0',
    collapsed ? 'md:w-16' : 'md:w-64',
    mobileOpen ? 'translate-x-0' : '-translate-x-full'
  )}
>
  <div class="border-b border-sidebar-border p-4">
    <div class="flex items-center justify-between">
      <a href="/" class="flex items-center gap-3" onclick={onMobileClose}>
        <div class="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-gradient-to-br from-sky-500 to-blue-700 shadow-md">
          <LayoutGrid class="h-5 w-5 text-white" />
        </div>
        {#if !collapsed}
          <div class="min-w-0">
            <h1 class="text-lg font-bold tracking-tight">RzOps</h1>
            <p class="text-xs text-sidebar-foreground/85">CMDB 管理平台</p>
          </div>
        {/if}
      </a>
      <!-- 移动端关闭按钮 -->
      <button
        class="rounded-md p-1 text-sidebar-foreground/80 hover:bg-sidebar-accent md:hidden"
        onclick={onMobileClose}
      >
        <X class="h-5 w-5" />
      </button>
    </div>
  </div>

  <nav class="flex-1 space-y-1 overflow-y-auto overflow-x-hidden px-2 py-3">
    <!-- Dashboard -->
    <a
      href="/"
      onclick={onMobileClose}
      class={cn(
        'flex items-center rounded-md text-sm transition-colors hover:bg-sidebar-accent',
        collapsed ? 'justify-center px-2 py-2' : 'gap-2.5 px-3 py-2',
        $page.url.pathname === '/'
          ? 'bg-sidebar-accent font-medium text-sidebar-accent-foreground shadow-sm'
          : 'text-sidebar-foreground/95'
      )}
      title={collapsed ? 'Dashboard' : undefined}
    >
      <LayoutGrid class={cn('h-4 w-4 shrink-0', collapsed ? '' : 'opacity-80')} />
      {#if !collapsed}Dashboard{/if}
    </a>

    {#each visibleGroups as group}
      {@const hasActive = isGroupActive(group, $page.url.pathname)}
      {@const GroupIcon = groupIcons[group.label]}

      {#if collapsed}
        <!-- 折叠模式：图标按钮 + hover 浮出子菜单 -->
        <div class="group relative">
          <button
            class={cn(
              'flex w-full items-center justify-center rounded-md px-2 py-2 transition-colors hover:bg-sidebar-accent',
              hasActive ? 'bg-sidebar-accent text-sidebar-accent-foreground' : 'text-sidebar-foreground/95'
            )}
            title={group.label}
          >
            {#if GroupIcon}
              <GroupIcon class="h-4 w-4 shrink-0" />
            {/if}
          </button>
          <div class="invisible absolute left-full top-0 z-50 ml-2 min-w-44 rounded-lg border border-sidebar-border bg-sidebar-background p-2 opacity-0 shadow-xl transition-all duration-150 group-hover:visible group-hover:opacity-100">
            <span class="block px-2 pb-1 pt-1 text-xs font-semibold tracking-wide text-sidebar-foreground/85">
              {group.label}
            </span>
            <div class="space-y-0.5">
              {#each group.items as item}
                <a
                  href={item.href}
                  class={cn(
                    'block rounded-md px-2 py-1.5 text-sm transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground',
                    isActive(item.href, $page.url.pathname)
                      ? 'bg-sidebar-accent font-medium text-sidebar-accent-foreground'
                      : 'text-sidebar-foreground/90'
                  )}
                >
                  {item.label}
                </a>
              {/each}
            </div>
          </div>
        </div>
      {:else}
        <!-- 展开模式：可折叠分组 -->
        <Collapsible.Root
          open={!!expandedGroups[group.label]}
          onOpenChange={(open) => { expandedGroups[group.label] = open; saveGroups(); }}
        >
          <Collapsible.Trigger
            class={cn(
              'flex w-full items-center gap-2.5 rounded-md px-3 py-2 text-sm transition-colors hover:bg-sidebar-accent',
              hasActive
                ? 'font-medium text-sidebar-primary'
                : 'text-sidebar-foreground/95'
            )}
          >
            {#if GroupIcon}
              <GroupIcon class="h-4 w-4 shrink-0 opacity-80" />
            {/if}
            <span class="flex-1 text-left font-medium">{group.label}</span>
            <ChevronDown class="h-4 w-4 opacity-70 transition-transform [[data-state=open]>&]:rotate-180" />
          </Collapsible.Trigger>

          <Collapsible.Content>
            <div class="ml-3 space-y-1 border-l border-sidebar-border pl-2">
              {#each group.items as item}
                <a
                  href={item.href}
                  onclick={onMobileClose}
                  class={cn(
                    'relative flex items-center rounded-md px-3 py-1.5 text-sm transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground',
                    isActive(item.href, $page.url.pathname)
                      ? 'bg-sidebar-accent font-medium text-sidebar-accent-foreground'
                      : 'text-sidebar-foreground/90'
                  )}
                >
                  {#if isActive(item.href, $page.url.pathname)}
                    <span class="absolute -left-2 top-1/2 h-4 w-1 -translate-y-1/2 rounded-full bg-sidebar-primary"></span>
                  {/if}
                  {item.label}
                </a>
              {/each}
            </div>
          </Collapsible.Content>
        </Collapsible.Root>
      {/if}
    {/each}
  </nav>

  <!-- 折叠/展开按钮（仅桌面端显示） -->
  <div class="hidden border-t border-sidebar-border p-2 md:block">
    <button
      class={cn(
        'flex w-full items-center rounded-md py-2 text-sm text-sidebar-foreground/90 transition-colors hover:bg-sidebar-accent',
        collapsed ? 'justify-center' : 'justify-between px-3'
      )}
      onclick={toggleCollapsed}
      title={collapsed ? '展开菜单' : '折叠菜单'}
    >
      {#if collapsed}
        <ChevronsRight class="h-4 w-4" />
      {:else}
        <span class="text-sidebar-foreground/95">收起菜单</span>
        <ChevronsLeft class="h-4 w-4 opacity-80" />
      {/if}
    </button>
  </div>
</aside>
