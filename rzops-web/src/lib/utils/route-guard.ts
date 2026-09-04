import { hasPerm } from './permissions';

/**
 * 路由访问守卫：按路径前缀映射所需权限点。
 * 路由守卫在 Sidebar 权限过滤之外的兜底，防止通过 URL 直接访问未授权页面。
 *
 * 路径前缀 → 所需权限点（业务资源用 read，系统资源用 system:*）
 */
const PREFIX_PERMS: [string, string][] = [
  ['/servers', 'server:read'],
  ['/datacenters', 'datacenter:read'],
  ['/providers', 'provider:read'],
  ['/domains', 'domain:read'],
  ['/certificates', 'certificate:read'],
  ['/server-ips', 'server_ip:read'],
  ['/server-ports', 'server_port:read'],
  ['/server-port-templates', 'server_port_template:read'],
  ['/ops-sites', 'ops_site:read'],
  ['/database-instances', 'database_instance:read'],
  ['/backup-plans', 'backup_plan:read'],
  ['/monitor-targets', 'monitor_target:read'],
  ['/attachments', 'attachment:read'],
  ['/contracts', 'contract:read'],
  ['/dicts', 'dict:read'],
  ['/users', 'system:user'],
  ['/roles', 'system:role'],
  ['/recycle', 'system:recycle'],
  ['/audit-logs', 'system:audit'],
  ['/change-records', 'system:change'],
];

/**
 * 根据路径返回所需权限点；无需权限的路径（如 /login、/）返回 null。
 */
export function permForPath(pathname: string): string | null {
  for (const [prefix, perm] of PREFIX_PERMS) {
    if (pathname === prefix || pathname.startsWith(prefix + '/')) {
      return perm;
    }
  }
  return null;
}

/**
 * 检查当前用户对指定路径是否有访问权限。
 */
export function canAccessPath(pathname: string, isSuperuser: boolean, permissions: string[]): boolean {
  if (isSuperuser) return true;
  const perm = permForPath(pathname);
  if (!perm) return true;
  if (perm.startsWith('system:')) return hasPerm(perm);
  return permissions.includes(perm);
}

/** 供 +layout.svelte 使用的简易函数映射（避免在模板中重复导入 store） */
export const routePerms = new Map<string, (perms: string[]) => boolean>(
  PREFIX_PERMS.map(([prefix, perm]) => [
    prefix,
    (perms: string[]) => {
      if (perm.startsWith('system:')) return perms.includes(perm);
      return perms.includes(perm);
    },
  ]),
);
