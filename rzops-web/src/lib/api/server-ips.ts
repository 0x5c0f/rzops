import { api } from './client';
import type {
  ServerIpResponse,
  ServerIpListResponse,
  CreateServerIpRequest,
  UpdateServerIpRequest,
  ListServerIpsQuery,
} from '$lib/types/server_ip';

export const serverIpsApi = {
  list: (params?: ListServerIpsQuery) =>
    api.get<ServerIpListResponse>('/api/v1/server-ips', params),

  getById: (id: string) =>
    api.get<ServerIpResponse>(`/api/v1/server-ips/${id}`),

  create: (data: CreateServerIpRequest) =>
    api.post<ServerIpResponse>('/api/v1/server-ips', data),

  update: (id: string, data: UpdateServerIpRequest) =>
    api.put<ServerIpResponse>(`/api/v1/server-ips/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/server-ips/${id}`),
  unbind: (id: string) =>
    api.post<unknown>(`/api/v1/server-ips/${id}/unbind`),
};
