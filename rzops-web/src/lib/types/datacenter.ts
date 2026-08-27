export interface DataCenterResponse {
  id: string;
  name: string;
  provider_id: string | null;
  phone: string | null;
  address: string | null;
  country: string | null;
  line_type: string[] | null;
  description: string | null;
  status: string;
  created_at: string;
  updated_at: string;
}

export interface DataCenterListResponse {
  data: DataCenterResponse[];
  count: number;
}

export interface CreateDataCenterRequest {
  name: string;
  provider_id?: string;
  phone?: string;
  address?: string;
  country?: string;
  line_type?: string[];
  description?: string;
  status?: string;
}

export interface UpdateDataCenterRequest extends Partial<CreateDataCenterRequest> {}

export interface ListDataCentersQuery {
  q?: string;
  status?: string;
  country?: string;
  page?: number;
  per_page?: number;
}
