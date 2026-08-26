import { api } from './client';
import type {
  CredentialResponse,
  CredentialListResponse,
  CreateCredentialRequest,
  UpdateCredentialRequest,
  ListCredentialsQuery,
} from '$lib/types/credential';

export const credentialsApi = {
  list: (params?: ListCredentialsQuery) =>
    api.get<CredentialListResponse>('/api/v1/credentials', params),

  getById: (id: string) =>
    api.get<CredentialResponse>(`/api/v1/credentials/${id}`),

  create: (data: CreateCredentialRequest) =>
    api.post<CredentialResponse>('/api/v1/credentials', data),

  update: (id: string, data: UpdateCredentialRequest) =>
    api.put<CredentialResponse>(`/api/v1/credentials/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/credentials/${id}`),
};
