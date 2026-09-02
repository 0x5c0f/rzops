/**
 * 关联实体选项工具
 * 用于获取供应商、数据中心等关联数据的下拉选项
 */

import { providersApi } from '$lib/api/providers';
import { datacentersApi } from '$lib/api/datacenters';
import { serversApi } from '$lib/api/servers';
import { databaseInstancesApi } from '$lib/api/database-instances';
import { domainsApi } from '$lib/api/domains';
import { certificatesApi } from '$lib/api/certificates';
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
    const res = await providersApi.list({ per_page: 200 });
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
    const res = await datacentersApi.list({ per_page: 200 });
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
    const res = await serversApi.list({ per_page: 200 });
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
 * 远程搜索服务器选项（用于 RemoteSearchSelect，按关键字分页搜索）
 */
export async function searchServerOptions(keyword: string): Promise<SelectOption[]> {
  try {
    const res = await serversApi.list({ q: keyword || undefined, per_page: 20 });
    return res.data.map(item => ({
      label: `${item.name}${item.primary_ip ? ` (${item.primary_ip})` : ''}`,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to search servers:', err);
    return [];
  }
}

/**
 * 分页搜索服务器（用于 TableSelectModal，返回完整对象和总数）
 */
export async function searchServerPaginated(
  keyword: string,
  page: number,
  perPage: number
): Promise<{ data: Record<string, unknown>[]; total: number }> {
  try {
    const res = await serversApi.list({ q: keyword || undefined, page, per_page: perPage });
    return {
      data: res.data.map(item => ({
        id: item.id,
        name: item.name,
        primary_ip: item.primary_ip || '-',
        status: item.status,
        server_type: item.server_type || '-',
      })),
      total: res.count,
    };
  } catch (err) {
    console.error('Failed to search servers paginated:', err);
    return { data: [], total: 0 };
  }
}

/**
 * 分页搜索数据库实例（用于 TableSelectModal）
 */
export async function searchDatabasePaginated(
  keyword: string,
  page: number,
  perPage: number
): Promise<{ data: Record<string, unknown>[]; total: number }> {
  try {
    const res = await databaseInstancesApi.list({ q: keyword || undefined, page, per_page: perPage });
    return {
      data: res.data.map(item => ({
        id: item.id,
        name: item.name,
        db_type: item.db_type || '-',
        status: item.status,
        port: item.port || '-',
      })),
      total: res.count,
    };
  } catch (err) {
    console.error('Failed to search databases paginated:', err);
    return { data: [], total: 0 };
  }
}

/**
 * 分页搜索站点（用于 TableSelectModal）
 */
export async function searchSitePaginated(
  keyword: string,
  page: number,
  perPage: number
): Promise<{ data: Record<string, unknown>[]; total: number }> {
  try {
    const res = await opsSitesApi.list({ q: keyword || undefined, page, per_page: perPage });
    return {
      data: res.data.map(item => ({
        id: item.id,
        name: item.name,
        url: item.url || '-',
        status: item.status,
        service_target: item.service_target || '-',
      })),
      total: res.count,
    };
  } catch (err) {
    console.error('Failed to search sites paginated:', err);
    return { data: [], total: 0 };
  }
}

/**
 * 分页搜索域名（用于 TableSelectModal）
 */
export async function searchDomainPaginated(
  keyword: string,
  page: number,
  perPage: number
): Promise<{ data: Record<string, unknown>[]; total: number }> {
  try {
    const res = await domainsApi.list({ q: keyword || undefined, page, per_page: perPage });
    return {
      data: res.data.map(item => ({
        id: item.id,
        name: item.domain_name,
        registrar: item.provider_id || '-',
        status: item.is_enabled ? 'enabled' : 'disabled',
        expire_date: item.expiry_date || '-',
      })),
      total: res.count,
    };
  } catch (err) {
    console.error('Failed to search domains paginated:', err);
    return { data: [], total: 0 };
  }
}

/**
 * 分页搜索证书（用于 TableSelectModal）
 */
export async function searchCertificatePaginated(
  keyword: string,
  page: number,
  perPage: number
): Promise<{ data: Record<string, unknown>[]; total: number }> {
  try {
    const res = await certificatesApi.list({ q: keyword || undefined, page, per_page: perPage });
    return {
      data: res.data.map(item => ({
        id: item.id,
        name: item.name,
        issuer: item.provider_id || '-',
        status: item.status,
        expire_date: item.lease_end_date || '-',
      })),
      total: res.count,
    };
  } catch (err) {
    console.error('Failed to search certificates paginated:', err);
    return { data: [], total: 0 };
  }
}

/**
 * 远程搜索数据库实例选项
 */
export async function searchDatabaseInstanceOptions(keyword: string): Promise<SelectOption[]> {
  try {
    const res = await databaseInstancesApi.list({ q: keyword || undefined, per_page: 20 });
    return res.data.map(item => ({
      label: `${item.name} (${item.db_type})`,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to search database instances:', err);
    return [];
  }
}

/**
 * 远程搜索域名选项
 */
export async function searchDomainOptions(keyword: string): Promise<SelectOption[]> {
  try {
    const res = await domainsApi.list({ q: keyword || undefined, per_page: 20 });
    return res.data.map(item => ({
      label: item.domain_name,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to search domains:', err);
    return [];
  }
}

/**
 * 远程搜索站点选项
 */
export async function searchOpsSiteOptions(keyword: string): Promise<SelectOption[]> {
  try {
    const res = await opsSitesApi.list({ q: keyword || undefined, per_page: 20 });
    return res.data.map(item => ({
      label: item.name,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to search ops sites:', err);
    return [];
  }
}

/**
 * 远程搜索供应商选项
 */
export async function searchProviderOptions(keyword: string): Promise<SelectOption[]> {
  try {
    const res = await providersApi.list({ q: keyword || undefined, per_page: 20 });
    return res.data.map(item => ({
      label: item.name,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to search providers:', err);
    return [];
  }
}

/**
 * 获取数据库实例选项
 */
export async function getDatabaseInstanceOptions(): Promise<SelectOption[]> {
  try {
    const res = await databaseInstancesApi.list({ per_page: 200 });
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
 * 获取证书选项
 */
export async function getCertificateOptions(): Promise<SelectOption[]> {
  try {
    const res = await certificatesApi.list({ per_page: 100 });
    return res.data.map(item => ({
      label: item.name,
      value: item.id,
    }));
  } catch (err) {
    console.error('Failed to load certificates:', err);
    return [];
  }
}

/**
 * 获取域名选项
 */
export async function getDomainOptions(): Promise<SelectOption[]> {
  try {
    const res = await domainsApi.list({ per_page: 200 });
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
 * 获取站点选项
 */
export async function getOpsSiteOptions(): Promise<SelectOption[]> {
  try {
    const res = await opsSitesApi.list({ per_page: 100 });
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
    const res = await backupPlansApi.list({ per_page: 200 });
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
    const res = await monitorTargetsApi.list({ per_page: 200 });
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
