export interface ServerPortResponse {
  id: string;
  server_id: string;
  protocol: string;
  port: number;
  service_name: string;
  access_scope: string | null;
  is_enabled: boolean;
  description: string | null;
  created_at: string;
  updated_at: string;
  server_name: string | null;
  server_status: string | null;
}

export interface ServerPortListResponse {
  data: ServerPortResponse[];
  count: number;
}

export interface CreateServerPortRequest {
  server_id: string;
  protocol: string;
  port: number;
  service_name: string;
  access_scope?: string;
  is_enabled?: boolean;
  description?: string;
}

export interface UpdateServerPortRequest extends Partial<CreateServerPortRequest> {}

export interface ListServerPortsQuery {
  server_id?: string;
  protocol?: string;
  q?: string;
  page?: number;
  per_page?: number;
}

export interface ApplyTemplateRequest {
  template_id: string;
  server_ids: string[];
}
