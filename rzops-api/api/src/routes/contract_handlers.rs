use std::sync::Arc;
use axum::{extract::{Extension, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;
use rzops_domain::models::contract::Contract;
use rzops_domain::ports::contract_repository::{ContractFilter, ContractRepository};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::contract_dto::*;
use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};

fn to_resp(e:&Contract)->ContractResponse{ContractResponse{id:e.id,name:e.name.clone(),provider_id:e.provider_id,subject_type:e.subject_type.clone(),subject_id:e.subject_id,contract_no:e.contract_no.clone(),start_date:e.start_date,end_date:e.end_date,amount:e.amount.map(|v|v.to_string()),currency:e.currency.clone(),status:e.status.clone(),remarks:e.remarks.clone(),created_at:e.created_at,updated_at:e.updated_at}}

#[utoipa::path(get, path = "/api/v1/contracts/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = ContractResponse), (status = 404, body = ErrorResponse)), tag = "Contracts", security(("bearer_auth" = [])))]
pub async fn get_contract(_auth:AuthUser,State(r):State<Arc<dyn ContractRepository>>,Path(id):Path<Uuid>)->impl IntoResponse{match r.find_by_id(id).await{Ok(Some(e))=>(StatusCode::OK,Json(to_resp(&e))).into_response(),Ok(None)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}}
#[utoipa::path(get, path = "/api/v1/contracts", params(ListContractsQuery), responses((status = 200, body = ContractListResponse)), tag = "Contracts", security(("bearer_auth" = [])))]
pub async fn list_contracts(_auth:AuthUser,State(r):State<Arc<dyn ContractRepository>>,Query(q):Query<ListContractsQuery>)->impl IntoResponse{let p=q.page.unwrap_or(1).max(1);let pp=q.per_page.unwrap_or(20).min(100);let f=ContractFilter{status:q.status,q:q.q,limit:Some(pp),offset:Some((p-1)*pp)};match r.find_all(f.clone()).await{Ok(v)=>{let c=r.count(f).await.unwrap_or(0);(StatusCode::OK,Json(ContractListResponse{data:v.iter().map(to_resp).collect(),count:c})).into_response()},Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}}
#[utoipa::path(post, path = "/api/v1/contracts", request_body = CreateContractRequest, responses((status = 201, body = ContractResponse), (status = 400, body = ErrorResponse)), tag = "Contracts", security(("bearer_auth" = [])))]
pub async fn create_contract(auth:AuthUser,State(r):State<Arc<dyn ContractRepository>>,Extension(change_log):Extension<ChangeLogState>,Json(b):Json<CreateContractRequest>)->impl IntoResponse{
    let now=Utc::now();let e=Contract{id:Uuid::new_v4(),name:b.name,provider_id:b.provider_id,subject_type:b.subject_type,subject_id:b.subject_id,contract_no:b.contract_no,start_date:b.start_date,end_date:b.end_date,amount:b.amount.and_then(|v|v.parse::<Decimal>().ok()),currency:b.currency.unwrap_or_else(||"CNY".to_string()),status:b.status.unwrap_or_else(|| "draft".to_string()),remarks:b.remarks,created_at:now,updated_at:now};
    match r.create(&e).await{
        Ok(c)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Create,"contract",Some(c.id),serde_json::json!(null),serde_json::to_value(to_resp(&c)).unwrap_or(serde_json::json!({})),None).await;
            (StatusCode::CREATED,Json(to_resp(&c))).into_response()
        }
        Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()
    }
}
#[utoipa::path(put, path = "/api/v1/contracts/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateContractRequest, responses((status = 200, body = ContractResponse), (status = 404, body = ErrorResponse)), tag = "Contracts", security(("bearer_auth" = [])))]
pub async fn update_contract(auth:AuthUser,State(r):State<Arc<dyn ContractRepository>>,Extension(change_log):Extension<ChangeLogState>,Path(id):Path<Uuid>,Json(b):Json<UpdateContractRequest>)->impl IntoResponse{
    let ex=match r.find_by_id(id).await{Ok(Some(e))=>e,Ok(None)=>return(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>return(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()};
    let before_value=serde_json::to_value(to_resp(&ex)).unwrap_or(serde_json::json!({}));
    let e=Contract{id:ex.id,name:b.name.unwrap_or(ex.name),provider_id:b.provider_id.or(ex.provider_id),subject_type:b.subject_type.or(ex.subject_type),subject_id:b.subject_id.or(ex.subject_id),contract_no:b.contract_no.or(ex.contract_no),start_date:b.start_date.or(ex.start_date),end_date:b.end_date.or(ex.end_date),amount:b.amount.and_then(|v|v.parse::<Decimal>().ok()).or(ex.amount),currency:b.currency.unwrap_or(ex.currency),status:b.status.unwrap_or(ex.status),remarks:b.remarks.or(ex.remarks),created_at:ex.created_at,updated_at:Utc::now()};
    match r.update(id,&e).await{
        Ok(Some(c))=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Update,"contract",Some(c.id),before_value,serde_json::to_value(to_resp(&c)).unwrap_or(serde_json::json!({})),None).await;
            (StatusCode::OK,Json(to_resp(&c))).into_response()
        }
        Ok(None)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()
    }
}
#[utoipa::path(delete, path = "/api/v1/contracts/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "Contracts", security(("bearer_auth" = [])))]
pub async fn delete_contract(auth:AuthUser,State(r):State<Arc<dyn ContractRepository>>,Extension(change_log):Extension<ChangeLogState>,Path(id):Path<Uuid>)->impl IntoResponse{
    match r.delete(id).await{
        Ok(true)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Delete,"contract",Some(id),serde_json::json!({}),serde_json::json!(null),None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()
    }
}
