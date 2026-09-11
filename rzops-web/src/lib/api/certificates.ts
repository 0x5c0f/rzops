import { api } from './client';
import type {
  CertificateResponse,
  CertificateListResponse,
  CreateCertificateRequest,
  UpdateCertificateRequest,
  ListCertificatesQuery,
} from '$lib/types/certificate';

export const certificatesApi = {
  list: (params?: ListCertificatesQuery) =>
    api.get<CertificateListResponse>('/api/v1/certificates', params),

  getById: (id: string) =>
    api.get<CertificateResponse>(`/api/v1/certificates/${id}`),

  create: (data: CreateCertificateRequest) =>
    api.post<CertificateResponse>('/api/v1/certificates', data),

  update: (id: string, data: UpdateCertificateRequest) =>
    api.put<CertificateResponse>(`/api/v1/certificates/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/certificates/${id}`),
};
