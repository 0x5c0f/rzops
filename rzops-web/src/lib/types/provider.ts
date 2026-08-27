export interface ProviderResponse {
  id: string;
  name: string;
  provider_types: string[];
  contact_name: string | null;
  contact_phone: string | null;
  contact_qq: string | null;
  fax: string | null;
  address: string | null;
  website: string | null;
  description: string | null;
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
  provider_types?: string[];
  contact_name?: string;
  contact_phone?: string;
  contact_qq?: string;
  fax?: string;
  address?: string;
  website?: string;
  description?: string;
  status?: string;
}

export interface UpdateProviderRequest extends Partial<CreateProviderRequest> {}

export interface ListProvidersQuery {
  q?: string;
  status?: string;
  page?: number;
  per_page?: number;
}
