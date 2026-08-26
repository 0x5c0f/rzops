export interface CredentialResponse {
  id: string;
  name: string;
  credential_type: string;
  username: string | null;
  secret_ref: string | null;
  owner_id: string | null;
  status: string;
  remarks: string | null;
  created_at: string;
  updated_at: string;
}

export interface CredentialListResponse {
  data: CredentialResponse[];
  count: number;
}

export interface CreateCredentialRequest {
  name: string;
  credential_type: string;
  username?: string;
  secret_ref?: string;
  owner_id?: string;
  status?: string;
  remarks?: string;
}

export interface UpdateCredentialRequest extends Partial<CreateCredentialRequest> {}

export interface ListCredentialsQuery {
  q?: string;
  status?: string;
  credential_type?: string;
  limit?: number;
  offset?: number;
}
