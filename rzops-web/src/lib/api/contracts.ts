import { api } from './client';
import type {
  ContractResponse,
  ContractListResponse,
  CreateContractRequest,
  UpdateContractRequest,
  ListContractsQuery,
} from '$lib/types/contract';

export const contractsApi = {
  list: (params?: ListContractsQuery) =>
    api.get<ContractListResponse>('/api/v1/contracts', params),

  getById: (id: string) =>
    api.get<ContractResponse>(`/api/v1/contracts/${id}`),

  create: (data: CreateContractRequest) =>
    api.post<ContractResponse>('/api/v1/contracts', data),

  update: (id: string, data: UpdateContractRequest) =>
    api.put<ContractResponse>(`/api/v1/contracts/${id}`, data),

  delete: (id: string) =>
    api.delete<void>(`/api/v1/contracts/${id}`),
};
