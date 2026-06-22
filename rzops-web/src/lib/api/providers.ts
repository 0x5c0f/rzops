import { api } from './client';
import type {
  ProviderResponse,
  ProviderListResponse,
  CreateProviderRequest,
  UpdateProviderRequest,
  ListProvidersQuery,
} from '$lib/types/provider';

export const providersApi = {
  list: (params?: ListProvidersQuery) =>
    api.get<ProviderListResponse>('/api/v1/providers', params),

  getById: (id: string) =>
    api.get<ProviderResponse>(`/api/v1/providers/${id}`),

  create: (data: CreateProviderRequest) =>
    api.post<ProviderResponse>('/api/v1/providers', data),

  update: (id: string, data: UpdateProviderRequest) =>
    api.put<ProviderResponse>(`/api/v1/providers/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/providers/${id}`),
};
