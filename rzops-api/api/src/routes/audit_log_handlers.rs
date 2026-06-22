use std::sync::Arc;
use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;
use rzops_domain::ports::audit_log_repository::{AuditLogFilter, AuditLogRepository};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::audit_log_dto::*;
use crate::auth_extractor::AuthUser;

fn to_resp(e:&rzops_domain::models::audit_log::AuditLog)->AuditLogResponse{AuditLogResponse{id:e.id,actor_id:e.actor_id,action:e.action.clone(),resource_type:e.resource_type.clone(),resource_id:e.resource_id,ip_address:e.ip_address.clone(),user_agent:e.user_agent.clone(),extra_data:e.extra_data.clone(),created_at:e.created_at}}

#[utoipa::path(get, path = "/api/v1/audit-logs/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = AuditLogResponse), (status = 404, body = ErrorResponse)), tag = "AuditLog", security(("bearer_auth" = [])))]
pub async fn get_audit_log(_auth:AuthUser,State(r):State<Arc<dyn AuditLogRepository>>,Path(id):Path<Uuid>)->impl IntoResponse{match r.find_by_id(id).await{Ok(Some(e))=>(StatusCode::OK,Json(to_resp(&e))).into_response(),Ok(None)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}}
#[utoipa::path(get, path = "/api/v1/audit-logs", params(ListAuditLogsQuery), responses((status = 200, body = AuditLogListResponse)), tag = "AuditLog", security(("bearer_auth" = [])))]
pub async fn list_audit_logs(_auth:AuthUser,State(r):State<Arc<dyn AuditLogRepository>>,Query(q):Query<ListAuditLogsQuery>)->impl IntoResponse{let p=q.page.unwrap_or(1).max(1);let pp=q.per_page.unwrap_or(20).min(100);let f=AuditLogFilter{actor_id:q.actor_id,resource_type:q.resource_type,limit:Some(pp),offset:Some((p-1)*pp)};match r.find_all(f.clone()).await{Ok(v)=>{let c=r.count(f).await.unwrap_or(0);(StatusCode::OK,Json(AuditLogListResponse{data:v.iter().map(to_resp).collect(),count:c})).into_response()},Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}}
