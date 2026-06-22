use std::sync::Arc;
use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;
use rzops_domain::enums::ReservedStatus;
use rzops_domain::models::backup_plan::BackupPlan;
use rzops_domain::ports::backup_plan_repository::{BackupPlanFilter, BackupPlanRepository};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::backup_plan_dto::*;
use crate::auth_extractor::AuthUser;

fn parse_status(s: &str) -> ReservedStatus { match s { "draft"=>ReservedStatus::Draft,"active"=>ReservedStatus::Active,"inactive"=>ReservedStatus::Inactive,"archived"=>ReservedStatus::Archived,_=>ReservedStatus::Draft } }
fn status_to_string(s: &ReservedStatus) -> String { match s { ReservedStatus::Draft=>"draft",ReservedStatus::Active=>"active",ReservedStatus::Inactive=>"inactive",ReservedStatus::Archived=>"archived" }.to_string() }
fn to_resp(e: &BackupPlan) -> BackupPlanResponse {
    BackupPlanResponse { id:e.id,name:e.name.clone(),target_type:e.target_type.clone(),target_id:e.target_id,schedule:e.schedule.clone(),retention_days:e.retention_days,status:status_to_string(&e.status),remarks:e.remarks.clone(),created_at:e.created_at,updated_at:e.updated_at }
}
#[utoipa::path(get, path = "/api/v1/backup-plans/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = BackupPlanResponse), (status = 404, body = ErrorResponse)), tag = "BackupPlans", security(("bearer_auth" = [])))]
pub async fn get_backup_plan(_auth: AuthUser, State(r): State<Arc<dyn BackupPlanRepository>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match r.find_by_id(id).await{Ok(Some(e))=>(StatusCode::OK,Json(to_resp(&e))).into_response(),Ok(None)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}
}
#[utoipa::path(get, path = "/api/v1/backup-plans", params(ListBackupPlansQuery), responses((status = 200, body = BackupPlanListResponse)), tag = "BackupPlans", security(("bearer_auth" = [])))]
pub async fn list_backup_plans(_auth: AuthUser, State(r): State<Arc<dyn BackupPlanRepository>>, Query(q): Query<ListBackupPlansQuery>) -> impl IntoResponse {
    let p=q.page.unwrap_or(1).max(1);let pp=q.per_page.unwrap_or(20).min(100);
    let f=BackupPlanFilter{status:q.status,q:q.q,limit:Some(pp),offset:Some((p-1)*pp)};
    match r.find_all(f.clone()).await{Ok(v)=>{let c=r.count(f).await.unwrap_or(0);(StatusCode::OK,Json(BackupPlanListResponse{data:v.iter().map(to_resp).collect(),count:c})).into_response()},Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}
}
#[utoipa::path(post, path = "/api/v1/backup-plans", request_body = CreateBackupPlanRequest, responses((status = 201, body = BackupPlanResponse), (status = 400, body = ErrorResponse)), tag = "BackupPlans", security(("bearer_auth" = [])))]
pub async fn create_backup_plan(_auth: AuthUser, State(r): State<Arc<dyn BackupPlanRepository>>, Json(b): Json<CreateBackupPlanRequest>) -> impl IntoResponse {
    let now=Utc::now();let e=BackupPlan{id:Uuid::new_v4(),name:b.name,target_type:b.target_type,target_id:b.target_id,schedule:b.schedule,retention_days:b.retention_days,status:b.status.as_deref().map(parse_status).unwrap_or(ReservedStatus::Draft),remarks:b.remarks,created_at:now,updated_at:now};
    match r.create(&e).await{Ok(c)=>(StatusCode::CREATED,Json(to_resp(&c))).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}
}
#[utoipa::path(put, path = "/api/v1/backup-plans/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateBackupPlanRequest, responses((status = 200, body = BackupPlanResponse), (status = 404, body = ErrorResponse)), tag = "BackupPlans", security(("bearer_auth" = [])))]
pub async fn update_backup_plan(_auth: AuthUser, State(r): State<Arc<dyn BackupPlanRepository>>, Path(id): Path<Uuid>, Json(b): Json<UpdateBackupPlanRequest>) -> impl IntoResponse {
    let ex=match r.find_by_id(id).await{Ok(Some(e))=>e,Ok(None)=>return(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>return(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()};
    let e=BackupPlan{id:ex.id,name:b.name.unwrap_or(ex.name),target_type:b.target_type.or(ex.target_type),target_id:b.target_id.or(ex.target_id),schedule:b.schedule.or(ex.schedule),retention_days:b.retention_days.or(ex.retention_days),status:b.status.as_deref().map(parse_status).unwrap_or(ex.status),remarks:b.remarks.or(ex.remarks),created_at:ex.created_at,updated_at:Utc::now()};
    match r.update(id,&e).await{Ok(Some(c))=>(StatusCode::OK,Json(to_resp(&c))).into_response(),Ok(None)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}
}
#[utoipa::path(delete, path = "/api/v1/backup-plans/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "BackupPlans", security(("bearer_auth" = [])))]
pub async fn delete_backup_plan(_auth: AuthUser, State(r): State<Arc<dyn BackupPlanRepository>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match r.delete(id).await{Ok(true)=>StatusCode::NO_CONTENT.into_response(),Ok(false)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}
}
