import { api } from './client';
import type {
  UserResponse,
  UserListResponse,
  CreateUserRequest,
  UpdateUserRequest,
  ResetPasswordRequest,
  ListUsersQuery,
} from '$lib/types/user';

export const usersApi = {
  list: (params: ListUsersQuery) => api.get<UserListResponse>('/api/v1/users', params),

  get: (id: string) => api.get<UserResponse>(`/api/v1/users/${id}`),

  create: (body: CreateUserRequest) => api.post<UserResponse>('/api/v1/users', body),

  update: (id: string, body: UpdateUserRequest) => api.put<UserResponse>(`/api/v1/users/${id}`, body),

  resetPassword: (id: string, body: ResetPasswordRequest) =>
    api.post<{ ok: boolean }>(`/api/v1/users/${id}/reset-password`, body),

  delete: (id: string) => api.delete<{ ok: boolean }>(`/api/v1/users/${id}`),
};
