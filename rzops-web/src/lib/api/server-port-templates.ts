import { api } from './client';
import type {
  ServerPortTemplateResponse,
  ServerPortTemplateListResponse,
  CreateServerPortTemplateRequest,
  UpdateServerPortTemplateRequest,
  ListServerPortTemplatesQuery,
} from '$lib/types/server_port_template';

export const serverPortTemplatesApi = {
  list: (params?: ListServerPortTemplatesQuery) =>
    api.get<ServerPortTemplateListResponse>('/api/v1/server-port-templates', params),

  getById: (id: string) =>
    api.get<ServerPortTemplateResponse>(`/api/v1/server-port-templates/${id}`),

  create: (data: CreateServerPortTemplateRequest) =>
    api.post<ServerPortTemplateResponse>('/api/v1/server-port-templates', data),

  update: (id: string, data: UpdateServerPortTemplateRequest) =>
    api.put<ServerPortTemplateResponse>(`/api/v1/server-port-templates/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/server-port-templates/${id}`),
};
