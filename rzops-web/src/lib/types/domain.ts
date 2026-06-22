export interface DomainResponse {
  id: string;
  domain_name: string;
  registrar: string | null;
  registration_date: string | null;
  expiration_date: string | null;
  dns_provider: string | null;
  icp_filing_no: string | null;
  icp_filing_status: string | null;
  purpose: string | null;
  owner_id: string | null;
  is_enabled: boolean | null;
  business_unit_id: string | null;
  company_id: string | null;
  renewal_amount: string | null;
  renewal_currency: string | null;
  account_credential_id: string | null;
  platform_phone: string | null;
  domain_email: string | null;
  privacy_status: string | null;
  remarks: string | null;
  created_at: string;
  updated_at: string;
}

export interface DomainListResponse {
  data: DomainResponse[];
  count: number;
}

export interface CreateDomainRequest {
  domain_name: string;
  registrar?: string;
  registration_date?: string;
  expiration_date?: string;
  dns_provider?: string;
  icp_filing_no?: string;
  icp_filing_status?: string;
  purpose?: string;
  owner_id?: string;
  is_enabled?: boolean;
  business_unit_id?: string;
  company_id?: string;
  renewal_amount?: string;
  renewal_currency?: string;
  account_credential_id?: string;
  platform_phone?: string;
  domain_email?: string;
  privacy_status?: string;
  remarks?: string;
}

export interface UpdateDomainRequest extends Partial<CreateDomainRequest> {}

export interface ListDomainsQuery {
  q?: string;
  is_enabled?: boolean;
  limit?: number;
  offset?: number;
}
