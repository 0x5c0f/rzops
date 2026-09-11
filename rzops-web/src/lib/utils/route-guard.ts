import { hasPerm } from './permissions';

/**
 * 路由访问守卫：按路径前缀映射所需权限点。
 * 路由守卫在 Sidebar 权限过滤之外的兜底，防止通过 URL 直接访问未授权页面。
 *
 * 匹配顺序：先匹配新建/编辑页（要求 create/update），再匹配列表/详情页（read）。
 * 业务资源用 xxx:read/create/update，系统资源用 system:*
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

/** 新建/编辑页 → 需要 create/update 权限 */
const EDIT_PERMS: [string, string, string][] = [
  ['/servers', 'server', 'update'],
  ['/datacenters', 'datacenter', 'update'],
  ['/providers', 'provider', 'update'],
  ['/domains', 'domain', 'update'],
  ['/certificates', 'certificate', 'update'],
  ['/server-ips', 'server_ip', 'update'],
  ['/server-ports', 'server_port', 'update'],
  ['/server-port-templates', 'server_port_template', 'update'],
  ['/ops-sites', 'ops_site', 'update'],
  ['/database-instances', 'database_instance', 'update'],
  ['/backup-plans', 'backup_plan', 'update'],
  ['/monitor-targets', 'monitor_target', 'update'],
  ['/attachments', 'attachment', 'update'],
  ['/dicts', 'dict', 'update'],
];

/**
 * 根据路径返回所需权限点；无需权限的路径（如 /login、/）返回 null。
 * - /xxx/new  → xxx:create
 * - /xxx/[id]/edit → xxx:update
 * - 其余 → 列表/详情 read
 */
export function permForPath(pathname: string): string | null {
  // 新建页
  for (const [prefix, res] of EDIT_PERMS) {
    if (pathname === prefix + '/new') return `${res}:create`;
  }
  // 编辑页
  for (const [prefix, res] of EDIT_PERMS) {
    if (new RegExp(`^${prefix}/[^/]+/edit$`).test(pathname)) return `${res}:update`;
  }
  // 列表/详情
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
  return hasPerm(perm);
}

/** 供 +layout.svelte 使用的路径守卫函数（返回该路径是否需要权限 & 是否通过） */
export function routeGuard(pathname: string, perms: string[]): { required: boolean; ok: boolean } {
  const perm = permForPath(pathname);
  if (!perm) return { required: false, ok: true };
  return { required: true, ok: hasPerm(perm) };
}
