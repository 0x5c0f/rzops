import { api } from './client';
import type {
  OpsSiteResponse,
  OpsSiteListResponse,
  CreateOpsSiteRequest,
  UpdateOpsSiteRequest,
  ListOpsSitesQuery,
} from '$lib/types/ops_site';

export const opsSitesApi = {
  list: (params?: ListOpsSitesQuery) =>
    api.get<OpsSiteListResponse>('/api/v1/ops-sites', params),

  getById: (id: string) =>
    api.get<OpsSiteResponse>(`/api/v1/ops-sites/${id}`),

  create: (data: CreateOpsSiteRequest) =>
    api.post<OpsSiteResponse>('/api/v1/ops-sites', data),

  update: (id: string, data: UpdateOpsSiteRequest) =>
    api.put<OpsSiteResponse>(`/api/v1/ops-sites/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/ops-sites/${id}`),
};
