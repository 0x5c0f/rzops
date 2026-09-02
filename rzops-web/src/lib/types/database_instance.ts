export interface DatabaseInstanceResponse {
  id: string;
  server_id: string | null;
  name: string;
  db_type: string;
  description: string | null;
  status: string;
  environment: string | null;
  offline_time: string | null;
  is_self_installed: boolean | null;
  importance: string | null;
  is_ops_managed: boolean | null;
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
  environment?: string;
  offline_time?: string;
  is_self_installed?: boolean;
  importance?: string;
  is_ops_managed?: boolean;
  port?: number;
  instance_name?: string;
}

export interface UpdateDatabaseInstanceRequest extends Partial<CreateDatabaseInstanceRequest> {}

export interface ListDatabaseInstancesQuery {
  q?: string;
  status?: string;
  environment?: string;
  db_type?: string;
  server_id?: string;
  page?: number;
  per_page?: number;
}
