import { api } from './client';
import type {
  ServerPortResponse,
  ServerPortListResponse,
  CreateServerPortRequest,
  UpdateServerPortRequest,
  ListServerPortsQuery,
  ApplyTemplateRequest,
} from '$lib/types/server_port';

export const serverPortsApi = {
  list: (params?: ListServerPortsQuery) =>
    api.get<ServerPortListResponse>('/api/v1/server-ports', params),

  getById: (id: string) =>
    api.get<ServerPortResponse>(`/api/v1/server-ports/${id}`),

  create: (data: CreateServerPortRequest) =>
    api.post<ServerPortResponse>('/api/v1/server-ports', data),

  update: (id: string, data: UpdateServerPortRequest) =>
    api.put<ServerPortResponse>(`/api/v1/server-ports/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/server-ports/${id}`),

  /** 从端口模板批量实例化到多台服务器（每台服务器独立端口记录） */
  applyTemplate: (data: ApplyTemplateRequest) =>
    api.post<ServerPortListResponse>('/api/v1/server-ports/apply-template', data),
};
