/**
 * 关联实体选项工具
 * 用于获取供应商、数据中心等关联数据的下拉选项
 */

import { providersApi } from '$lib/api/providers';
import { datacentersApi } from '$lib/api/datacenters';
import { serversApi } from '$lib/api/servers';

export interface SelectOption {
  label: string;
  value: string;
}

/**
 * 获取供应商选项
 */
export async function getProviderOptions(): Promise<SelectOption[]> {
  try {
    const res = await providersApi.list({ limit: 200 });
    return res.data.map(item => ({
      label: item.name,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to load providers:', err);
    return [];
  }
}

/**
 * 获取数据中心选项
 */
export async function getDataCenterOptions(): Promise<SelectOption[]> {
  try {
    const res = await datacentersApi.list({ limit: 200 });
    return res.data.map(item => ({
      label: item.name,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to load data centers:', err);
    return [];
  }
}

/**
 * 获取服务器选项
 */
export async function getServerOptions(): Promise<SelectOption[]> {
  try {
    const res = await serversApi.list({ limit: 200 });
    return res.data.map(item => ({
      label: `${item.name}${item.primary_ip ? ` (${item.primary_ip})` : ''}`,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to load servers:', err);
    return [];
  }
}

/**
 * 确保选项包含当前值（用于编辑时显示已选值）
 */
export function ensureOption(
  options: SelectOption[],
  value: string | null | undefined,
  fallbackLabel?: string
): SelectOption[] {
  if (!value) return options;
  if (options.some(opt => opt.value === value)) return options;
  return [...options, { label: fallbackLabel || value, value }];
}

/**
 * 确保选项包含多个当前值
 */
export function ensureOptions(
  options: SelectOption[],
  values: string[] | null | undefined,
  fallbackLabel?: (v: string) => string
): SelectOption[] {
  if (!values || values.length === 0) return options;
  return values.reduce(
    (acc, v) => ensureOption(acc, v, fallbackLabel ? fallbackLabel(v) : undefined),
    options
  );
}
