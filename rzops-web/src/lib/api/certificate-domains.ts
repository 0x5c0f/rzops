import { api } from './client';
import type {
  CertificateDomainResponse,
  CertificateDomainListResponse,
  CreateCertificateDomainRequest,
  UpdateCertificateDomainRequest,
  ListCertificateDomainsQuery,
} from '$lib/types/certificate_domain';

export const certificateDomainsApi = {
  list: (params?: ListCertificateDomainsQuery) =>
    api.get<CertificateDomainListResponse>('/api/v1/certificate-domains', params),

  getById: (id: string) =>
    api.get<CertificateDomainResponse>(`/api/v1/certificate-domains/${id}`),

  create: (data: CreateCertificateDomainRequest) =>
    api.post<CertificateDomainResponse>('/api/v1/certificate-domains', data),

  update: (id: string, data: UpdateCertificateDomainRequest) =>
    api.put<CertificateDomainResponse>(`/api/v1/certificate-domains/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/certificate-domains/${id}`),
};
