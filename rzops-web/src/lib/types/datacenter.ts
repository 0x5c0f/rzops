export interface DataCenterResponse {
  id: string;
  name: string;
  code: string | null;
  location: string | null;
  address: string | null;
  provider_id: string | null;
  tier_level: string | null;
  total_racks: number | null;
  used_racks: number | null;
  power_capacity_kw: number | null;
  contact_name: string | null;
  contact_phone: string | null;
  phone: string | null;
  country: string | null;
  province: string | null;
  city: string | null;
  line_type: string | null;
  description: string | null;
  remarks: string | null;
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
  code?: string;
  location?: string;
  address?: string;
  provider_id?: string;
  tier_level?: string;
  total_racks?: number;
  used_racks?: number;
  power_capacity_kw?: number;
  contact_name?: string;
  contact_phone?: string;
  phone?: string;
  country?: string;
  province?: string;
  city?: string;
  line_type?: string;
  description?: string;
  remarks?: string;
  status?: string;
}

export interface UpdateDataCenterRequest extends Partial<CreateDataCenterRequest> {}

export interface ListDataCentersQuery {
  q?: string;
  status?: string;
  country?: string;
  limit?: number;
  offset?: number;
}
