import { get } from 'svelte/store';
import { auth } from '$lib/stores/auth';

/**
 * 权限工具：判断当前登录用户是否拥有指定权限点。
 *
 * 权限点格式：
 * - 业务资源：`server:read` / `server:create` / `server:update` / `server:delete`（14 种资源）
 * - 系统资源：`system:user` / `system:role` / `system:recycle` / `system:audit` / `system:change`
 *
 * 超管（is_superuser）恒为 true。
 */
export function hasPerm(perm: string): boolean {
  const user = get(auth).user;
  if (!user) return false;
  if (user.is_superuser) return true;
  return (user.permissions ?? []).includes(perm);
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
