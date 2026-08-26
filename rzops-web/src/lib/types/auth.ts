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
}

export interface MeResponse extends UserInfo {}
