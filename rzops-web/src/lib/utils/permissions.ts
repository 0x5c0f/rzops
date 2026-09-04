import { get } from 'svelte/store';
import { auth } from '$lib/stores/auth';

/**
 * 权限工具：判断当前登录用户是否拥有指定权限点。
 *
 * 权限点格式：
 * - 业务资源：`server:read` / `server:create` / `server:update` / `server:delete`（14 种资源）
 * - 系统资源：`system:user` / `system:role` / `system:recycle` / `system:audit` / `system:change`
 *
 * 业务资源权限支持递进（与后端一致）：
 * delete 隐含 update/create/read；update 隐含 create/read；create 隐含 read。
 * 超管（is_superuser）恒为 true。
 */
export function hasPerm(perm: string): boolean {
  const user = get(auth).user;
  if (!user) return false;
  if (user.is_superuser) return true;
  const perms = user.permissions ?? [];
  if (perms.includes(perm)) return true;
  // 系统权限无操作层级，不递进
  if (perm.startsWith('system:')) return false;
  const idx = perm.lastIndexOf(':');
  if (idx <= 0) return false;
  const resource = perm.slice(0, idx);
  const action = perm.slice(idx + 1);
  const rank = (a: string): number =>
    ({ delete: 4, update: 3, create: 2, read: 1 } as Record<string, number>)[a] ?? 0;
  const need = rank(action);
  if (need === 0) return false;
  return perms.some((p) => {
    const i = p.lastIndexOf(':');
    if (i <= 0) return false;
    return p.slice(0, i) === resource && rank(p.slice(i + 1)) >= need;
  });
}

/** 是否拥有某业务资源的只读权限（菜单可见性判断用） */
export function canRead(resource: string): boolean {
  return hasPerm(`${resource}:read`);
}

/** 是否拥有某业务资源的创建权限 */
export function canCreate(resource: string): boolean {
  return hasPerm(`${resource}:create`);
}

/** 是否拥有某业务资源的编辑权限 */
export function canUpdate(resource: string): boolean {
  return hasPerm(`${resource}:update`);
}

/** 是否拥有某业务资源的删除权限 */
export function canDelete(resource: string): boolean {
  return hasPerm(`${resource}:delete`);
}

/** 是否拥有某系统资源权限（用户管理 / 角色管理 / 回收站 / 审计 / 变更） */
export function canSystem(resource: string): boolean {
  return hasPerm(`system:${resource}`);
}
