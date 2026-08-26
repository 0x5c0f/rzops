export interface ServerIpResponse {
  id: string;
  server_id: string;
  ip_address: string;
  ip_type: string | null;
  is_primary: boolean;
  isp_provider_id: string | null;
  description: string | null;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface ServerIpListResponse {
  data: ServerIpResponse[];
  count: number;
}

export interface CreateServerIpRequest {
  server_id: string;
  ip_address: string;
  ip_type?: string;
  is_primary?: boolean;
  isp_provider_id?: string;
  description?: string;
  status?: string;
}

export interface UpdateServerIpRequest {
  ip_address?: string;
  ip_type?: string;
  is_primary?: boolean;
  isp_provider_id?: string;
  description?: string;
  status?: string;
}

export interface ListServerIpsQuery {
  server_id?: string;
  status?: string;
  q?: string;
  limit?: number;
  offset?: number;
}
