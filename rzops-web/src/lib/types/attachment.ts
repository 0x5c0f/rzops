export interface AttachmentResponse {
  id: string;
  filename: string;
  target_type: string | null;
  target_id: string | null;
  target_name: string | null;
  storage_key: string | null;
  content_type: string | null;
  size_bytes: number | null;
  uploaded_by_id: string | null;
  uploader_name: string | null;
  status: string;
  remarks: string | null;
  created_at: string;
  updated_at: string;
}

export interface AttachmentListResponse {
  data: AttachmentResponse[];
  count: number;
}

export interface CreateAttachmentRequest {
  filename: string;
  target_type?: string;
  target_id?: string;
  storage_key?: string;
  content_type?: string;
  size_bytes?: number;
  uploaded_by_id?: string;
  status?: string;
  remarks?: string;
}

export interface ListAttachmentsQuery {
  q?: string;
  status?: string;
  target_type?: string;
  target_id?: string;
  page?: number;
  per_page?: number;
}
