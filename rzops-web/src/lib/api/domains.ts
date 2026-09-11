import { api } from './client';
import type {
  DomainResponse,
  DomainListResponse,
  CreateDomainRequest,
  UpdateDomainRequest,
  ListDomainsQuery,
} from '$lib/types/domain';

export const domainsApi = {
  list: (params?: ListDomainsQuery) =>
    api.get<DomainListResponse>('/api/v1/domains', params),

  getById: (id: string) =>
    api.get<DomainResponse>(`/api/v1/domains/${id}`),

  create: (data: CreateDomainRequest) =>
    api.post<DomainResponse>('/api/v1/domains', data),

  update: (id: string, data: UpdateDomainRequest) =>
    api.put<DomainResponse>(`/api/v1/domains/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/domains/${id}`),
};
