export interface RecycleItem {
  resource_type: string;
  id: string;
  name: string;
  deleted_at: string;
  /** 删除前的完整数据快照（该表整行转 JSON） */
  data?: Record<string, unknown> | null;
}

export interface RecycleListResponse {
  data: RecycleItem[];
  count: number;
}

export interface ListRecycleQuery {
  resource_type?: string;
  q?: string;
  page?: number;
  per_page?: number;
}

export interface RestoreRequest {
  resource_type: string;
  id: string;
}

/** 回收站资源类型标签映射 */
export const resourceTypeLabels: Record<string, string> = {
  server: '服务器',
  datacenter: '数据中心',
  provider: '供应商',
  domain: '域名',
  certificate: '证书',
  server_ip: '服务器IP',
  server_port: '服务器端口',
  server_port_template: '端口模板',
  ops_site: '站点',
  database_instance: '数据库实例',
  backup_plan: '备份计划',
  monitor_target: '监控目标',
  attachment: '附件',
  contract: '合同',
  dict: '字典',
  user: '用户',
  role: '角色',
};
