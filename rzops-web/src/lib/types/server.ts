export interface ServerResponse {
  id: string;
  asset_code: string | null;
  name: string;
  primary_ip: string | null;
  location: string | null;
  isp_provider_id: string | null;
  data_center_id: string | null;
  hosting_type: string | null;
  is_dual_line: boolean | null;
  lease_start_date: string | null;
  lease_end_date: string | null;
  price: string | null;
  price_currency: string | null;
  server_type: string | null;
  role_tags: string[];
  is_database_server: boolean | null;
  cpu: string | null;
  memory_gb: number | null;
  is_raid: boolean | null;
  raid_level: string | null;
  disk_layout: string | null;
  hardware_config: string | null;
  architecture: string | null;
  maintainer_id: string | null;
  brand: string | null;
  warranty_info: string | null;
  operating_system: string | null;
  web_server_type: string[];
  server_provider_id: string | null;
  software_provider_id: string | null;
  status: string;
  environment: string | null;
  offline_time: string | null;
  offline_reason: string | null;
  remarks: string | null;
  created_at: string;
  updated_at: string;
}

export interface ServerListResponse {
  data: ServerResponse[];
  count: number;
}

export interface CreateServerRequest {
  name: string;
  asset_code?: string;
  primary_ip?: string;
  location?: string;
  isp_provider_id?: string;
  data_center_id?: string;
  hosting_type?: string;
  is_dual_line?: boolean;
  lease_start_date?: string;
  lease_end_date?: string;
  price?: string;
  price_currency?: string;
  server_type?: string;
  role_tags?: string[];
  is_database_server?: boolean;
  cpu?: string;
  memory_gb?: number;
  is_raid?: boolean;
  raid_level?: string;
  disk_layout?: string;
  hardware_config?: string;
  architecture?: string;
  maintainer_id?: string;
  brand?: string;
  warranty_info?: string;
  operating_system?: string;
  web_server_type?: string[];
  server_provider_id?: string;
  software_provider_id?: string;
  status?: string;
  environment?: string;
  offline_time?: string;
  offline_reason?: string;
  remarks?: string;
}

export interface UpdateServerRequest extends Partial<CreateServerRequest> {}

export interface ListServersQuery {
  q?: string;
  status?: string;
  environment?: string;
  server_type?: string;
  data_center_id?: string;
  isp_provider_id?: string;
  page?: number;
  per_page?: number;
}
