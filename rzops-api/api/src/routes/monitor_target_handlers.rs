use std::sync::Arc;
use axum::{extract::{Extension, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;
use rzops_domain::models::monitor_target::MonitorTarget;
use rzops_domain::ports::monitor_target_repository::{MonitorTargetFilter, MonitorTargetRepository};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::monitor_target_dto::*;
use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};

fn to_resp(e:&MonitorTarget)->MonitorTargetResponse{MonitorTargetResponse{id:e.id,name:e.name.clone(),target_type:e.target_type.clone(),target_id:e.target_id,monitor_type:e.monitor_type.clone(),endpoint:e.endpoint.clone(),interval_seconds:e.interval_seconds,status:e.status.clone(),remarks:e.remarks.clone(),created_at:e.created_at,updated_at:e.updated_at}}

#[utoipa::path(get, path = "/api/v1/monitor-targets/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = MonitorTargetResponse), (status = 404, body = ErrorResponse)), tag = "MonitorTargets", security(("bearer_auth" = [])))]
pub async fn get_monitor_target(_auth: AuthUser, State(r):State<Arc<dyn MonitorTargetRepository>>,Path(id):Path<Uuid>)->impl IntoResponse{match r.find_by_id(id).await{Ok(Some(e))=>(StatusCode::OK,Json(to_resp(&e))).into_response(),Ok(None)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}}
#[utoipa::path(get, path = "/api/v1/monitor-targets", params(ListMonitorTargetsQuery), responses((status = 200, body = MonitorTargetListResponse)), tag = "MonitorTargets", security(("bearer_auth" = [])))]
pub async fn list_monitor_targets(_auth: AuthUser, State(r):State<Arc<dyn MonitorTargetRepository>>,Query(q):Query<ListMonitorTargetsQuery>)->impl IntoResponse{let p=q.page.unwrap_or(1).max(1);let pp=q.per_page.unwrap_or(20).min(100);let f=MonitorTargetFilter{status:q.status,q:q.q,limit:Some(pp),offset:Some((p-1)*pp)};match r.find_all(f.clone()).await{Ok(v)=>{let c=r.count(f).await.unwrap_or(0);(StatusCode::OK,Json(MonitorTargetListResponse{data:v.iter().map(to_resp).collect(),count:c})).into_response()},Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}}
#[utoipa::path(post, path = "/api/v1/monitor-targets", request_body = CreateMonitorTargetRequest, responses((status = 201, body = MonitorTargetResponse), (status = 400, body = ErrorResponse)), tag = "MonitorTargets", security(("bearer_auth" = [])))]
pub async fn create_monitor_target(auth: AuthUser, State(r):State<Arc<dyn MonitorTargetRepository>>, Extension(change_log): Extension<ChangeLogState>, Json(b):Json<CreateMonitorTargetRequest>)->impl IntoResponse{
    let now=Utc::now();let e=MonitorTarget{id:Uuid::new_v4(),name:b.name,target_type:b.target_type,target_id:b.target_id,monitor_type:b.monitor_type,endpoint:b.endpoint,interval_seconds:b.interval_seconds,status:b.status.unwrap_or_else(|| "draft".to_string()),remarks:b.remarks,created_at:now,updated_at:now};
    match r.create(&e).await{
        Ok(c)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Create,"monitor_target",Some(c.id),serde_json::json!(null),serde_json::to_value(to_resp(&c)).unwrap_or(serde_json::json!({})),None).await;
            (StatusCode::CREATED,Json(to_resp(&c))).into_response()
        }
        Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()
    }
}
#[utoipa::path(put, path = "/api/v1/monitor-targets/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateMonitorTargetRequest, responses((status = 200, body = MonitorTargetResponse), (status = 404, body = ErrorResponse)), tag = "MonitorTargets", security(("bearer_auth" = [])))]
pub async fn update_monitor_target(auth: AuthUser, State(r):State<Arc<dyn MonitorTargetRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id):Path<Uuid>, Json(b):Json<UpdateMonitorTargetRequest>)->impl IntoResponse{
    let ex=match r.find_by_id(id).await{Ok(Some(e))=>e,Ok(None)=>return(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>return(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()};
    let before_value=serde_json::to_value(to_resp(&ex)).unwrap_or(serde_json::json!({}));
    let e=MonitorTarget{id:ex.id,name:b.name.unwrap_or(ex.name),target_type:b.target_type.or(ex.target_type),target_id:b.target_id.or(ex.target_id),monitor_type:b.monitor_type.or(ex.monitor_type),endpoint:b.endpoint.or(ex.endpoint),interval_seconds:b.interval_seconds.or(ex.interval_seconds),status:b.status.unwrap_or(ex.status),remarks:b.remarks.or(ex.remarks),created_at:ex.created_at,updated_at:Utc::now()};
    match r.update(id,&e).await{
        Ok(Some(c))=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Update,"monitor_target",Some(c.id),before_value,serde_json::to_value(to_resp(&c)).unwrap_or(serde_json::json!({})),None).await;
            (StatusCode::OK,Json(to_resp(&c))).into_response()
        }
        Ok(None)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()
    }
}
#[utoipa::path(delete, path = "/api/v1/monitor-targets/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "MonitorTargets", security(("bearer_auth" = [])))]
pub async fn delete_monitor_target(auth: AuthUser, State(r):State<Arc<dyn MonitorTargetRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id):Path<Uuid>)->impl IntoResponse{
    match r.delete(id).await{
        Ok(true)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Delete,"monitor_target",Some(id),serde_json::json!({}),serde_json::json!(null),None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()
    }
}
