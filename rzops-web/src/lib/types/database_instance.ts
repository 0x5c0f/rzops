export interface DatabaseInstanceResponse {
  id: string;
  server_id: string | null;
  name: string;
  db_type: string;
  description: string | null;
  status: string;
  offline_time: string | null;
  is_self_installed: boolean | null;
  importance: string | null;
  is_ops_managed: boolean | null;
  backup_plan_id: string | null;
  monitor_target_id: string | null;
  port: number | null;
  instance_name: string | null;
  created_at: string;
  updated_at: string;
}

export interface DatabaseInstanceListResponse {
  data: DatabaseInstanceResponse[];
  count: number;
}

export interface CreateDatabaseInstanceRequest {
  name: string;
  db_type: string;
  server_id?: string;
  description?: string;
  status?: string;
  offline_time?: string;
  is_self_installed?: boolean;
  importance?: string;
  is_ops_managed?: boolean;
  backup_plan_id?: string;
  monitor_target_id?: string;
  port?: number;
  instance_name?: string;
}

export interface UpdateDatabaseInstanceRequest extends Partial<CreateDatabaseInstanceRequest> {}

export interface ListDatabaseInstancesQuery {
  q?: string;
  status?: string;
  db_type?: string;
  server_id?: string;
  page?: number;
  per_page?: number;
}
