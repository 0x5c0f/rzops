export interface SiteServerRelationResponse {
  id: string;
  site_id: string;
  server_id: string;
  deploy_role: string | null;
  is_primary: boolean;
  created_at: string;
}

export interface SiteDatabaseRelationResponse {
  id: string;
  site_id: string;
  database_instance_id: string;
  usage_type: string | null;
  is_primary: boolean;
  created_at: string;
}

export interface SiteDomainRelationResponse {
  id: string;
  site_id: string;
  domain_id: string;
  is_primary: boolean;
  created_at: string;
}

export interface CreateSiteServerRelationRequest {
  site_id: string;
  server_id: string;
  deploy_role?: string;
  is_primary?: boolean;
}

export interface CreateSiteDatabaseRelationRequest {
  site_id: string;
  database_instance_id: string;
  usage_type?: string;
  is_primary?: boolean;
}

export interface CreateSiteDomainRelationRequest {
  site_id: string;
  domain_id: string;
  is_primary?: boolean;
}
