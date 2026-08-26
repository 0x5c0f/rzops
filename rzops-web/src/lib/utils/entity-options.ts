/**
 * 关联实体选项工具
 * 用于获取供应商、数据中心等关联数据的下拉选项
 */

import { providersApi } from '$lib/api/providers';
import { datacentersApi } from '$lib/api/datacenters';
import { serversApi } from '$lib/api/servers';
import { databaseInstancesApi } from '$lib/api/database-instances';
import { domainsApi } from '$lib/api/domains';
import { credentialsApi } from '$lib/api/credentials';
import { opsSitesApi } from '$lib/api/ops-sites';
import { backupPlansApi } from '$lib/api/backup-plans';
import { monitorTargetsApi } from '$lib/api/monitor-targets';

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
 * 获取数据库实例选项
 */
export async function getDatabaseInstanceOptions(): Promise<SelectOption[]> {
  try {
    const res = await databaseInstancesApi.list({ limit: 200 });
    return res.data.map(item => ({
      label: `${item.name} (${item.db_type})`,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to load database instances:', err);
    return [];
  }
}

/**
 * 获取域名选项
 */
export async function getDomainOptions(): Promise<SelectOption[]> {
  try {
    const res = await domainsApi.list({ limit: 200 });
    return res.data.map(item => ({
      label: item.domain_name,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to load domains:', err);
    return [];
  }
}

/**
 * 获取凭证选项
 */
export async function getCredentialOptions(): Promise<SelectOption[]> {
  try {
    const res = await credentialsApi.list({ limit: 200 });
    return res.data.map(item => ({
      label: item.name,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to load credentials:', err);
    return [];
  }
}

/**
 * 获取站点选项
 */
export async function getOpsSiteOptions(): Promise<SelectOption[]> {
  try {
    const res = await opsSitesApi.list({ limit: 200 });
    return res.data.map(item => ({
      label: item.name,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to load ops sites:', err);
    return [];
  }
}

/**
 * 获取备份计划选项
 */
export async function getBackupPlanOptions(): Promise<SelectOption[]> {
  try {
    const res = await backupPlansApi.list({ limit: 200 });
    return res.data.map(item => ({
      label: item.name,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to load backup plans:', err);
    return [];
  }
}

/**
 * 获取监控目标选项
 */
export async function getMonitorTargetOptions(): Promise<SelectOption[]> {
  try {
    const res = await monitorTargetsApi.list({ limit: 200 });
    return res.data.map(item => ({
      label: item.name,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to load monitor targets:', err);
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
