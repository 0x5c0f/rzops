export interface RoleBrief {
  id: string;
  code: string;
  name: string;
  description: string | null;
  is_builtin: boolean;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface UserResponse {
  id: string;
  email: string;
  is_active: boolean;
  is_superuser: boolean;
  full_name: string | null;
  roles: RoleBrief[];
  created_at: string;
}

export interface UserListResponse {
  data: UserResponse[];
  count: number;
}

export interface CreateUserRequest {
  email: string;
  password: string;
  full_name?: string | null;
  is_active?: boolean;
  is_superuser?: boolean;
  role_ids: string[];
}

export interface UpdateUserRequest {
  full_name?: string | null;
  is_active?: boolean;
  is_superuser?: boolean;
  role_ids?: string[];
}

export interface ResetPasswordRequest {
  new_password: string;
}

export interface ListUsersQuery {
  q?: string;
  page?: number;
  per_page?: number;
}
