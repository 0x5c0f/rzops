export interface OpsSiteResponse {
  id: string;
  name: string;
  url: string | null;
  service_target: string | null;
  importance: string | null;
  online_time: string | null;
  code_repo_type: string | null;
  code_repo_url: string | null;
  purpose: string | null;
  language_runtime: string | null;
  web_framework: string | null;
  is_test_site: boolean | null;
  last_backup_time: string | null;
  status: string;
  environment: string | null;
  offline_time: string | null;
  offline_reason: string | null;
  function_summary: string | null;
  remarks: string | null;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export interface OpsSiteListResponse {
  data: OpsSiteResponse[];
  count: number;
}

export interface CreateOpsSiteRequest {
  name: string;
  url?: string;
  service_target?: string;
  importance?: string;
  online_time?: string;
  code_repo_type?: string;
  code_repo_url?: string;
  purpose?: string;
  language_runtime?: string;
  web_framework?: string;
  is_test_site?: boolean;
  last_backup_time?: string;
  status?: string;
  environment?: string;
  offline_time?: string;
  offline_reason?: string;
  function_summary?: string;
  remarks?: string;
}

export interface UpdateOpsSiteRequest extends Partial<CreateOpsSiteRequest> {}

export interface ListOpsSitesQuery {
  q?: string;
  status?: string;
  environment?: string;
  importance?: string;
  server_id?: string;
  page?: number;
  per_page?: number;
}
