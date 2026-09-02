/**
 * 关联资源状态显示工具
 * 用于在详情页/列表页中展示关联资源的状态，如"服务器名(已退役)"
 */

export interface ResourceStatusInfo {
  label: string;
  isOffline: boolean;
}

// 各资源类型的"下线"状态值
const OFFLINE_STATUSES: Record<string, string[]> = {
  server: ['retired'],
  site: ['temporary_offline', 'permanent_offline'],
  database: ['inactive', 'offline'],
  ip: ['disabled'],
  provider: ['inactive'],
};

// 状态值 -> 中文标签的映射（静态兜底，实际优先用字典）
const STATUS_LABELS: Record<string, string> = {
  active: '运行中',
  retired: '已退役',
  temporary_offline: '临时下线',
  permanent_offline: '永久下线',
  inactive: '已下线',
  offline: '已下线',
  enabled: '启用',
  disabled: '停用',
  reserved: '预留',
};

/**
 * 判断资源是否处于下线/停用状态
 */
export function isResourceOffline(resourceType: string, status?: string | null): boolean {
  if (!status) return false;
  const offlineList = OFFLINE_STATUSES[resourceType] || [];
  return offlineList.includes(status);
}

/**
 * 获取状态的中文标签
 */
export function getStatusLabel(status?: string | null): string {
  if (!status) return '';
  return STATUS_LABELS[status] || status;
}

/**
 * 格式化关联资源显示：名称 + 状态
 * 例如："web-server-01 (已退役)"
 * 如果资源正常运行，只返回名称
 */
export function formatResourceWithStatus(
  name: string | null | undefined,
  status?: string | null,
  resourceType?: string
): string {
  if (!name) return '-';
  if (!status || !resourceType) return name;
  if (isResourceOffline(resourceType, status)) {
    return `${name} (${getStatusLabel(status)})`;
  }
  return name;
}

/**
 * 获取关联资源状态的CSS类
 * 下线状态返回灰色样式类
 */
export function getResourceStatusClass(
  status?: string | null,
  resourceType?: string
): string {
  if (!status || !resourceType) return '';
  if (isResourceOffline(resourceType, status)) {
    return 'text-muted-foreground italic';
  }
  return '';
}
