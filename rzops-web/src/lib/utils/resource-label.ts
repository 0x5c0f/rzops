import { providersApi } from '$lib/api/providers';
import { datacentersApi } from '$lib/api/datacenters';
import { serversApi } from '$lib/api/servers';
import { serverIpsApi } from '$lib/api/server-ips';
import { serverPortsApi } from '$lib/api/server-ports';
import { domainsApi } from '$lib/api/domains';
import { certificatesApi } from '$lib/api/certificates';
import { databaseInstancesApi } from '$lib/api/database-instances';
import { opsSitesApi } from '$lib/api/ops-sites';
import { backupPlansApi } from '$lib/api/backup-plans';
import { monitorTargetsApi } from '$lib/api/monitor-targets';
import { contractsApi } from '$lib/api/contracts';
import { attachmentsApi } from '$lib/api/attachments';

/**
 * 资源元数据：用于在审计日志 / 变更记录中把 resource_type + resource_id
 * 解析为可读的资源名称，并提供跳转到对应资源详情页的路径。
 */
interface ResourceMeta {
  /** 拉取资源详情的 API（无详情页或不可解析则为 null） */
  api: ((id: string) => Promise<unknown>) | null;
  /** 详情响应中用于展示的字段名 */
  labelField: string;
  /** 详情页跳转路径 */
  detailPath: ((id: string) => string) | null;
  /** 中文标签 */
  zh: string;
}

const META: Record<string, ResourceMeta> = {
  provider: { api: providersApi.getById, labelField: 'name', detailPath: (id) => `/providers/${id}`, zh: '供应商' },
  datacenter: { api: datacentersApi.getById, labelField: 'name', detailPath: (id) => `/datacenters/${id}`, zh: '数据中心' },
  server: { api: serversApi.getById, labelField: 'name', detailPath: (id) => `/servers/${id}`, zh: '服务器' },
  server_ip: { api: serverIpsApi.getById, labelField: 'ip_address', detailPath: (id) => `/server-ips/${id}`, zh: '服务器IP' },
  server_port: { api: serverPortsApi.getById, labelField: 'service_name', detailPath: (id) => `/server-ports/${id}`, zh: '服务器端口' },
  domain: { api: domainsApi.getById, labelField: 'name', detailPath: (id) => `/domains/${id}`, zh: '域名' },
  certificate: { api: certificatesApi.getById, labelField: 'name', detailPath: (id) => `/certificates/${id}`, zh: '证书' },
  database_instance: { api: databaseInstancesApi.getById, labelField: 'name', detailPath: (id) => `/database-instances/${id}`, zh: '数据库实例' },
  ops_site: { api: opsSitesApi.getById, labelField: 'name', detailPath: (id) => `/ops-sites/${id}`, zh: '站点' },
  backup_plan: { api: backupPlansApi.getById, labelField: 'name', detailPath: (id) => `/backup-plans/${id}`, zh: '备份计划' },
  monitor_target: { api: monitorTargetsApi.getById, labelField: 'name', detailPath: (id) => `/monitor-targets/${id}`, zh: '监控目标' },
  contract: { api: contractsApi.getById, labelField: 'name', detailPath: (id) => `/contracts/${id}`, zh: '合同' },
  attachment: { api: attachmentsApi.getById, labelField: 'filename', detailPath: (id) => `/attachments/${id}`, zh: '附件' },
  site_relation: { api: null, labelField: '', detailPath: null, zh: '站点关联' },
  site_server: { api: null, labelField: '', detailPath: null, zh: '站点-服务器关联' },
  site_database: { api: null, labelField: '', detailPath: null, zh: '站点-数据库关联' },
  site_domain: { api: null, labelField: '', detailPath: null, zh: '站点-域名关联' },
  auth: { api: null, labelField: '', detailPath: null, zh: '认证' },
  user: { api: null, labelField: '', detailPath: null, zh: '用户' },
  unknown: { api: null, labelField: '', detailPath: null, zh: '未知' },
};

/** 解析单个资源为可读名称；失败或不可解析返回 null。 */
export async function resolveResourceLabel(
  type: string | null,
  id: string | null
): Promise<string | null> {
  if (!type || !id) return null;
  const meta = META[type];
  if (!meta || !meta.api) return null;
  try {
    const data = (await meta.api(id)) as Record<string, unknown>;
    const v = data[meta.labelField];
    return typeof v === 'string' && v.trim() ? v : null;
  } catch {
    return null;
  }
}

/** 资源详情页跳转路径；无则返回 null。 */
export function getResourceLink(type: string | null, id: string | null): string | null {
  if (!type || !id) return null;
  const meta = META[type];
  if (!meta || !meta.detailPath) return null;
  return meta.detailPath(id);
}

/**
 * 从资源快照（变更记录的 before/after_data，即资源详情 JSON）中提取展示名。
 * 避免对同一资源再发起一次详情请求。
 */
export function labelFromSnapshot(type: string | null, data: unknown): string | null {
  if (!type || !data || typeof data !== 'object') return null;
  const meta = META[type];
  if (!meta || !meta.labelField) return null;
  const v = (data as Record<string, unknown>)[meta.labelField];
  return typeof v === 'string' && v.trim() ? v : null;
}

/** 资源类型中文标签。 */
export function resourceTypeZh(type: string | null): string {
  if (!type) return '-';
  return META[type]?.zh || type;
}
