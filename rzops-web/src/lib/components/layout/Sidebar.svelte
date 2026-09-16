<script lang="ts">
  import { page } from '$app/stores';
  import { browser } from '$app/environment';
  import { cn } from '$lib/utils';
  import { auth } from '$lib/stores/auth';
  import { canRead, canSystem } from '$lib/utils/permissions';
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
    /** 所需权限点；为空表示无需特殊权限（如 Dashboard） */
    perm?: string;
  }

  interface NavGroup {
    label: string;
    items: NavItem[];
  }

  const navGroups: NavGroup[] = [
    {
      label: '基础设施',
      items: [
        { href: '/servers', label: '服务器', perm: 'server:read' },
        { href: '/datacenters', label: '数据中心', perm: 'datacenter:read' },
        { href: '/providers', label: '供应商', perm: 'provider:read' },
      ],
    },
    {
      label: '网络',
      items: [
        { href: '/domains', label: '域名', perm: 'domain:read' },
        { href: '/certificates', label: '证书', perm: 'certificate:read' },
        { href: '/server-ips', label: '服务器IP', perm: 'server_ip:read' },
        { href: '/server-ports', label: '服务器端口', perm: 'server_port:read' },
        { href: '/server-port-templates', label: '端口模板', perm: 'server_port_template:read' },
      ],
    },
    {
      label: '应用',
      items: [
        { href: '/ops-sites', label: '站点', perm: 'ops_site:read' },
        { href: '/database-instances', label: '数据库实例', perm: 'database_instance:read' },
      ],
    },
    {
      label: '运维',
      items: [
        { href: '/backup-plans', label: '备份计划', perm: 'backup_plan:read' },
        { href: '/monitor-targets', label: '监控目标', perm: 'monitor_target:read' },
      ],
    },
    {
      label: '管理',
      items: [
        { href: '/users', label: '用户管理', perm: 'system:user' },
        { href: '/roles', label: '角色管理', perm: 'system:role' },
        { href: '/dicts', label: '字典管理', perm: 'dict:read' },
        { href: '/recycle', label: '回收站', perm: 'system:recycle' },
        { href: '/attachments', label: '附件', perm: 'attachment:read' },
      ],
    },
    {
      label: '审计',
      items: [
        { href: '/audit-logs', label: '审计日志', perm: 'system:audit' },
        { href: '/change-records', label: '变更记录', perm: 'system:change' },
      ],
    },
  ];

  // Track which groups are expanded (persisted)
  let expandedGroups = $state<Record<string, boolean>>({});
  // Sidebar collapsed state (persisted)
  let collapsed = $state(false);
  let groupsInitialized = $state(false);
  // 折叠态浮层：click 固定的分组 + hover 预览的分组（两者并存，click 优先常驻）
  let collapsedPopover = $state<string | null>(null);
  let hoverGroup = $state<string | null>(null);
  // 折叠态浮层 fixed 定位锚点（脱离 nav overflow 约束，避免被 main 内容覆盖/裁剪）
  let popoverAnchor = $state<{ top: number; left: number } | null>(null);

  // 折叠态点击图标固定打开浮层后，点击页面其他区域或滚动（含侧边栏内部滚动）自动关闭
  $effect(() => {
    if (!collapsedPopover) return;
    const close = () => { collapsedPopover = null; };
    const onScroll = () => { collapsedPopover = null; };
    window.addEventListener('click', close);
    window.addEventListener('scroll', onScroll, true); // capture：捕获 nav 内部滚动
    return () => {
      window.removeEventListener('click', close);
      window.removeEventListener('scroll', onScroll, true);
    };
  });

  function setPopoverAnchor(el: HTMLElement) {
    const r = el.getBoundingClientRect();
    popoverAnchor = { top: r.top, left: r.left + r.width + 8 };
  }

  function isCollapsedGroupOpen(label: string): boolean {
    return collapsedPopover === label || hoverGroup === label;
  }

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

  // 按权限过滤菜单：无 read 权限的资源不显示，系统资源按 system:* 权限判断。
  const visibleGroups = $derived(
    navGroups
      .map((g) => ({
        ...g,
        items: g.items.filter((item) => {
          if (!item.perm) return true;
          return item.perm.startsWith('system:') ? canSystem(item.perm.slice(7)) : canRead(item.perm.replace(/:read$/, ''));
        }),
      }))
      .filter((g) => g.items.length > 0),
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

  <nav class="no-scrollbar flex-1 space-y-0.5 overflow-y-auto overflow-x-hidden px-2 py-2">
    <!-- Dashboard -->
    <a
      href="/"
      onclick={onMobileClose}
      class={cn(
        'flex items-center rounded-md text-sm transition-colors hover:bg-sidebar-accent',
        collapsed ? 'justify-center px-2 py-1.5' : 'gap-2.5 px-3 py-1.5',
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
        <!-- 折叠模式：图标按钮，hover 预览 + 点击固定弹出子菜单（Ant Design Pro 风格） -->
        <!-- 浮层用 fixed 定位并挂在按钮右侧，避免被 nav overflow / main 内容遮挡 -->
        <div
          role="group"
          class="group relative"
          onmouseenter={(e) => {
            hoverGroup = group.label;
            setPopoverAnchor(e.currentTarget as HTMLElement);
          }}
          onmouseleave={() => { hoverGroup = null; }}
        >
          <button
            class={cn(
              'flex w-full items-center justify-center rounded-md px-2 py-1.5 transition-colors hover:bg-sidebar-accent',
              hasActive || isCollapsedGroupOpen(group.label) ? 'bg-sidebar-accent text-sidebar-accent-foreground' : 'text-sidebar-foreground/95'
            )}
            title={group.label}
            onclick={(e) => {
              e.stopPropagation();
              collapsedPopover = collapsedPopover === group.label ? null : group.label;
              setPopoverAnchor(e.currentTarget as HTMLElement);
            }}
            aria-expanded={isCollapsedGroupOpen(group.label)}
            aria-haspopup="menu"
          >
            {#if GroupIcon}
              <GroupIcon class="h-4 w-4 shrink-0" />
            {/if}
          </button>
          <div
            class={cn(
              'fixed z-[9999] min-w-48 rounded-lg border border-sidebar-border bg-sidebar-background p-2 shadow-xl transition-all duration-150',
              isCollapsedGroupOpen(group.label) ? 'visible opacity-100' : 'invisible opacity-0'
            )}
            style={popoverAnchor ? `top:${popoverAnchor.top}px;left:${popoverAnchor.left}px;` : ''}
            role="menu"
          >
            <!-- 主菜单头部：图标 + 名称 + 分隔线，与子菜单形成层级 -->
            <div class="flex items-center gap-2 border-b border-sidebar-border px-2 pb-2 pt-1">
              {#if GroupIcon}
                <GroupIcon class="h-4 w-4 shrink-0 text-sidebar-primary" />
              {/if}
              <span class="text-sm font-semibold text-sidebar-foreground">{group.label}</span>
            </div>
            <!-- 子菜单：缩进 + 左侧前导线，当前项带指示条 -->
            <div class="ml-3 mt-1 space-y-0.5 border-l border-sidebar-border pl-2 pt-1">
              {#each group.items as item}
                <a
                  href={item.href}
                  onclick={(e) => {
                    e.stopPropagation();
                    collapsedPopover = null;
                    hoverGroup = null;
                    onMobileClose();
                  }}
                  class={cn(
                    'relative block rounded-md px-2 py-1.5 text-sm transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground',
                    isActive(item.href, $page.url.pathname)
                      ? 'bg-sidebar-accent font-medium text-sidebar-accent-foreground'
                      : 'text-sidebar-foreground/90'
                  )}
                >
                  {#if isActive(item.href, $page.url.pathname)}
                    <span class="absolute -left-[11px] top-1/2 h-4 w-1 -translate-y-1/2 rounded-full bg-sidebar-primary"></span>
                  {/if}
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
              'flex w-full items-center gap-2.5 rounded-md px-3 py-1.5 text-sm transition-colors hover:bg-sidebar-accent',
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
                    'relative flex items-center rounded-md px-3 py-1 text-sm transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground',
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
        'flex w-full items-center rounded-md py-1.5 text-sm text-sidebar-foreground/90 transition-colors hover:bg-sidebar-accent',
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
