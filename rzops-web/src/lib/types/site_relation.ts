export interface SiteServerRelationResponse {
  id: string;
  site_id: string;
  server_id: string;
  role: string | null;
  remarks: string | null;
  created_at: string;
}

export interface SiteDatabaseRelationResponse {
  id: string;
  site_id: string;
  database_instance_id: string;
  role: string | null;
  remarks: string | null;
  created_at: string;
}

export interface SiteDomainRelationResponse {
  id: string;
  site_id: string;
  domain_id: string;
  role: string | null;
  remarks: string | null;
  created_at: string;
}

export interface CreateSiteServerRelationRequest {
  site_id: string;
  server_id: string;
  role?: string;
  remarks?: string;
}

export interface CreateSiteDatabaseRelationRequest {
  site_id: string;
  database_instance_id: string;
  role?: string;
  remarks?: string;
}

export interface CreateSiteDomainRelationRequest {
  site_id: string;
  domain_id: string;
  role?: string;
  remarks?: string;
}
