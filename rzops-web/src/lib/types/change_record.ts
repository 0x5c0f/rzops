export interface ChangeRecordResponse {
  id: string;
  actor_id: string | null;
  change_type: string;
  resource_type: string | null;
  resource_id: string | null;
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
  q?: string;
  actor_id?: string;
  resource_type?: string;
  limit?: number;
  offset?: number;
}
