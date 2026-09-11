export interface LoginRequest {
  email: string;
  password: string;
}

export interface RegisterRequest {
  email: string;
  password: string;
  full_name: string;
}

export interface AuthResponse {
  access_token: string;
  token_type: string;
  user: UserInfo;
}

export interface UserInfo {
  id: string;
  email: string;
  full_name: string;
  is_superuser: boolean;
  /** 用户拥有的角色 code（多角色合并） */
  roles: string[];
  /** 用户拥有的权限点集合（跨角色去重） */
  permissions: string[];
}

export interface MeResponse extends UserInfo {}
