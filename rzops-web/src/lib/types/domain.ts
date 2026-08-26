export interface DomainResponse {
  id: string;
  domain_name: string;
  business_unit_id: string | null;
  company_id: string | null;
  expiry_date: string | null;
  renewal_amount: string | null;
  renewal_currency: string | null;
  provider_id: string | null;
  account_credential_id: string | null;
  platform_phone: string | null;
  domain_email: string | null;
  privacy_status: string | null;
  is_enabled: boolean;
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
  business_unit_id?: string;
  company_id?: string;
  expiry_date?: string;
  renewal_amount?: string;
  renewal_currency?: string;
  provider_id?: string;
  account_credential_id?: string;
  platform_phone?: string;
  domain_email?: string;
  privacy_status?: string;
  is_enabled?: boolean;
  remarks?: string;
}

export interface UpdateDomainRequest extends Partial<CreateDomainRequest> {}

export interface ListDomainsQuery {
  q?: string;
  is_enabled?: boolean;
  page?: number;
  per_page?: number;
}
