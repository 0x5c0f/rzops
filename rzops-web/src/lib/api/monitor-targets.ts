import { api } from './client';
import type {
  MonitorTargetResponse,
  MonitorTargetListResponse,
  CreateMonitorTargetRequest,
  UpdateMonitorTargetRequest,
  ListMonitorTargetsQuery,
} from '$lib/types/monitor_target';

export const monitorTargetsApi = {
  list: (params?: ListMonitorTargetsQuery) =>
    api.get<MonitorTargetListResponse>('/api/v1/monitor-targets', params),

  getById: (id: string) =>
    api.get<MonitorTargetResponse>(`/api/v1/monitor-targets/${id}`),

  create: (data: CreateMonitorTargetRequest) =>
    api.post<MonitorTargetResponse>('/api/v1/monitor-targets', data),

  update: (id: string, data: UpdateMonitorTargetRequest) =>
    api.put<MonitorTargetResponse>(`/api/v1/monitor-targets/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/monitor-targets/${id}`),
};
