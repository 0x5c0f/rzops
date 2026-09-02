use axum::{extract::{Extension, Path, State}, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;
use rzops_domain::models::ops_site_relation::{OpsSiteServer, OpsSiteDatabase, OpsSiteDomain};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::site_relation_dto::*;
use crate::routes::SiteRelationState;
use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};

// 反向查询：某服务器关联到的站点
#[utoipa::path(get, path = "/api/v1/site-relations/servers/{server_id}/sites", params(("server_id" = uuid::Uuid, Path)), responses((status = 200, body = [SiteRefByServerResponse]), (status = 500, body = ErrorResponse)), tag = "SiteRelation", security(("bearer_auth" = [])))]
pub async fn list_sites_by_server(_auth: AuthUser, State(st): State<SiteRelationState>, Path(server_id): Path<Uuid>) -> impl IntoResponse {
    match st.site_server.find_sites_by_server(server_id).await {
        Ok(v) => (StatusCode::OK, Json(v.iter().map(|e| SiteRefByServerResponse { relation_id: e.relation_id, site_id: e.site_id, site_name: e.site_name.clone(), deploy_role: e.deploy_role.clone() }).collect::<Vec<_>>())).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })).into_response(),
    }
}

// 反向查询：某数据库实例关联到的站点
#[utoipa::path(get, path = "/api/v1/site-relations/databases/{database_instance_id}/sites", params(("database_instance_id" = uuid::Uuid, Path)), responses((status = 200, body = [SiteRefByDatabaseResponse]), (status = 500, body = ErrorResponse)), tag = "SiteRelation", security(("bearer_auth" = [])))]
pub async fn list_sites_by_database(_auth: AuthUser, State(st): State<SiteRelationState>, Path(database_instance_id): Path<Uuid>) -> impl IntoResponse {
    match st.site_database.find_sites_by_database(database_instance_id).await {
        Ok(v) => (StatusCode::OK, Json(v.iter().map(|e| SiteRefByDatabaseResponse { site_id: e.site_id, site_name: e.site_name.clone(), usage_type: e.usage_type.clone() }).collect::<Vec<_>>())).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })).into_response(),
    }
}


// Site-Server
#[utoipa::path(get, path = "/api/v1/site-relations/site-servers/{site_id}", params(("site_id" = uuid::Uuid, Path)), responses((status = 200, body = [SiteServerRelationResponse]), (status = 500, body = ErrorResponse)), tag = "SiteRelation", security(("bearer_auth" = [])))]
pub async fn list_site_servers(_auth:AuthUser,State(st):State<SiteRelationState>,Path(site_id):Path<Uuid>)->impl IntoResponse{match st.site_server.find_by_site(site_id).await{Ok(v)=>(StatusCode::OK,Json(v.iter().map(|e|SiteServerRelationResponse{id:e.id,site_id:e.site_id,server_id:e.server_id,deploy_role:e.deploy_role.clone(),created_at:e.created_at}).collect::<Vec<_>>())).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}}
#[utoipa::path(post, path = "/api/v1/site-relations/site-servers", request_body = CreateSiteServerRelationRequest, responses((status = 201, body = SiteServerRelationResponse), (status = 500, body = ErrorResponse)), tag = "SiteRelation", security(("bearer_auth" = [])))]
pub async fn create_site_server(auth:AuthUser,State(st):State<SiteRelationState>,Extension(change_log):Extension<ChangeLogState>,Json(b):Json<CreateSiteServerRelationRequest>)->impl IntoResponse{
    let e=OpsSiteServer{id:Uuid::new_v4(),site_id:b.site_id,server_id:b.server_id,deploy_role:b.deploy_role,created_at:Utc::now()};
    match st.site_server.create(&e).await{
        Ok(c)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Bind,"site_server",Some(c.id),serde_json::json!(null),serde_json::json!({"site_id":c.site_id,"server_id":c.server_id}),None).await;
            (StatusCode::CREATED,Json(SiteServerRelationResponse{id:c.id,site_id:c.site_id,server_id:c.server_id,deploy_role:c.deploy_role.clone(),created_at:c.created_at})).into_response()
        }
        Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}
}
#[utoipa::path(put, path = "/api/v1/site-relations/site-servers/by-id/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateSiteServerRelationRequest, responses((status = 200, body = SiteServerRelationResponse), (status = 404, body = ErrorResponse)), tag = "SiteRelation", security(("bearer_auth" = [])))]
pub async fn update_site_server(auth:AuthUser,State(st):State<SiteRelationState>,Extension(change_log):Extension<ChangeLogState>,Path(id):Path<Uuid>,Json(b):Json<UpdateSiteServerRelationRequest>)->impl IntoResponse{
    match st.site_server.update(id, b.deploy_role.clone()).await{
        Ok(true)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Update,"site_server",Some(id),serde_json::json!(b),serde_json::json!(null),None).await;
            (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
        }
        Ok(false)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}
}

#[utoipa::path(delete, path = "/api/v1/site-relations/site-servers/by-id/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 204), (status = 404, body = ErrorResponse)), tag = "SiteRelation", security(("bearer_auth" = [])))]
pub async fn delete_site_server(auth:AuthUser,State(st):State<SiteRelationState>,Extension(change_log):Extension<ChangeLogState>,Path(id):Path<Uuid>)->impl IntoResponse{
    match st.site_server.delete(id).await{
        Ok(true)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Unbind,"site_server",Some(id),serde_json::json!({}),serde_json::json!(null),None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}
}

// Site-Database
#[utoipa::path(get, path = "/api/v1/site-relations/site-databases/{site_id}", params(("site_id" = uuid::Uuid, Path)), responses((status = 200, body = [SiteDatabaseRelationResponse]), (status = 500, body = ErrorResponse)), tag = "SiteRelation", security(("bearer_auth" = [])))]
pub async fn list_site_databases(_auth:AuthUser,State(st):State<SiteRelationState>,Path(site_id):Path<Uuid>)->impl IntoResponse{match st.site_database.find_by_site(site_id).await{Ok(v)=>(StatusCode::OK,Json(v.iter().map(|e|SiteDatabaseRelationResponse{id:e.id,site_id:e.site_id,database_instance_id:e.database_instance_id,usage_type:e.usage_type.clone(),created_at:e.created_at}).collect::<Vec<_>>())).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}}
#[utoipa::path(post, path = "/api/v1/site-relations/site-databases", request_body = CreateSiteDatabaseRelationRequest, responses((status = 201, body = SiteDatabaseRelationResponse), (status = 500, body = ErrorResponse)), tag = "SiteRelation", security(("bearer_auth" = [])))]
pub async fn create_site_database(auth:AuthUser,State(st):State<SiteRelationState>,Extension(change_log):Extension<ChangeLogState>,Json(b):Json<CreateSiteDatabaseRelationRequest>)->impl IntoResponse{
    let e=OpsSiteDatabase{id:Uuid::new_v4(),site_id:b.site_id,database_instance_id:b.database_instance_id,usage_type:b.usage_type,created_at:Utc::now()};
    match st.site_database.create(&e).await{
        Ok(c)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Bind,"site_database",Some(c.id),serde_json::json!(null),serde_json::json!({"site_id":c.site_id,"database_instance_id":c.database_instance_id}),None).await;
            (StatusCode::CREATED,Json(SiteDatabaseRelationResponse{id:c.id,site_id:c.site_id,database_instance_id:c.database_instance_id,usage_type:c.usage_type.clone(),created_at:c.created_at})).into_response()
        }
        Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}
}
#[utoipa::path(delete, path = "/api/v1/site-relations/site-databases/by-id/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 204), (status = 404, body = ErrorResponse)), tag = "SiteRelation", security(("bearer_auth" = [])))]
pub async fn delete_site_database(auth:AuthUser,State(st):State<SiteRelationState>,Extension(change_log):Extension<ChangeLogState>,Path(id):Path<Uuid>)->impl IntoResponse{
    match st.site_database.delete(id).await{
        Ok(true)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Unbind,"site_database",Some(id),serde_json::json!({}),serde_json::json!(null),None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}
}

// Site-Domain
#[utoipa::path(get, path = "/api/v1/site-relations/site-domains/{site_id}", params(("site_id" = uuid::Uuid, Path)), responses((status = 200, body = [SiteDomainRelationResponse]), (status = 500, body = ErrorResponse)), tag = "SiteRelation", security(("bearer_auth" = [])))]
pub async fn list_site_domains(_auth:AuthUser,State(st):State<SiteRelationState>,Path(site_id):Path<Uuid>)->impl IntoResponse{match st.site_domain.find_by_site(site_id).await{Ok(v)=>(StatusCode::OK,Json(v.iter().map(|e|SiteDomainRelationResponse{id:e.id,site_id:e.site_id,domain_id:e.domain_id,domain_role:e.domain_role.clone(),created_at:e.created_at}).collect::<Vec<_>>())).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}}
#[utoipa::path(post, path = "/api/v1/site-relations/site-domains", request_body = CreateSiteDomainRelationRequest, responses((status = 201, body = SiteDomainRelationResponse), (status = 500, body = ErrorResponse)), tag = "SiteRelation", security(("bearer_auth" = [])))]
pub async fn create_site_domain(auth:AuthUser,State(st):State<SiteRelationState>,Extension(change_log):Extension<ChangeLogState>,Json(b):Json<CreateSiteDomainRelationRequest>)->impl IntoResponse{
    let e=OpsSiteDomain{id:Uuid::new_v4(),site_id:b.site_id,domain_id:b.domain_id,domain_role:b.domain_role,created_at:Utc::now()};
    match st.site_domain.create(&e).await{
        Ok(c)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Bind,"site_domain",Some(c.id),serde_json::json!(null),serde_json::json!({"site_id":c.site_id,"domain_id":c.domain_id}),None).await;
            (StatusCode::CREATED,Json(SiteDomainRelationResponse{id:c.id,site_id:c.site_id,domain_id:c.domain_id,domain_role:c.domain_role.clone(),created_at:c.created_at})).into_response()
        }
        Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}
}
#[utoipa::path(delete, path = "/api/v1/site-relations/site-domains/by-id/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 204), (status = 404, body = ErrorResponse)), tag = "SiteRelation", security(("bearer_auth" = [])))]
pub async fn delete_site_domain(auth:AuthUser,State(st):State<SiteRelationState>,Extension(change_log):Extension<ChangeLogState>,Path(id):Path<Uuid>)->impl IntoResponse{
    match st.site_domain.delete(id).await{
        Ok(true)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Unbind,"site_domain",Some(id),serde_json::json!({}),serde_json::json!(null),None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}
}
