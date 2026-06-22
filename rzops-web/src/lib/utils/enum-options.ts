/**
 * 枚举选项工具
 * 将后端枚举转换为前端下拉选项
 */

export interface SelectOption {
  label: string;
  value: string;
}

// 服务器状态
export const serverStatusOptions: SelectOption[] = [
  { label: '运行中', value: 'active' },
  { label: '已退役', value: 'retired' },
];

// 托管类型
export const hostingTypeOptions: SelectOption[] = [
  { label: '托管', value: 'colocation' },
  { label: '租赁', value: 'rental' },
  { label: '云', value: 'cloud' },
  { label: '自有', value: 'self_owned' },
  { label: '其他', value: 'other' },
];

// 服务器类型
export const serverTypeOptions: SelectOption[] = [
  { label: '物理机', value: 'physical' },
  { label: '虚拟机', value: 'virtual' },
  { label: '云服务器', value: 'cloud' },
  { label: '容器', value: 'container' },
  { label: '其他', value: 'other' },
];

// 服务器角色
export const serverRoleOptions: SelectOption[] = [
  { label: 'Web', value: 'web' },
  { label: '数据库', value: 'db' },
  { label: '缓存', value: 'cache' },
  { label: 'Worker', value: 'worker' },
  { label: '文件', value: 'file' },
  { label: '监控', value: 'monitor' },
  { label: '备份', value: 'backup' },
  { label: '其他', value: 'other' },
];

// 架构
export const architectureOptions: SelectOption[] = [
  { label: 'x86_64', value: 'x86_64' },
  { label: 'arm64', value: 'arm64' },
];

// RAID级别
export const raidLevelOptions: SelectOption[] = [
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
];

// Web服务器软件
export const webServerSoftwareOptions: SelectOption[] = [
  { label: 'Nginx', value: 'nginx' },
  { label: 'Apache', value: 'apache' },
  { label: 'IIS', value: 'iis' },
  { label: 'OpenResty', value: 'openresty' },
  { label: 'Caddy', value: 'caddy' },
  { label: 'Traefik', value: 'traefik' },
  { label: 'Tomcat', value: 'tomcat' },
  { label: '其他', value: 'other' },
];

// IP状态
export const ipStatusOptions: SelectOption[] = [
  { label: '启用', value: 'enabled' },
  { label: '停用', value: 'disabled' },
  { label: '预留', value: 'reserved' },
];

// IP类型
export const ipTypeOptions: SelectOption[] = [
  { label: '公网', value: 'public' },
  { label: '内网', value: 'private' },
  { label: '管理', value: 'management' },
  { label: '备份', value: 'backup' },
  { label: 'VIP', value: 'vip' },
];

// 协议类型
export const protocolOptions: SelectOption[] = [
  { label: 'TCP', value: 'tcp' },
  { label: 'UDP', value: 'udp' },
  { label: 'HTTP', value: 'http' },
  { label: 'HTTPS', value: 'https' },
];

// 供应商类型
export const providerTypeOptions: SelectOption[] = [
  { label: 'ISP', value: 'isp' },
  { label: 'IDC', value: 'idc' },
  { label: '域名', value: 'domain' },
  { label: '证书', value: 'certificate' },
  { label: '硬件', value: 'hardware' },
  { label: '软件', value: 'software' },
  { label: '云', value: 'cloud' },
  { label: '其他', value: 'other' },
];

// 通用状态
export const commonStatusOptions: SelectOption[] = [
  { label: '活跃', value: 'active' },
  { label: '停用', value: 'inactive' },
  { label: '已归档', value: 'archived' },
];

// 证书状态
export const certificateStatusOptions: SelectOption[] = [
  { label: '有效', value: 'active' },
  { label: '已过期', value: 'expired' },
  { label: '已归档', value: 'archived' },
];

// 证书类型
export const certificateTypeOptions: SelectOption[] = [
  { label: '单域名', value: 'single' },
  { label: '多域名', value: 'multi_domain' },
  { label: '通配符', value: 'wildcard' },
  { label: '其他', value: 'other' },
];

// 数据库类型
export const databaseTypeOptions: SelectOption[] = [
  { label: 'MySQL', value: 'mysql' },
  { label: 'PostgreSQL', value: 'postgresql' },
  { label: 'SQL Server', value: 'sqlserver' },
  { label: 'Oracle', value: 'oracle' },
  { label: 'Redis', value: 'redis' },
  { label: 'MongoDB', value: 'mongodb' },
  { label: '其他', value: 'other' },
];

// 数据库状态
export const databaseStatusOptions: SelectOption[] = [
  { label: '运行中', value: 'active' },
  { label: '已退役', value: 'retired' },
];

// 重要性
export const importanceOptions: SelectOption[] = [
  { label: '关键', value: 'critical' },
  { label: '高', value: 'high' },
  { label: '中', value: 'medium' },
  { label: '低', value: 'low' },
];

// 站点状态
export const siteStatusOptions: SelectOption[] = [
  { label: '运行中', value: 'active' },
  { label: '临时下线', value: 'temporary_offline' },
  { label: '永久下线', value: 'permanent_offline' },
];

// 服务目标
export const serviceTargetOptions: SelectOption[] = [
  { label: '内部', value: 'internal' },
  { label: '外部', value: 'external' },
  { label: '合作伙伴', value: 'partner' },
  { label: '混合', value: 'mixed' },
];

// 代码仓库类型
export const codeRepoTypeOptions: SelectOption[] = [
  { label: 'Git', value: 'git' },
  { label: 'SVN', value: 'svn' },
  { label: '无', value: 'none' },
  { label: '其他', value: 'other' },
];

// Web框架
export const webFrameworkOptions: SelectOption[] = [
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
];

// 凭据类型
export const credentialTypeOptions: SelectOption[] = [
  { label: '密码', value: 'password' },
  { label: 'SSH密钥', value: 'ssh_key' },
  { label: 'API Token', value: 'api_token' },
  { label: '证书', value: 'certificate' },
  { label: '其他', value: 'other' },
];

// 监控类型
export const monitorTypeOptions: SelectOption[] = [
  { label: 'Ping', value: 'ping' },
  { label: 'HTTP', value: 'http' },
  { label: 'TCP', value: 'tcp' },
  { label: 'TLS', value: 'tls' },
  { label: '自定义', value: 'custom' },
];

// 合同状态
export const contractStatusOptions: SelectOption[] = [
  { label: '草稿', value: 'draft' },
  { label: '生效中', value: 'active' },
  { label: '即将到期', value: 'expiring' },
  { label: '已过期', value: 'expired' },
  { label: '已归档', value: 'archived' },
];

// 变更类型
export const changeTypeOptions: SelectOption[] = [
  { label: '创建', value: 'create' },
  { label: '更新', value: 'update' },
  { label: '状态变更', value: 'status_change' },
  { label: '删除', value: 'delete' },
  { label: '绑定', value: 'bind' },
  { label: '解绑', value: 'unbind' },
];

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
