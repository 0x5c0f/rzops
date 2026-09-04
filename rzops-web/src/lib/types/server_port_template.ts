export interface ServerPortTemplateResponse {
  id: string;
  name: string;
  protocol: string;
  port: number;
  service_name: string;
  access_scope: string | null;
  is_enabled: boolean;
  description: string | null;
  created_at: string;
  updated_at: string;
}

export interface ServerPortTemplateListResponse {
  data: ServerPortTemplateResponse[];
  count: number;
}

export interface CreateServerPortTemplateRequest {
  name: string;
  protocol: string;
  port: number;
  service_name: string;
  access_scope?: string;
  is_enabled?: boolean;
  description?: string;
}

export interface UpdateServerPortTemplateRequest extends Partial<CreateServerPortTemplateRequest> {}

export interface ListServerPortTemplatesQuery {
  q?: string;
  page?: number;
  per_page?: number;
}
