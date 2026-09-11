import { api } from './client';
import type {
  ChangeRecordResponse,
  ChangeRecordListResponse,
  ListChangeRecordsQuery,
} from '$lib/types/change_record';

export const changeRecordsApi = {
  list: (params?: ListChangeRecordsQuery) =>
    api.get<ChangeRecordListResponse>('/api/v1/change-records', params),

  getById: (id: string) =>
    api.get<ChangeRecordResponse>(`/api/v1/change-records/${id}`),
};
