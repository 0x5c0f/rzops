export interface AuditLogResponse {
  id: string;
  actor_id: string | null;
  action: string;
  resource_type: string | null;
  resource_id: string | null;
  ip_address: string | null;
  user_agent: string | null;
  extra_data: unknown;
  created_at: string;
}

export interface AuditLogListResponse {
  data: AuditLogResponse[];
  count: number;
}

export interface ListAuditLogsQuery {
  q?: string;
  actor_id?: string;
  resource_type?: string;
  limit?: number;
  offset?: number;
}
