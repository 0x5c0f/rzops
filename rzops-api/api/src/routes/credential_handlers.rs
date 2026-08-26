use std::sync::Arc;
use axum::{extract::{Extension, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;
use rzops_domain::models::credential::Credential;
use rzops_domain::ports::credential_repository::{CredentialFilter, CredentialRepository};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::credential_dto::*;
use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};


fn to_resp(e: &Credential) -> CredentialResponse {
    CredentialResponse { id: e.id, name: e.name.clone(), credential_type: e.credential_type.clone(), username: e.username.clone(), secret_ref: e.secret_ref.clone(), owner_id: e.owner_id, status: e.status.clone(), remarks: e.remarks.clone(), created_at: e.created_at, updated_at: e.updated_at }
}

/// Snapshot for change log with sensitive field (secret_ref) stripped.
fn snapshot(e: &Credential) -> serde_json::Value {
    let mut v = serde_json::to_value(to_resp(e)).unwrap_or(serde_json::json!({}));
    if let Some(m) = v.as_object_mut() { m.remove("secret_ref"); }
    v
}

#[utoipa::path(get, path = "/api/v1/credentials/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = CredentialResponse), (status = 404, body = ErrorResponse)), tag = "Credentials", security(("bearer_auth" = [])))]
pub async fn get_credential(_auth: AuthUser, State(r): State<Arc<dyn CredentialRepository>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match r.find_by_id(id).await { Ok(Some(e)) => (StatusCode::OK, Json(to_resp(&e))).into_response(), Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse{error:"not found".into()})).into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse{error:e.to_string()})).into_response() }
}
#[utoipa::path(get, path = "/api/v1/credentials", params(ListCredentialsQuery), responses((status = 200, body = CredentialListResponse)), tag = "Credentials", security(("bearer_auth" = [])))]
pub async fn list_credentials(_auth: AuthUser, State(r): State<Arc<dyn CredentialRepository>>, Query(q): Query<ListCredentialsQuery>) -> impl IntoResponse {
    let p=q.page.unwrap_or(1).max(1); let pp=q.per_page.unwrap_or(20).min(100);
    let f=CredentialFilter{status:q.status,credential_type:q.credential_type,q:q.q,limit:Some(pp),offset:Some((p-1)*pp)};
    match r.find_all(f.clone()).await { Ok(v)=>{let c=r.count(f).await.unwrap_or(0);(StatusCode::OK,Json(CredentialListResponse{data:v.iter().map(to_resp).collect(),count:c})).into_response()},Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response() }
}
#[utoipa::path(post, path = "/api/v1/credentials", request_body = CreateCredentialRequest, responses((status = 201, body = CredentialResponse), (status = 400, body = ErrorResponse)), tag = "Credentials", security(("bearer_auth" = [])))]
pub async fn create_credential(auth: AuthUser, State(r): State<Arc<dyn CredentialRepository>>, Extension(change_log): Extension<ChangeLogState>, Json(b): Json<CreateCredentialRequest>) -> impl IntoResponse {
    let now=Utc::now(); let e=Credential{id:Uuid::new_v4(),name:b.name,credential_type:b.credential_type,username:b.username,secret_ref:b.secret_ref,owner_id:b.owner_id,status:b.status.unwrap_or_else(|| "draft".to_string()),remarks:b.remarks,created_at:now,updated_at:now};
    match r.create(&e).await {
        Ok(c)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Create,"credential",Some(c.id),serde_json::json!(null),snapshot(&c),None).await;
            (StatusCode::CREATED,Json(to_resp(&c))).into_response()
        }
        Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response() }
}
#[utoipa::path(put, path = "/api/v1/credentials/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateCredentialRequest, responses((status = 200, body = CredentialResponse), (status = 404, body = ErrorResponse)), tag = "Credentials", security(("bearer_auth" = [])))]
pub async fn update_credential(auth: AuthUser, State(r): State<Arc<dyn CredentialRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id): Path<Uuid>, Json(b): Json<UpdateCredentialRequest>) -> impl IntoResponse {
    let ex=match r.find_by_id(id).await{Ok(Some(e))=>e,Ok(None)=>return (StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>return(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()};
    let before_value=snapshot(&ex);
    let e=Credential{id:ex.id,name:b.name.unwrap_or(ex.name),credential_type:b.credential_type.unwrap_or(ex.credential_type),username:b.username.or(ex.username),secret_ref:b.secret_ref.or(ex.secret_ref),owner_id:b.owner_id.or(ex.owner_id),status:b.status.unwrap_or(ex.status),remarks:b.remarks.or(ex.remarks),created_at:ex.created_at,updated_at:Utc::now()};
    match r.update(id,&e).await{
        Ok(Some(c))=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Update,"credential",Some(c.id),before_value,snapshot(&c),None).await;
            (StatusCode::OK,Json(to_resp(&c))).into_response()
        }
        Ok(None)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}
}
#[utoipa::path(delete, path = "/api/v1/credentials/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "Credentials", security(("bearer_auth" = [])))]
pub async fn delete_credential(auth: AuthUser, State(r): State<Arc<dyn CredentialRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match r.delete(id).await{
        Ok(true)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Delete,"credential",Some(id),serde_json::json!({}),serde_json::json!(null),None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}
}
