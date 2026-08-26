export interface ChangeRecordResponse {
  id: string;
  actor_id: string | null;
  change_type: string;
  resource_type: string | null;
  resource_id: string | null;
  actor_email: string | null;
  before_data: unknown;
  after_data: unknown;
  remarks: string | null;
  created_at: string;
}

export interface ChangeRecordListResponse {
  data: ChangeRecordResponse[];
  count: number;
}

export interface ListChangeRecordsQuery {
  change_type?: string;
  resource_type?: string;
  actor_id?: string;
  created_from?: string;
  created_to?: string;
  page?: number;
  per_page?: number;
}
