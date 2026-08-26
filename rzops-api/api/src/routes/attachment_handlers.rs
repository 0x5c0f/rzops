use std::sync::Arc;
use axum::{extract::{Extension, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;
use rzops_domain::enums::ReservedStatus;
use rzops_domain::models::attachment::Attachment;
use rzops_domain::ports::attachment_repository::{AttachmentFilter, AttachmentRepository};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::attachment_dto::*;
use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};

fn parse_status(s:&str)->ReservedStatus{match s{"draft"=>ReservedStatus::Draft,"active"=>ReservedStatus::Active,"inactive"=>ReservedStatus::Inactive,"archived"=>ReservedStatus::Archived,_=>ReservedStatus::Draft}}
fn status_to_string(s:&ReservedStatus)->String{match s{ReservedStatus::Draft=>"draft",ReservedStatus::Active=>"active",ReservedStatus::Inactive=>"inactive",ReservedStatus::Archived=>"archived"}.to_string()}
fn to_resp(e:&Attachment)->AttachmentResponse{AttachmentResponse{id:e.id,filename:e.filename.clone(),target_type:e.target_type.clone(),target_id:e.target_id,storage_key:e.storage_key.clone(),content_type:e.content_type.clone(),size_bytes:e.size_bytes,uploaded_by_id:e.uploaded_by_id,status:status_to_string(&e.status),remarks:e.remarks.clone(),created_at:e.created_at,updated_at:e.updated_at}}

#[utoipa::path(get, path = "/api/v1/attachments/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = AttachmentResponse), (status = 404, body = ErrorResponse)), tag = "Attachment", security(("bearer_auth" = [])))]
pub async fn get_attachment(_auth:AuthUser,State(r):State<Arc<dyn AttachmentRepository>>,Path(id):Path<Uuid>)->impl IntoResponse{match r.find_by_id(id).await{Ok(Some(e))=>(StatusCode::OK,Json(to_resp(&e))).into_response(),Ok(None)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}}
#[utoipa::path(get, path = "/api/v1/attachments", params(ListAttachmentsQuery), responses((status = 200, body = AttachmentListResponse)), tag = "Attachment", security(("bearer_auth" = [])))]
pub async fn list_attachments(_auth:AuthUser,State(r):State<Arc<dyn AttachmentRepository>>,Query(q):Query<ListAttachmentsQuery>)->impl IntoResponse{let p=q.page.unwrap_or(1).max(1);let pp=q.per_page.unwrap_or(20).min(100);let f=AttachmentFilter{status:q.status,target_type:q.target_type,target_id:q.target_id,q:q.q,limit:Some(pp),offset:Some((p-1)*pp)};match r.find_all(f.clone()).await{Ok(v)=>{let c=r.count(f).await.unwrap_or(0);(StatusCode::OK,Json(AttachmentListResponse{data:v.iter().map(to_resp).collect(),count:c})).into_response()},Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}}
#[utoipa::path(post, path = "/api/v1/attachments", request_body = CreateAttachmentRequest, responses((status = 201, body = AttachmentResponse), (status = 400, body = ErrorResponse)), tag = "Attachment", security(("bearer_auth" = [])))]
pub async fn create_attachment(auth:AuthUser,State(r):State<Arc<dyn AttachmentRepository>>,Extension(change_log):Extension<ChangeLogState>,Json(b):Json<CreateAttachmentRequest>)->impl IntoResponse{
    let now=Utc::now();let e=Attachment{id:Uuid::new_v4(),filename:b.filename,target_type:b.target_type,target_id:b.target_id,storage_key:b.storage_key,content_type:b.content_type,size_bytes:b.size_bytes,uploaded_by_id:b.uploaded_by_id,status:b.status.as_deref().map(parse_status).unwrap_or(ReservedStatus::Draft),remarks:b.remarks,created_at:now,updated_at:now};
    match r.create(&e).await{
        Ok(c)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Create,"attachment",Some(c.id),serde_json::json!(null),serde_json::to_value(to_resp(&c)).unwrap_or(serde_json::json!({})),None).await;
            (StatusCode::CREATED,Json(to_resp(&c))).into_response()
        }
        Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()
    }
}
#[utoipa::path(put, path = "/api/v1/attachments/{id}", params(("id" = uuid::Uuid, Path)), request_body = CreateAttachmentRequest, responses((status = 200, body = AttachmentResponse), (status = 404, body = ErrorResponse)), tag = "Attachment", security(("bearer_auth" = [])))]
pub async fn update_attachment(auth:AuthUser,State(r):State<Arc<dyn AttachmentRepository>>,Extension(change_log):Extension<ChangeLogState>,Path(id):Path<Uuid>,Json(b):Json<CreateAttachmentRequest>)->impl IntoResponse{
    let ex=match r.find_by_id(id).await{Ok(Some(e))=>e,Ok(None)=>return(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>return(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()};
    let before_value=serde_json::to_value(to_resp(&ex)).unwrap_or(serde_json::json!({}));
    let e=Attachment{id:ex.id,filename:b.filename,target_type:b.target_type.or(ex.target_type),target_id:b.target_id.or(ex.target_id),storage_key:b.storage_key.or(ex.storage_key),content_type:b.content_type.or(ex.content_type),size_bytes:b.size_bytes.or(ex.size_bytes),uploaded_by_id:b.uploaded_by_id.or(ex.uploaded_by_id),status:b.status.as_deref().map(parse_status).unwrap_or(ex.status),remarks:b.remarks.or(ex.remarks),created_at:ex.created_at,updated_at:Utc::now()};
    match r.update(id,&e).await{
        Ok(Some(c))=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Update,"attachment",Some(c.id),before_value,serde_json::to_value(to_resp(&c)).unwrap_or(serde_json::json!({})),None).await;
            (StatusCode::OK,Json(to_resp(&c))).into_response()
        }
        Ok(None)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()
    }
}
#[utoipa::path(delete, path = "/api/v1/attachments/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 204), (status = 404, body = ErrorResponse)), tag = "Attachment", security(("bearer_auth" = [])))]
pub async fn delete_attachment(auth:AuthUser,State(r):State<Arc<dyn AttachmentRepository>>,Extension(change_log):Extension<ChangeLogState>,Path(id):Path<Uuid>)->impl IntoResponse{
    match r.delete(id).await{
        Ok(true)=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Delete,"attachment",Some(id),serde_json::json!({}),serde_json::json!(null),None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()
    }
}
