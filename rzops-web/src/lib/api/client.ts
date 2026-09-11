const BASE_URL = '';

import { showToast } from '$lib/stores/toast.svelte';

export class ApiError extends Error {
  constructor(
    public status: number,
    message: string,
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

async function request<T>(path: string, init?: RequestInit): Promise<T> {
  const token = typeof window !== 'undefined' ? localStorage.getItem('token') : null;

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(init?.headers as Record<string, string>),
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  // 空字符串的引用/日期字段会破坏后端 Option<Uuid>/Option<DateTime> 反序列化，
  // 提交前统一剔除（等于不更新该字段，后端部分更新语义会保留原值）。
  let body = init?.body;
  if (body && typeof body === 'string' && (init?.method === 'POST' || init?.method === 'PUT')) {
    try {
      const parsed = JSON.parse(body);
      body = JSON.stringify(sanitizeEmptyRefs(parsed));
    } catch {
      // 非 JSON body（如 FormData），保持原样
    }
  }

  const res = await fetch(`${BASE_URL}${path}`, { ...init, headers, body });

  if (res.status === 401) {
    if (typeof window !== 'undefined') {
      localStorage.removeItem('token');
      // 登录接口失败由登录页自行展示错误，不弹全局 toast（避免双重提示）
      if (!path.includes('/auth/login')) {
        showToast('登录已过期，请重新登录', 'error', 3000);
        // 已在登录页时不重复跳转，避免 401 触发的整页跳转造成 reload 死循环
        if (window.location.pathname !== '/login') {
          window.location.href = '/login';
        }
      }
    }
    throw new ApiError(401, '未授权，请重新登录');
  }

  if (res.status === 204) {
    return undefined as T;
  }

  if (!res.ok) {
    const err = await res.json().catch(() => ({ error: '未知错误' }));
    const msg = err.error ?? `HTTP ${res.status}`;
    if (typeof window !== 'undefined') {
      showToast(msg, 'error', 4000);
    }
    throw new ApiError(res.status, msg);
  }

  return res.json() as Promise<T>;
}

export const api = {
  get: <T>(path: string, params?: unknown) => {
    if (!params) return request<T>(path);

    const entries = Object.entries(params as Record<string, unknown>).filter(
      ([, v]) => v !== undefined && v !== null && v !== '',
    );

    if (entries.length === 0) return request<T>(path);

    const qs = new URLSearchParams(entries.map(([k, v]) => [k, String(v)])).toString();
    return request<T>(`${path}?${qs}`);
  },

  post: <T>(path: string, body?: unknown) =>
    request<T>(path, {
      method: 'POST',
      body: body ? JSON.stringify(body) : undefined,
    }),

  put: <T>(path: string, body?: unknown) =>
    request<T>(path, {
      method: 'PUT',
      body: body ? JSON.stringify(body) : undefined,
    }),

  delete: <T>(path: string) => request<T>(path, { method: 'DELETE' }),
};

// 空字符串的引用字段（*_id）与日期字段（*_date / *_time）会让后端
// Option<Uuid> / Option<DateTime> 反序列化失败（422）。编辑表单常用空串表示"未设置"，
// 提交前剔除这些字段，让后端部分更新语义保留原值。
function sanitizeEmptyRefs(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(sanitizeEmptyRefs);
  if (value && typeof value === 'object') {
    const out: Record<string, unknown> = {};
    for (const [k, v] of Object.entries(value as Record<string, unknown>)) {
      if (v === '' && /_(id|date|time)$/.test(k)) continue;
      out[k] = sanitizeEmptyRefs(v);
    }
    return out;
  }
  return value;
}
