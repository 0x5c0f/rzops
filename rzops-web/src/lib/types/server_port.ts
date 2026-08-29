export interface ServerBrief {
  id: string;
  name: string;
}

export interface ServerPortResponse {
  id: string;
  protocol: string;
  port: number;
  service_name: string;
  access_scope: string | null;
  is_enabled: boolean;
  description: string | null;
  created_at: string;
  updated_at: string;
  server_ids: string[];
  servers: ServerBrief[];
}

export interface ServerPortListResponse {
  data: ServerPortResponse[];
  count: number;
}

export interface CreateServerPortRequest {
  server_ids: string[];
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
