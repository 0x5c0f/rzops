/**
 * 权限矩阵共享定义：按左侧菜单结构分组，供角色新建/编辑页复用。
 *
 * - 业务资源：三档递进操作（查看 < 编辑 < 删除），提交时展开为完整权限点
 * - 系统资源：单权限点 system:*
 */

export interface MatrixResource {
  key: string;
  label: string;
  /** 系统级资源（单权限点 system:*），无操作递进 */
  system?: boolean;
}

export interface MatrixGroup {
  title: string;
  resources: MatrixResource[];
}

/** 业务资源三档递进操作（提交时按档位展开） */
export const LEVEL_ACTIONS = [
  { level: 1, key: 'read', label: '查看' },
  { level: 2, key: 'update', label: '编辑' },
  { level: 3, key: 'delete', label: '删除' },
];

/** 各档位对应的完整权限点展开 */
const LEVEL_PERMS: Record<number, string[]> = {
  1: ['read'],
  2: ['read', 'create', 'update'],
  3: ['read', 'create', 'update', 'delete'],
};

/** 按菜单结构分组 */
export const MATRIX_GROUPS: MatrixGroup[] = [
  {
    title: '基础设施',
    resources: [
      { key: 'server', label: '服务器' },
      { key: 'datacenter', label: '数据中心' },
      { key: 'provider', label: '供应商' },
    ],
  },
  {
    title: '网络',
    resources: [
      { key: 'domain', label: '域名' },
      { key: 'certificate', label: '证书' },
      { key: 'server_ip', label: '服务器IP' },
      { key: 'server_port', label: '服务器端口' },
      { key: 'server_port_template', label: '端口模板' },
    ],
  },
  {
    title: '应用',
    resources: [
      { key: 'ops_site', label: '站点' },
      { key: 'database_instance', label: '数据库实例' },
    ],
  },
  {
    title: '运维',
    resources: [
      { key: 'backup_plan', label: '备份计划' },
      { key: 'monitor_target', label: '监控目标' },
    ],
  },
  {
    title: '管理',
    resources: [
      { key: 'attachment', label: '附件' },
      { key: 'dict', label: '字典' },
      { key: 'user', label: '用户管理', system: true },
      { key: 'role', label: '角色管理', system: true },
      { key: 'recycle', label: '回收站', system: true },
    ],
  },
  {
    title: '审计',
    resources: [
      { key: 'audit', label: '审计日志', system: true },
      { key: 'change', label: '变更记录', system: true },
    ],
  },
];

/** 所有业务资源（用于校验合法性） */
export const BUSINESS_KEYS = MATRIX_GROUPS.flatMap((g) =>
  g.resources.filter((r) => !r.system).map((r) => r.key),
);

/** 计算某业务资源的当前档位（从权限点集合推断） */
export function resourceLevel(resource: string, permissions: string[]): number {
  let level = 0;
  for (const { level: lv, key } of LEVEL_ACTIONS) {
    if (permissions.includes(`${resource}:${key}`)) level = Math.max(level, lv);
  }
  return level;
}

/** 计算某资源的「隐含级别」——权限点集合中实际隐含到达的最高档位 */
export function effectiveLevel(resource: string, permissions: string[]): number {
  let level = 0;
  for (const { level: lv, key } of LEVEL_ACTIONS) {
    // create 也视为编辑档的一部分
    const expanded = LEVEL_PERMS[lv] ?? [];
    const has = expanded.some((a) => permissions.includes(`${resource}:${a}`));
    if (has) level = Math.max(level, lv);
  }
  return level;
}

/** 设置某业务资源到指定档位（0=无权限，1=查看，2=编辑，3=删除） */
export function setResourceLevel(
  resource: string,
  level: number,
  permissions: string[],
): string[] {
  // 先移除该资源所有操作权限点
  const rest = permissions.filter((p) => {
    const i = p.lastIndexOf(':');
    if (i <= 0) return true;
    const r = p.slice(0, i);
    const a = p.slice(i + 1);
    if (r !== resource) return true;
    return !['read', 'create', 'update', 'delete'].includes(a);
  });
  if (level <= 0) return rest;
  const perms = LEVEL_PERMS[level] ?? [];
  return [...rest, ...perms.map((a) => `${resource}:${a}`)];
}

/** 判断某系统权限点是否已勾选 */
export function hasSystemPerm(systemKey: string, permissions: string[]): boolean {
  return permissions.includes(`system:${systemKey}`);
}

/** 切换系统权限点 */
export function toggleSystemPerm(systemKey: string, permissions: string[]): string[] {
  const perm = `system:${systemKey}`;
  return permissions.includes(perm)
    ? permissions.filter((p) => p !== perm)
    : [...permissions, perm];
}
