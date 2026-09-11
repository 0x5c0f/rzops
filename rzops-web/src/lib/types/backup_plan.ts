export interface BackupPlanResponse {
  id: string;
  name: string;
  target_type: string | null;
  target_id: string | null;
  target_name: string | null;
  schedule: string | null;
  retention_days: number | null;
  status: string;
  remarks: string | null;
  created_at: string;
  updated_at: string;
}

export interface BackupPlanListResponse {
  data: BackupPlanResponse[];
  count: number;
}

export interface CreateBackupPlanRequest {
  name: string;
  target_type?: string;
  target_id?: string;
  schedule?: string;
  retention_days?: number;
  status?: string;
  remarks?: string;
}

export interface UpdateBackupPlanRequest extends Partial<CreateBackupPlanRequest> {}

export interface ListBackupPlansQuery {
  q?: string;
  status?: string;
  target_type?: string;
  target_id?: string;
  page?: number;
  per_page?: number;
}
