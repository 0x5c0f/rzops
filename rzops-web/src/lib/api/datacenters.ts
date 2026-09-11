import { api } from './client';
import type {
  DataCenterResponse,
  DataCenterListResponse,
  CreateDataCenterRequest,
  UpdateDataCenterRequest,
  ListDataCentersQuery,
} from '$lib/types/datacenter';

export const datacentersApi = {
  list: (params?: ListDataCentersQuery) =>
    api.get<DataCenterListResponse>('/api/v1/data-centers', params),

  getById: (id: string) =>
    api.get<DataCenterResponse>(`/api/v1/data-centers/${id}`),

  create: (data: CreateDataCenterRequest) =>
    api.post<DataCenterResponse>('/api/v1/data-centers', data),

  update: (id: string, data: UpdateDataCenterRequest) =>
    api.put<DataCenterResponse>(`/api/v1/data-centers/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/data-centers/${id}`),
};
