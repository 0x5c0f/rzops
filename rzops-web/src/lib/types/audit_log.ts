export interface AuditLogResponse {
  id: string;
  actor_id: string | null;
  action: string;
  resource_type: string | null;
  resource_id: string | null;
  ip_address: string | null;
  user_agent: string | null;
  actor_email: string | null;
  extra_data: unknown;
  created_at: string;
}

export interface AuditLogListResponse {
  data: AuditLogResponse[];
  count: number;
}

export interface ListAuditLogsQuery {
  action?: string;
  resource_type?: string;
  actor_id?: string;
  created_from?: string;
  created_to?: string;
  page?: number;
  per_page?: number;
}
