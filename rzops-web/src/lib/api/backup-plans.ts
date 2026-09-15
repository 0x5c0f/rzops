import { api } from './client';
import type {
  BackupPlanResponse,
  BackupPlanListResponse,
  CreateBackupPlanRequest,
  UpdateBackupPlanRequest,
  ListBackupPlansQuery,
} from '$lib/types/backup_plan';

export const backupPlansApi = {
  list: (params?: ListBackupPlansQuery) =>
    api.get<BackupPlanListResponse>('/api/v1/backup-plans', params),

  getById: (id: string) =>
    api.get<BackupPlanResponse>(`/api/v1/backup-plans/${id}`),

  create: (data: CreateBackupPlanRequest) =>
    api.post<BackupPlanResponse>('/api/v1/backup-plans', data),

  update: (id: string, data: UpdateBackupPlanRequest) =>
    api.put<BackupPlanResponse>(`/api/v1/backup-plans/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/backup-plans/${id}`),
  unbind: (id: string) =>
    api.post<unknown>(`/api/v1/backup-plans/${id}/unbind`),
};
