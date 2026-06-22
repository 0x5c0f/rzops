export interface CertificateResponse {
  id: string;
  name: string;
  provider_id: string | null;
  lease_start_date: string | null;
  lease_end_date: string | null;
  certificate_type: string | null;
  status: string;
  private_key_credential_id: string | null;
  domain_id: string | null;
  domain_pattern: string | null;
  certificate_id: string | null;
  is_primary: boolean | null;
  remarks: string | null;
  created_at: string;
  updated_at: string;
}

export interface CertificateListResponse {
  data: CertificateResponse[];
  count: number;
}

export interface CreateCertificateRequest {
  name: string;
  provider_id?: string;
  lease_start_date?: string;
  lease_end_date?: string;
  certificate_type?: string;
  status?: string;
  private_key_credential_id?: string;
  domain_id?: string;
  domain_pattern?: string;
  certificate_id?: string;
  is_primary?: boolean;
  remarks?: string;
}

export interface UpdateCertificateRequest extends Partial<CreateCertificateRequest> {}

export interface ListCertificatesQuery {
  q?: string;
  status?: string;
  limit?: number;
  offset?: number;
}
