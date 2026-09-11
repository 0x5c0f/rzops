export interface CertificateDomainResponse {
  id: string;
  certificate_id: string;
  domain_id: string | null;
  domain_pattern: string;
  is_primary: boolean;
  created_at: string;
}

export interface CertificateDomainListResponse {
  data: CertificateDomainResponse[];
  count: number;
}

export interface CreateCertificateDomainRequest {
  certificate_id: string;
  domain_id?: string;
  domain_pattern: string;
  is_primary?: boolean;
}

export interface UpdateCertificateDomainRequest {
  domain_id?: string | null;
  domain_pattern?: string;
  is_primary?: boolean;
}

export interface ListCertificateDomainsQuery {
  certificate_id?: string;
  domain_id?: string;
  q?: string;
  page?: number;
  per_page?: number;
}
