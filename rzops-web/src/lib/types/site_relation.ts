export interface SiteServerRelationResponse {
  id: string;
  site_id: string;
  server_id: string;
  deploy_role: string | null;
  created_at: string;
}

export interface SiteDatabaseRelationResponse {
  id: string;
  site_id: string;
  database_instance_id: string;
  usage_type: string | null;
  created_at: string;
}

export interface SiteDomainRelationResponse {
  id: string;
  site_id: string;
  domain_id: string;
  domain_role: string | null;
  created_at: string;
}

export interface CreateSiteServerRelationRequest {
  site_id: string;
  server_id: string;
  deploy_role?: string;
}

export interface CreateSiteDatabaseRelationRequest {
  site_id: string;
  database_instance_id: string;
  usage_type?: string;
}

export interface CreateSiteDomainRelationRequest {
  site_id: string;
  domain_id: string;
  domain_role?: string;
}

export interface SiteRefByServerResponse {
  relation_id: string;
  site_id: string;
  site_name: string;
  deploy_role: string | null;
}

export interface SiteRefByDatabaseResponse {
  site_id: string;
  site_name: string;
  usage_type: string | null;
}
