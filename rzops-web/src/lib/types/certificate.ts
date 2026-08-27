export interface CertificateResponse {
  id: string;
  name: string;
  provider_id: string | null;
  lease_start_date: string | null;
  lease_end_date: string | null;
  certificate_type: string | null;
  status: string;
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
  remarks?: string;
}

export interface UpdateCertificateRequest extends Partial<CreateCertificateRequest> {}

export interface ListCertificatesQuery {
  q?: string;
  status?: string;
  page?: number;
  per_page?: number;
}
