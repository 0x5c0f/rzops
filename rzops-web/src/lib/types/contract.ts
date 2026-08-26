export interface ContractResponse {
  id: string;
  name: string;
  provider_id: string | null;
  subject_type: string | null;
  subject_id: string | null;
  contract_no: string | null;
  start_date: string | null;
  end_date: string | null;
  amount: string | null;
  currency: string | null;
  status: string;
  remarks: string | null;
  created_at: string;
  updated_at: string;
}

export interface ContractListResponse {
  data: ContractResponse[];
  count: number;
}

export interface CreateContractRequest {
  name: string;
  provider_id?: string;
  subject_type?: string;
  subject_id?: string;
  contract_no?: string;
  start_date?: string;
  end_date?: string;
  amount?: string;
  currency?: string;
  status?: string;
  remarks?: string;
}

export interface UpdateContractRequest extends Partial<CreateContractRequest> {}

export interface ListContractsQuery {
  q?: string;
  status?: string;
  page?: number;
  per_page?: number;
}
