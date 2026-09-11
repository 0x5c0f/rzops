export interface RoleResponse {
  id: string;
  code: string;
  name: string;
  description: string | null;
  is_builtin: boolean;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface RoleListResponse {
  data: RoleResponse[];
  count: number;
}

export interface RoleDetailResponse {
  role: RoleResponse;
  permissions: string[];
}

export interface CreateRoleRequest {
  code: string;
  name: string;
  description?: string | null;
  permissions: string[];
}

export interface UpdateRoleRequest {
  name?: string;
  description?: string | null;
  is_active?: boolean;
  permissions?: string[];
}

/** 权限点分组定义：资源 → 可勾选的操作 */
export interface PermissionResource {
  key: string;
  label: string;
  /** 是否系统级（单权限点 system:*） */
  system?: boolean;
}
