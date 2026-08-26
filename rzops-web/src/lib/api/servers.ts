import { api } from './client';
import type {
  ServerResponse,
  ServerListResponse,
  CreateServerRequest,
  UpdateServerRequest,
  ListServersQuery,
} from '$lib/types/server';

export const serversApi = {
  list: (params?: ListServersQuery) =>
    api.get<ServerListResponse>('/api/v1/servers', params),

  getById: (id: string) =>
    api.get<ServerResponse>(`/api/v1/servers/${id}`),

  create: (data: CreateServerRequest) =>
    api.post<ServerResponse>('/api/v1/servers', data),

  update: (id: string, data: UpdateServerRequest) =>
    api.put<ServerResponse>(`/api/v1/servers/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/servers/${id}`),
};
