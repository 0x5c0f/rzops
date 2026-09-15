import { api } from './client';
import type {
  DatabaseInstanceResponse,
  DatabaseInstanceListResponse,
  CreateDatabaseInstanceRequest,
  UpdateDatabaseInstanceRequest,
  ListDatabaseInstancesQuery,
} from '$lib/types/database_instance';

export const databaseInstancesApi = {
  list: (params?: ListDatabaseInstancesQuery) =>
    api.get<DatabaseInstanceListResponse>('/api/v1/database-instances', params),

  getById: (id: string) =>
    api.get<DatabaseInstanceResponse>(`/api/v1/database-instances/${id}`),

  create: (data: CreateDatabaseInstanceRequest) =>
    api.post<DatabaseInstanceResponse>('/api/v1/database-instances', data),

  update: (id: string, data: UpdateDatabaseInstanceRequest) =>
    api.put<DatabaseInstanceResponse>(`/api/v1/database-instances/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/database-instances/${id}`),
  unbind: (id: string) =>
    api.post<unknown>(`/api/v1/database-instances/${id}/unbind`),
};
