import { api } from './client';
import type {
  RoleListResponse,
  RoleDetailResponse,
  CreateRoleRequest,
  UpdateRoleRequest,
} from '$lib/types/role';

export const rolesApi = {
  list: () => api.get<RoleListResponse>('/api/v1/roles'),

  get: (id: string) => api.get<RoleDetailResponse>(`/api/v1/roles/${id}`),

  create: (body: CreateRoleRequest) => api.post<RoleDetailResponse>('/api/v1/roles', body),

  update: (id: string, body: UpdateRoleRequest) => api.put<RoleDetailResponse>(`/api/v1/roles/${id}`, body),

  delete: (id: string) => api.delete<{ ok: boolean }>(`/api/v1/roles/${id}`),
};
