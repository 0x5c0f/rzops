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

  /** 真实文件上传（multipart），元数据由后端自动生成 */
  upload: async (file: File, opts?: { target_type?: string; target_id?: string; remarks?: string }): Promise<AttachmentResponse> => {
    const form = new FormData();
    form.append('file', file);
    if (opts?.target_type) form.append('target_type', opts.target_type);
    if (opts?.target_id) form.append('target_id', opts.target_id);
    if (opts?.remarks) form.append('remarks', opts.remarks);
    const token = typeof window !== 'undefined' ? localStorage.getItem('token') : null;
    const res = await fetch('/api/v1/attachments/upload', {
      method: 'POST',
      headers: token ? { Authorization: `Bearer ${token}` } : {},
      body: form,
    });
    if (!res.ok) {
      const err = await res.json().catch(() => ({ error: '上传失败' }));
      throw new Error(err.error ?? `HTTP ${res.status}`);
    }
    return res.json() as Promise<AttachmentResponse>;
  },

  /** 下载附件（带 token 拉取后触发浏览器下载） */
  download: async (id: string, filename: string): Promise<void> => {
    const token = typeof window !== 'undefined' ? localStorage.getItem('token') : null;
    const res = await fetch(`/api/v1/attachments/${id}/download`, {
      headers: token ? { Authorization: `Bearer ${token}` } : {},
    });
    if (!res.ok) throw new Error('下载失败');
    const blob = await res.blob();
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    a.remove();
    URL.revokeObjectURL(url);
  },
};
