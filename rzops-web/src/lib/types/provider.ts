export interface ProviderResponse {
  id: string;
  name: string;
  provider_type: string | null;
  contact_name: string | null;
  contact_phone: string | null;
  contact_email: string | null;
  contact_qq: string | null;
  fax: string | null;
  address: string | null;
  country: string | null;
  description: string | null;
  website: string | null;
  remarks: string | null;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface ProviderListResponse {
  data: ProviderResponse[];
  count: number;
}

export interface CreateProviderRequest {
  name: string;
  provider_type?: string;
  contact_name?: string;
  contact_phone?: string;
  contact_email?: string;
  contact_qq?: string;
  fax?: string;
  address?: string;
  country?: string;
  description?: string;
  website?: string;
  remarks?: string;
  status?: string;
}

export interface UpdateProviderRequest extends Partial<CreateProviderRequest> {}

export interface ListProvidersQuery {
  q?: string;
  status?: string;
  limit?: number;
  offset?: number;
}
