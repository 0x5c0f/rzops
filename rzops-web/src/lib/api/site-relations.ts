import { api } from './client';
import type {
  SiteServerRelationResponse,
  SiteDatabaseRelationResponse,
  SiteDomainRelationResponse,
  CreateSiteServerRelationRequest,
  CreateSiteDatabaseRelationRequest,
  CreateSiteDomainRelationRequest,
} from '$lib/types/site_relation';

export interface SiteRefByServer {
  relation_id: string;
  site_id: string;
  site_name: string;
  deploy_role: string | null;
  is_primary: boolean;
}

export interface SiteRefByDatabase {
  site_id: string;
  site_name: string;
  usage_type: string | null;
  is_primary: boolean;
}

export const siteRelationsApi = {
  // 反向查询：某资源所属站点
  listSitesByServer: (serverId: string) =>
    api.get<SiteRefByServer[]>(`/api/v1/site-relations/servers/${serverId}/sites`),

  listSitesByDatabase: (databaseInstanceId: string) =>
    api.get<SiteRefByDatabase[]>(`/api/v1/site-relations/databases/${databaseInstanceId}/sites`),

  // Site-Server
  listServers: (siteId: string) =>
    api.get<SiteServerRelationResponse[]>(`/api/v1/site-relations/site-servers/${siteId}`),

  createServer: (data: CreateSiteServerRelationRequest) =>
    api.post<SiteServerRelationResponse>('/api/v1/site-relations/site-servers', data),

  updateServer: (id: string, data: { deploy_role?: string; is_primary?: boolean }) =>
    api.put<{ id: string }>(`/api/v1/site-relations/site-servers/by-id/${id}`, data),

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
