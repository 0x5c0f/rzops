import { api } from './client';

export interface DictItem {
  id: string;
  dict_type: string;
  dict_code: string;
  dict_label: string;
  sort_order: number;
  enabled: boolean;
  remark: string | null;
  created_at: string;
  updated_at: string;
}

export interface DictListResponse {
  data: DictItem[];
  count: number;
}

export interface CreateDictRequest {
  dict_type: string;
  dict_code: string;
  dict_label: string;
  sort_order?: number;
  enabled?: boolean;
  remark?: string;
}

export interface UpdateDictRequest {
  dict_label?: string;
  sort_order?: number;
  enabled?: boolean;
  remark?: string;
}

export const dictsApi = {
  list: (params?: { dict_type?: string; enabled_only?: boolean; q?: string }) =>
    api.get<DictListResponse>('/api/v1/dicts', params),

  getById: (id: string) => api.get<DictItem>(`/api/v1/dicts/${id}`),

  create: (data: CreateDictRequest) => api.post<DictItem>('/api/v1/dicts', data),

  update: (id: string, data: UpdateDictRequest) =>
    api.put<DictItem>(`/api/v1/dicts/${id}`, data),

  delete: (id: string) => api.delete<void>(`/api/v1/dicts/${id}`),
};
