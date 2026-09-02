/**
 * 枚举/字典选项工具
 * 将后端枚举转换为前端下拉选项。
 * 现在选项由数据字典（cmdb_dict）动态维护：
 *  - 导出的是 Svelte store，初始为静态兜底值（保证 SSR/首屏可用）
 *  - 应用启动（根布局 onMount）调用 loadAllDicts() 从后端字典 API 加载后更新 store
 *  - 字典管理页可增删改，刷新页面后自动生效
 */
import { writable, type Writable } from 'svelte/store';
import { dictsApi } from '$lib/api/dicts';

export interface SelectOption {
  label: string;
  value: string;
}

// 静态兜底选项（字典加载失败或未加载时使用）
const FALLBACKS: Record<string, SelectOption[]> = {
  server_status: [
    { label: '运行中', value: 'active' },
    { label: '已退役', value: 'retired' },
  ],
  hosting_type: [
    { label: '托管', value: 'colocation' },
    { label: '租赁', value: 'rental' },
    { label: '云', value: 'cloud' },
    { label: '自有', value: 'self_owned' },
    { label: '其他', value: 'other' },
  ],
  server_type: [
    { label: '物理机', value: 'physical' },
    { label: '虚拟机', value: 'virtual' },
    { label: '云服务器', value: 'cloud' },
    { label: '容器', value: 'container' },
    { label: '其他', value: 'other' },
  ],
  server_role: [
    { label: 'Web', value: 'web' },
    { label: '数据库', value: 'db' },
    { label: '缓存', value: 'cache' },
    { label: 'Worker', value: 'worker' },
    { label: '文件', value: 'file' },
    { label: '监控', value: 'monitor' },
    { label: '备份', value: 'backup' },
    { label: '其他', value: 'other' },
  ],
  architecture: [
    { label: 'x86_64', value: 'x86_64' },
    { label: 'arm64', value: 'arm64' },
  ],
  raid_level: [
    { label: 'RAID 0', value: 'raid0' },
    { label: 'RAID 1', value: 'raid1' },
    { label: 'RAID 5', value: 'raid5' },
    { label: 'RAID 6', value: 'raid6' },
    { label: 'RAID 10', value: 'raid10' },
    { label: 'RAID 50', value: 'raid50' },
    { label: 'RAID 60', value: 'raid60' },
    { label: 'JBOD', value: 'jbod' },
    { label: '软RAID', value: 'soft_raid' },
    { label: '硬RAID', value: 'hardware_raid' },
  ],
  web_server_type: [
    { label: 'Nginx', value: 'nginx' },
    { label: 'Apache', value: 'apache' },
    { label: 'IIS', value: 'iis' },
    { label: 'OpenResty', value: 'openresty' },
    { label: 'Caddy', value: 'caddy' },
    { label: 'Traefik', value: 'traefik' },
    { label: 'Tomcat', value: 'tomcat' },
    { label: '其他', value: 'other' },
  ],
  ip_status: [
    { label: '启用', value: 'enabled' },
    { label: '停用', value: 'disabled' },
    { label: '预留', value: 'reserved' },
  ],
  ip_type: [
    { label: '公网', value: 'public' },
    { label: '内网', value: 'private' },
    { label: '管理', value: 'management' },
    { label: '备份', value: 'backup' },
    { label: 'VIP', value: 'vip' },
  ],
  protocol: [
    { label: 'TCP', value: 'tcp' },
    { label: 'UDP', value: 'udp' },
    { label: 'HTTP', value: 'http' },
    { label: 'HTTPS', value: 'https' },
  ],
  provider_type: [
    { label: 'ISP', value: 'isp' },
    { label: 'IDC', value: 'idc' },
    { label: '域名', value: 'domain' },
    { label: '证书', value: 'certificate' },
    { label: '硬件', value: 'hardware' },
    { label: '软件', value: 'software' },
    { label: '云', value: 'cloud' },
    { label: '其他', value: 'other' },
  ],
  common_status: [
    { label: '活跃', value: 'active' },
    { label: '停用', value: 'inactive' },
    { label: '已归档', value: 'archived' },
  ],
  reserved_status: [
    { label: '草稿', value: 'draft' },
    { label: '活跃', value: 'active' },
    { label: '停用', value: 'inactive' },
    { label: '已归档', value: 'archived' },
  ],
  certificate_status: [
    { label: '有效', value: 'active' },
    { label: '已过期', value: 'expired' },
    { label: '已归档', value: 'archived' },
  ],
  certificate_type: [
    { label: '单域名', value: 'single' },
    { label: '多域名', value: 'multi_domain' },
    { label: '通配符', value: 'wildcard' },
    { label: '其他', value: 'other' },
  ],
  db_type: [
    { label: 'MySQL', value: 'mysql' },
    { label: 'PostgreSQL', value: 'postgresql' },
    { label: 'SQL Server', value: 'sqlserver' },
    { label: 'Oracle', value: 'oracle' },
    { label: 'Redis', value: 'redis' },
    { label: 'MongoDB', value: 'mongodb' },
    { label: '其他', value: 'other' },
  ],
  db_status: [
    { label: '运行中', value: 'active' },
    { label: '已退役', value: 'retired' },
  ],
  importance: [
    { label: '关键', value: 'critical' },
    { label: '高', value: 'high' },
    { label: '中', value: 'medium' },
    { label: '低', value: 'low' },
  ],
  site_status: [
    { label: '运行中', value: 'active' },
    { label: '临时下线', value: 'temporary_offline' },
    { label: '永久下线', value: 'permanent_offline' },
  ],
  service_target: [
    { label: '内部', value: 'internal' },
    { label: '外部', value: 'external' },
    { label: '合作伙伴', value: 'partner' },
    { label: '混合', value: 'mixed' },
  ],
  code_repo_type: [
    { label: 'Git', value: 'git' },
    { label: 'SVN', value: 'svn' },
    { label: '无', value: 'none' },
    { label: '其他', value: 'other' },
  ],
  web_framework: [
    { label: 'Django', value: 'django' },
    { label: 'Flask', value: 'flask' },
    { label: 'FastAPI', value: 'fastapi' },
    { label: 'Spring Boot', value: 'spring_boot' },
    { label: 'Express', value: 'express' },
    { label: 'Rails', value: 'rails' },
    { label: 'Laravel', value: 'laravel' },
    { label: 'ASP.NET MVC', value: 'asp_net_mvc' },
    { label: 'ASP.NET Core', value: 'asp_net_core' },
    { label: 'Gin', value: 'gin' },
    { label: 'Echo', value: 'echo' },
    { label: 'Next.js', value: 'nextjs' },
    { label: 'Nuxt.js', value: 'nuxtjs' },
    { label: 'Ant Design Pro', value: 'ant_design_pro' },
    { label: '其他', value: 'other' },
  ],
  credential_type: [
    { label: '密码', value: 'password' },
    { label: 'SSH密钥', value: 'ssh_key' },
    { label: 'API Token', value: 'api_token' },
    { label: '证书', value: 'certificate' },
    { label: '其他', value: 'other' },
  ],
  currency: [
    { label: '人民币 CNY', value: 'CNY' },
    { label: '美元 USD', value: 'USD' },
    { label: '港币 HKD', value: 'HKD' },
    { label: '欧元 EUR', value: 'EUR' },
    { label: '日元 JPY', value: 'JPY' },
    { label: '英镑 GBP', value: 'GBP' },
    { label: '新加坡元 SGD', value: 'SGD' },
    { label: '澳元 AUD', value: 'AUD' },
    { label: '离岸人民币 CNH', value: 'CNH' },
  ],
  country: [
    { label: '中国', value: 'CN' },
    { label: '美国', value: 'US' },
    { label: '日本', value: 'JP' },
    { label: '德国', value: 'DE' },
    { label: '英国', value: 'GB' },
    { label: '新加坡', value: 'SG' },
    { label: '中国香港', value: 'HK' },
    { label: '中国台湾', value: 'TW' },
    { label: '韩国', value: 'KR' },
    { label: '澳大利亚', value: 'AU' },
    { label: '荷兰', value: 'NL' },
    { label: '法国', value: 'FR' },
  ],
  monitor_type: [
    { label: 'Ping', value: 'ping' },
    { label: 'HTTP', value: 'http' },
    { label: 'TCP', value: 'tcp' },
    { label: 'TLS', value: 'tls' },
    { label: '自定义', value: 'custom' },
  ],
  contract_status: [
    { label: '草稿', value: 'draft' },
    { label: '生效中', value: 'active' },
    { label: '即将到期', value: 'expiring' },
    { label: '已过期', value: 'expired' },
    { label: '已归档', value: 'archived' },
  ],
  line_type: [
    { label: '单线', value: 'single_line' },
    { label: '双线', value: 'dual_line' },
    { label: '多线', value: 'multi_line' },
    { label: '其他', value: 'other' },
  ],
  domain_privacy_status: [
    { label: '启用', value: 'enabled' },
    { label: '停用', value: 'disabled' },
    { label: '未知', value: 'unknown' },
  ],
  site_server_role: [
    { label: 'Web', value: 'web' },
    { label: 'API', value: 'api' },
    { label: 'Worker', value: 'worker' },
    { label: '静态', value: 'static' },
    { label: '其他', value: 'other' },
  ],
  site_database_usage: [
    { label: '主', value: 'primary' },
    { label: '副本', value: 'replica' },
    { label: '分析', value: 'analytics' },
    { label: '归档', value: 'archive' },
    { label: '其他', value: 'other' },
  ],
  asset_target_type: [
    { label: '服务器', value: 'server' },
    { label: '数据库', value: 'database' },
    { label: '站点', value: 'site' },
    { label: '域名', value: 'domain' },
    { label: '证书', value: 'certificate' },
    { label: '供应商', value: 'provider' },
    { label: '数据中心', value: 'data_center' },
    { label: '其他', value: 'other' },
  ],
  backup_target_type: [
    { label: '服务器', value: 'server' },
    { label: '数据库实例', value: 'database' },
    { label: '站点', value: 'site' },
  ],
};

// 创建字典 store：初始为静态兜底
function createDictStore(dictType: string): Writable<SelectOption[]> {
  return writable<SelectOption[]>(FALLBACKS[dictType] ?? []);
}

export const serverStatusOptions = createDictStore('server_status');
export const hostingTypeOptions = createDictStore('hosting_type');
export const serverTypeOptions = createDictStore('server_type');
export const serverRoleOptions = createDictStore('server_role');
export const architectureOptions = createDictStore('architecture');
export const raidLevelOptions = createDictStore('raid_level');
export const webServerSoftwareOptions = createDictStore('web_server_type');
export const ipStatusOptions = createDictStore('ip_status');
export const ipTypeOptions = createDictStore('ip_type');
export const protocolOptions = createDictStore('protocol');
export const providerTypeOptions = createDictStore('provider_type');
export const commonStatusOptions = createDictStore('common_status');
export const reservedStatusOptions = createDictStore('reserved_status');
export const certificateStatusOptions = createDictStore('certificate_status');
export const certificateTypeOptions = createDictStore('certificate_type');
export const databaseTypeOptions = createDictStore('db_type');
export const databaseStatusOptions = createDictStore('db_status');
export const importanceOptions = createDictStore('importance');
export const siteStatusOptions = createDictStore('site_status');
export const serviceTargetOptions = createDictStore('service_target');
export const codeRepoTypeOptions = createDictStore('code_repo_type');
export const webFrameworkOptions = createDictStore('web_framework');
export const currencyOptions = createDictStore('currency');
export const countryOptions = createDictStore('country');
export const monitorTypeOptions = createDictStore('monitor_type');
export const contractStatusOptions = createDictStore('contract_status');
export const lineTypeOptions = createDictStore('line_type');
export const environmentOptions = createDictStore('environment');
export const domainRoleOptions = createDictStore('domain_role');
export const domainPrivacyStatusOptions = createDictStore('domain_privacy_status');
export const siteServerRoleOptions = createDictStore('site_server_role');
export const siteDatabaseUsageOptions = createDictStore('site_database_usage');
export const assetTargetTypeOptions = createDictStore('asset_target_type');
export const backupTargetTypeOptions = createDictStore('backup_target_type');

// 变更类型（系统行为标记，保留静态）
export const changeTypeOptions: SelectOption[] = [
  { label: '创建', value: 'create' },
  { label: '更新', value: 'update' },
  { label: '状态变更', value: 'status_change' },
  { label: '删除', value: 'delete' },
  { label: '绑定', value: 'bind' },
  { label: '解绑', value: 'unbind' },
];

// dict_type → store 注册表
const STORE_MAP: Record<string, Writable<SelectOption[]>> = {
  server_status: serverStatusOptions,
  hosting_type: hostingTypeOptions,
  server_type: serverTypeOptions,
  server_role: serverRoleOptions,
  architecture: architectureOptions,
  raid_level: raidLevelOptions,
  web_server_type: webServerSoftwareOptions,
  ip_status: ipStatusOptions,
  ip_type: ipTypeOptions,
  protocol: protocolOptions,
  provider_type: providerTypeOptions,
  common_status: commonStatusOptions,
  reserved_status: reservedStatusOptions,
  certificate_status: certificateStatusOptions,
  certificate_type: certificateTypeOptions,
  db_type: databaseTypeOptions,
  db_status: databaseStatusOptions,
  importance: importanceOptions,
  site_status: siteStatusOptions,
  service_target: serviceTargetOptions,
  code_repo_type: codeRepoTypeOptions,
  web_framework: webFrameworkOptions,
  currency: currencyOptions,
  country: countryOptions,
  monitor_type: monitorTypeOptions,
  contract_status: contractStatusOptions,
  line_type: lineTypeOptions,
  domain_privacy_status: domainPrivacyStatusOptions,
  site_server_role: siteServerRoleOptions,
  site_database_usage: siteDatabaseUsageOptions,
  asset_target_type: assetTargetTypeOptions,
  backup_target_type: backupTargetTypeOptions,
};

let dictLoaded = false;

/**
 * 从后端字典 API 加载全部字典选项并更新对应 store。
 * 幂等：只加载一次；失败时保留静态兜底值。
 */
export async function loadAllDicts(): Promise<void> {
  if (dictLoaded) return;
  try {
    const res = await dictsApi.list({ enabled_only: true });
    const byType: Record<string, SelectOption[]> = {};
    for (const item of res.data) {
      (byType[item.dict_type] ??= []).push({
        label: item.dict_label,
        value: item.dict_code,
      });
    }
    for (const [ty, opts] of Object.entries(byType)) {
      const store = STORE_MAP[ty];
      if (store) {
        store.set(opts);
      }
    }
    dictLoaded = true;
  } catch (err) {
    console.error('Failed to load dict options:', err);
  }
}

/**
 * 获取选项的显示标签
 */
export function getOptionLabel(options: SelectOption[], value: string | null | undefined): string {
  if (!value) return '-';
  const option = options.find(opt => opt.value === value);
  return option ? option.label : value;
}

/**
 * 获取多个选项的显示标签
 */
export function getOptionLabels(options: SelectOption[], values: string[] | null | undefined): string[] {
  if (!values || values.length === 0) return [];
  return values.map(v => getOptionLabel(options, v));
}
