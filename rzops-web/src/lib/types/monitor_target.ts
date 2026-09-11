export interface MonitorTargetResponse {
  id: string;
  name: string;
  target_type: string | null;
  target_id: string | null;
  target_name: string | null;
  monitor_type: string | null;
  endpoint: string | null;
  interval_seconds: number | null;
  status: string;
  remarks: string | null;
  created_at: string;
  updated_at: string;
}

export interface MonitorTargetListResponse {
  data: MonitorTargetResponse[];
  count: number;
}

export interface CreateMonitorTargetRequest {
  name: string;
  target_type?: string;
  target_id?: string;
  monitor_type?: string;
  endpoint?: string;
  interval_seconds?: number;
  status?: string;
  remarks?: string;
}

export interface UpdateMonitorTargetRequest extends Partial<CreateMonitorTargetRequest> {}

export interface ListMonitorTargetsQuery {
  q?: string;
  status?: string;
  target_type?: string;
  target_id?: string;
  page?: number;
  per_page?: number;
}
