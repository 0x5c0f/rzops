import { api } from './client';
import type {
  RecycleListResponse,
  ListRecycleQuery,
  RestoreRequest,
} from '$lib/types/recycle';

export const recycleApi = {
  list: (params: ListRecycleQuery) => api.get<RecycleListResponse>('/api/v1/recycle', params),

  restore: (body: RestoreRequest) => api.post<{ ok: boolean }>('/api/v1/recycle/restore', body),

  purge: (resourceType: string, id: string) =>
    api.delete<{ ok: boolean }>(`/api/v1/recycle/${resourceType}/${id}`),
};
