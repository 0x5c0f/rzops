import { api } from './client';
import type {
  AuditLogResponse,
  AuditLogListResponse,
  ListAuditLogsQuery,
} from '$lib/types/audit_log';

export const auditLogsApi = {
  list: (params?: ListAuditLogsQuery) =>
    api.get<AuditLogListResponse>('/api/v1/audit-logs', params),

  getById: (id: string) =>
    api.get<AuditLogResponse>(`/api/v1/audit-logs/${id}`),
};
