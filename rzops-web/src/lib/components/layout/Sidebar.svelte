<script lang="ts">
  import { page } from '$app/stores';
  import { cn } from '$lib/utils';
  import * as Collapsible from '$lib/ui/collapsible';
  import ChevronDown from '@lucide/svelte/icons/chevron-down';

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
        { href: '/credentials', label: '凭据' },
        { href: '/backup-plans', label: '备份计划' },
        { href: '/monitor-targets', label: '监控目标' },
      ],
    },
    {
      label: '管理',
      items: [
        { href: '/contracts', label: '合同' },
        { href: '/attachments', label: '附件' },
        { href: '/site-relations', label: '站点关联' },
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

  // Track which groups are expanded
  let expandedGroups = $state<Record<string, boolean>>({});

  function toggleGroup(label: string) {
    expandedGroups[label] = !expandedGroups[label];
  }

  function isActive(href: string, currentPath: string) {
    if (href === '/') return currentPath === '/';
    return currentPath.startsWith(href);
  }

  function isGroupActive(group: NavGroup, currentPath: string) {
    return group.items.some(item => isActive(item.href, currentPath));
  }

  // Auto-expand groups that contain the current page
  $effect(() => {
    const currentPath = $page.url.pathname;
    navGroups.forEach(group => {
      if (isGroupActive(group, currentPath)) {
        expandedGroups[group.label] = true;
      }
    });
  });
</script>

<aside class="flex h-screen w-64 flex-col border-r bg-card">
  <div class="p-4">
    <a href="/" class="block">
      <h1 class="text-xl font-bold">RzOps</h1>
      <p class="text-sm text-muted-foreground">CMDB 管理平台</p>
    </a>
  </div>

  <nav class="flex-1 space-y-1 overflow-y-auto px-2 pb-4">
    <!-- Dashboard -->
    <a
      href="/"
      class={cn(
        'flex items-center rounded-md px-3 py-2 text-sm transition-colors hover:bg-accent',
        $page.url.pathname === '/' && 'bg-accent font-medium'
      )}
    >
      Dashboard
    </a>

    <!-- Navigation groups with collapsible -->
    {#each navGroups as group}
      {@const hasActive = isGroupActive(group, $page.url.pathname)}

      <Collapsible.Root
        open={expandedGroups[group.label] || hasActive}
        onOpenChange={(open) => expandedGroups[group.label] = open}
      >
        <Collapsible.Trigger
          class={cn(
            'flex w-full items-center justify-between rounded-md px-3 py-2 text-sm transition-colors hover:bg-accent',
            hasActive && 'text-primary'
          )}
        >
          <span class="font-medium">{group.label}</span>
          <ChevronDown class="h-4 w-4 transition-transform [[data-state=open]>&]:rotate-180" />
        </Collapsible.Trigger>

        <Collapsible.Content>
          <div class="ml-2 space-y-1 border-l pl-2">
            {#each group.items as item}
              <a
                href={item.href}
                class={cn(
                  'flex items-center rounded-md px-3 py-1.5 text-sm transition-colors hover:bg-accent',
                  isActive(item.href, $page.url.pathname) && 'bg-accent font-medium text-primary'
                )}
              >
                {item.label}
              </a>
            {/each}
          </div>
        </Collapsible.Content>
      </Collapsible.Root>
    {/each}
  </nav>
</aside>
