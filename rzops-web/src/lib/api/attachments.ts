import { api } from './client';
import type {
  AttachmentResponse,
  AttachmentListResponse,
  CreateAttachmentRequest,
  ListAttachmentsQuery,
} from '$lib/types/attachment';

export const attachmentsApi = {
  list: (params?: ListAttachmentsQuery) =>
    api.get<AttachmentListResponse>('/api/v1/attachments', params),

  getById: (id: string) =>
    api.get<AttachmentResponse>(`/api/v1/attachments/${id}`),

  create: (data: CreateAttachmentRequest) =>
    api.post<AttachmentResponse>('/api/v1/attachments', data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/attachments/${id}`),
};
