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
  running: '运行中',
  retired: '已退役',
  temporary_offline: '临时下线',
  temp_offline: '临时下线',
  permanent_offline: '永久下线',
  inactive: '已下线',
  offline: '已下线',
  enabled: '启用',
  disabled: '停用',
  reserved: '预留',
  paused: '已暂停',
  expired: '已过期',
  expiring: '即将过期',
  pending: '待处理',
  revoked: '已吊销',
  stopped: '已停止',
  archived: '已归档',
  draft: '草稿',
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

/**
 * 状态徽章样式映射（参照业界主流状态色：绿=正常、琥珀=预警、红=危险、灰=停用、紫=归档）
 * 用于列表状态列渲染彩色徽章（静态兜底，实际优先读字典 extra_data.color）
 */
const STATUS_BADGE_STYLES: Record<string, string> = {
  // 正常/运行
  active: 'bg-green-100 text-green-700 border-transparent',
  running: 'bg-green-100 text-green-700 border-transparent',
  enabled: 'bg-green-100 text-green-700 border-transparent',
  // 预警/暂缓
  temp_offline: 'bg-amber-100 text-amber-700 border-transparent',
  paused: 'bg-amber-100 text-amber-700 border-transparent',
  expiring: 'bg-amber-100 text-amber-700 border-transparent',
  pending: 'bg-amber-100 text-amber-700 border-transparent',
  // 危险/失效
  retired: 'bg-red-100 text-red-700 border-transparent',
  permanent_offline: 'bg-red-100 text-red-700 border-transparent',
  revoked: 'bg-red-100 text-red-700 border-transparent',
  expired: 'bg-red-100 text-red-700 border-transparent',
  offline: 'bg-red-100 text-red-700 border-transparent',
  // 停用
  disabled: 'bg-slate-100 text-slate-500 border-transparent',
  inactive: 'bg-slate-100 text-slate-500 border-transparent',
  stopped: 'bg-slate-100 text-slate-500 border-transparent',
  reserved: 'bg-blue-100 text-blue-700 border-transparent',
  // 归档（不强调）
  archived: 'bg-purple-50 text-purple-600 border-transparent',
};

/**
 * 颜色值 → 徽章样式类（字典 extra_data.color 支持的颜色）
 */
const COLOR_BADGE_STYLES: Record<string, string> = {
  green: 'bg-green-100 text-green-700 border-transparent',
  amber: 'bg-amber-100 text-amber-700 border-transparent',
  red: 'bg-red-100 text-red-700 border-transparent',
  blue: 'bg-blue-100 text-blue-700 border-transparent',
  purple: 'bg-purple-50 text-purple-600 border-transparent',
  slate: 'bg-slate-100 text-slate-500 border-transparent',
  gray: 'bg-slate-100 text-slate-500 border-transparent',
};

/**
 * 获取状态徽章样式类（无映射时返回 null）
 */
export function getStatusBadgeClass(status?: string | null): string | null {
  if (!status) return null;
  return STATUS_BADGE_STYLES[status] || null;
}

/**
 * 根据字典颜色值获取徽章样式类（extra_data.color）
 */
export function getColorBadgeClass(color?: string | null): string | null {
  if (!color) return null;
  return COLOR_BADGE_STYLES[color] || null;
}
