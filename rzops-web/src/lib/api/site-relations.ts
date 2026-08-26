import { api } from './client';
import type {
  SiteServerRelationResponse,
  SiteDatabaseRelationResponse,
  SiteDomainRelationResponse,
  CreateSiteServerRelationRequest,
  CreateSiteDatabaseRelationRequest,
  CreateSiteDomainRelationRequest,
} from '$lib/types/site_relation';

export const siteRelationsApi = {
  // Site-Server
  listServers: (siteId: string) =>
    api.get<SiteServerRelationResponse[]>(`/api/v1/site-relations/site-servers/${siteId}`),

  createServer: (data: CreateSiteServerRelationRequest) =>
    api.post<SiteServerRelationResponse>('/api/v1/site-relations/site-servers', data),

  deleteServer: (id: string) =>
    api.delete<void>(`/api/v1/site-relations/site-servers/by-id/${id}`),

  // Site-Database
  listDatabases: (siteId: string) =>
    api.get<SiteDatabaseRelationResponse[]>(`/api/v1/site-relations/site-databases/${siteId}`),

  createDatabase: (data: CreateSiteDatabaseRelationRequest) =>
    api.post<SiteDatabaseRelationResponse>('/api/v1/site-relations/site-databases', data),

  deleteDatabase: (id: string) =>
    api.delete<void>(`/api/v1/site-relations/site-databases/by-id/${id}`),

  // Site-Domain
  listDomains: (siteId: string) =>
    api.get<SiteDomainRelationResponse[]>(`/api/v1/site-relations/site-domains/${siteId}`),

  createDomain: (data: CreateSiteDomainRelationRequest) =>
    api.post<SiteDomainRelationResponse>('/api/v1/site-relations/site-domains', data),

  deleteDomain: (id: string) =>
    api.delete<void>(`/api/v1/site-relations/site-domains/by-id/${id}`),
};
