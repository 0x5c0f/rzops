/**
 * 全局 Toast 通知（Svelte 5 runes 实现）。
 *
 * 使用 `.svelte.ts` 扩展名以启用 runes：模块级 `$state` 跨组件共享响应式数组。
 * `showToast` 为普通导出的函数，可被任意 `.ts` 模块（如 api client）调用。
 */

export type ToastType = 'success' | 'error' | 'info';

export interface ToastItem {
  id: number;
  type: ToastType;
  message: string;
}

let toasts = $state<ToastItem[]>([]);
let seq = 0;

export function showToast(message: string, type: ToastType = 'info', duration = 4000): void {
  const id = ++seq;
  toasts = [...toasts, { id, type, message }];
  if (duration > 0) {
    setTimeout(() => dismissToast(id), duration);
  }
}

export function dismissToast(id: number): void {
  toasts = toasts.filter((t) => t.id !== id);
}

export function getToasts(): ToastItem[] {
  return toasts;
}
