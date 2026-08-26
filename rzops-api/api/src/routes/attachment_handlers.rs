use std::sync::Arc;
use axum::{
    extract::{Extension, Multipart, Path, Query, State},
    http::{header::{CONTENT_DISPOSITION, CONTENT_TYPE}, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;
use rzops_domain::models::attachment::Attachment;
use rzops_domain::ports::attachment_repository::{AttachmentFilter, AttachmentRepository};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::attachment_dto::*;
use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};

fn to_resp(e:&Attachment)->AttachmentResponse{AttachmentResponse{id:e.id,filename:e.filename.clone(),target_type:e.target_type.clone(),target_id:e.target_id,storage_key:e.storage_key.clone(),content_type:e.content_type.clone(),size_bytes:e.size_bytes,uploaded_by_id:e.uploaded_by_id,status:e.status.clone(),remarks:e.remarks.clone(),created_at:e.created_at,updated_at:e.updated_at}}

fn content_type_from_name(name: &str, fallback: Option<&str>) -> Option<String> {
    if let Some(ct) = fallback.filter(|s| !s.is_empty()) {
        return Some(ct.to_string());
    }
    let ext = name.rsplit('.').next().unwrap_or("").to_lowercase();
    let m = match ext.as_str() {
        "txt" => "text/plain",
        "pdf" => "application/pdf",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "zip" => "application/zip",
        "tar" | "gz" => "application/gzip",
        "json" => "application/json",
        "csv" => "text/csv",
        "md" => "text/markdown",
        "pem" | "crt" | "cer" => "application/x-pem-file",
        "key" => "application/pgp-keys",
        "log" => "text/plain",
        _ => "application/octet-stream",
    };
    Some(m.to_string())
}

/// 下载附件内容
#[utoipa::path(get, path = "/api/v1/attachments/{id}/download", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = inline(AttachmentResponse)), (status = 404, body = ErrorResponse)), tag = "Attachment", security(("bearer_auth" = [])))]
pub async fn download_attachment(_auth:AuthUser,State(r):State<Arc<dyn AttachmentRepository>>,Extension(upload_dir):Extension<String>,Path(id):Path<Uuid>)->impl IntoResponse{
    let e=match r.find_by_id(id).await{Ok(Some(e))=>e,Ok(None)=>return(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>return(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()};
    let key=e.storage_key.clone().unwrap_or_default();
    if key.is_empty(){return(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"file storage_key missing".into()})).into_response()}
    let path=format!("{}/{}",upload_dir,key);
    match tokio::fs::read(&path).await{
        Ok(bytes)=>{
            let ct=e.content_type.clone().unwrap_or_else(||"application/octet-stream".into());
            let fname=e.filename.clone();
            let mut headers=HeaderMap::new();
            if let Ok(v)=HeaderValue::from_str(&ct){headers.insert(CONTENT_TYPE,v);}
            let disp=format!("attachment; filename=\"*\"; filename*=UTF-8''{}", percent_encode(&fname));
            if let Ok(v)=HeaderValue::from_str(&disp){headers.insert(CONTENT_DISPOSITION,v);}
            (StatusCode::OK,headers,bytes).into_response()
        }
        Err(_)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"file not found on disk".into()})).into_response()
    }
}

fn percent_encode(s:&str)->String{
    s.as_bytes().iter().map(|&b|{
        if b.is_ascii_alphanumeric()||b==b'-'||b==b'_'||b==b'.'||b==b' '{
            if b==b' '{'_'.to_string()}else{(b as char).to_string()}
        }else{
            format!("%{:02X}",b)
        }
    }).collect()
}

type UploadParsed = (String, Vec<u8>, Option<String>, Option<Uuid>, Option<String>, Option<String>);

async fn parse_upload(mut mp: Multipart) -> Result<UploadParsed, (StatusCode, Json<ErrorResponse>)> {
    let mut filename: Option<String> = None;
    let mut target_type: Option<String> = None;
    let mut target_id: Option<Uuid> = None;
    let mut remarks: Option<String> = None;
    let mut data: Option<Vec<u8>> = None;
    let mut field_ct: Option<String> = None;
    loop {
        let field = match mp.next_field().await {
            Ok(Some(f)) => f,
            Ok(None) => break,
            Err(e) => return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error: format!("multipart error: {}", e) }))),
        };
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "file" => {
                filename = field.file_name().map(|s| s.to_string());
                field_ct = field.content_type().map(|s| s.to_string());
                let bytes = match field.bytes().await {
                    Ok(b) => b,
                    Err(e) => return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error: format!("read file error: {}", e) }))),
                };
                data = Some(bytes.to_vec());
            }
            "target_type" => target_type = field.text().await.ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
            "target_id" => { if let Ok(s) = field.text().await { target_id = Uuid::parse_str(s.trim()).ok(); } }
            "remarks" => remarks = field.text().await.ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
            _ => {}
        }
    }
    let fname = filename.unwrap_or_else(|| "unnamed".to_string());
    let bytes = match data {
        Some(d) => d,
        None => return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "no file part provided".into() }))),
    };
    if bytes.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "empty file".into() })));
    }
    if bytes.len() > 50 * 1024 * 1024 {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "file too large (max 50MB)".into() })));
    }
    Ok((fname, bytes, target_type, target_id, remarks, field_ct))
}

/// 上传附件（multipart: file + target_type + target_id + remarks）
#[utoipa::path(post, path = "/api/v1/attachments/upload", responses((status = 201, body = AttachmentResponse), (status = 400, body = ErrorResponse)), tag = "Attachment", security(("bearer_auth" = [])))]
pub async fn upload_attachment(auth: AuthUser, State(r): State<Arc<dyn AttachmentRepository>>, Extension(upload_dir): Extension<String>, Extension(change_log): Extension<ChangeLogState>, mp: Multipart) -> impl IntoResponse {
    let parsed = match parse_upload(mp).await {
        Ok(p) => p,
        Err((status, err)) => return (status, err).into_response(),
    };
    let (fname, bytes, target_type, target_id, remarks, field_ct) = parsed;
    let key = Uuid::new_v4().to_string();
    let path = format!("{}/{}", upload_dir, key);
    if let Err(e) = tokio::fs::create_dir_all(&upload_dir).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("create upload dir error: {}", e) })).into_response();
    }
    if let Err(e) = tokio::fs::write(&path, &bytes).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("save file error: {}", e) })).into_response();
    }
    let now = Utc::now();
    let ct = content_type_from_name(&fname, field_ct.as_deref());
    let e = Attachment { id: Uuid::new_v4(), filename: fname, target_type, target_id, storage_key: Some(key), content_type: ct, size_bytes: Some(bytes.len() as i64), uploaded_by_id: Some(auth.user_id), status: "active".to_string(), remarks, created_at: now, updated_at: now };
    match r.create(&e).await {
        Ok(c) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Create, "attachment", Some(c.id), serde_json::json!(null), serde_json::to_value(to_resp(&c)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::CREATED, Json(to_resp(&c))).into_response()
        }
        Err(err) => {
            let _ = tokio::fs::remove_file(&path).await;
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: err.to_string() })).into_response()
        }
    }
}

#[utoipa::path(get, path = "/api/v1/attachments/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = AttachmentResponse), (status = 404, body = ErrorResponse)), tag = "Attachment", security(("bearer_auth" = [])))]
pub async fn get_attachment(_auth:AuthUser,State(r):State<Arc<dyn AttachmentRepository>>,Path(id):Path<Uuid>)->impl IntoResponse{match r.find_by_id(id).await{Ok(Some(e))=>(StatusCode::OK,Json(to_resp(&e))).into_response(),Ok(None)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}}
#[utoipa::path(get, path = "/api/v1/attachments", params(ListAttachmentsQuery), responses((status = 200, body = AttachmentListResponse)), tag = "Attachment", security(("bearer_auth" = [])))]
pub async fn list_attachments(_auth:AuthUser,State(r):State<Arc<dyn AttachmentRepository>>,Query(q):Query<ListAttachmentsQuery>)->impl IntoResponse{let p=q.page.unwrap_or(1).max(1);let pp=q.per_page.unwrap_or(20).min(100);let f=AttachmentFilter{status:q.status,target_type:q.target_type,target_id:q.target_id,q:q.q,limit:Some(pp),offset:Some((p-1)*pp)};match r.find_all(f.clone()).await{Ok(v)=>{let c=r.count(f).await.unwrap_or(0);(StatusCode::OK,Json(AttachmentListResponse{data:v.iter().map(to_resp).collect(),count:c})).into_response()},Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()}}
#[utoipa::path(post, path = "/api/v1/attachments", request_body = CreateAttachmentRequest, responses((status = 201, body = AttachmentResponse), (status = 400, body = ErrorResponse)), tag = "Attachment", security(("bearer_auth" = [])))]
pub async fn create_attachment(auth:AuthUser,State(r):State<Arc<dyn AttachmentRepository>>,Extension(change_log):Extension<ChangeLogState>,Json(b):Json<CreateAttachmentRequest>)->impl IntoResponse{
    let now=Utc::now();let e=Attachment{id:Uuid::new_v4(),filename:b.filename,target_type:b.target_type,target_id:b.target_id,storage_key:b.storage_key,content_type:b.content_type,size_bytes:b.size_bytes,uploaded_by_id:b.uploaded_by_id.or(Some(auth.user_id)),status:b.status.unwrap_or_else(|| "active".to_string()),remarks:b.remarks,created_at:now,updated_at:now};
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
    let e=Attachment{id:ex.id,filename:b.filename,target_type:b.target_type.or(ex.target_type),target_id:b.target_id.or(ex.target_id),storage_key:b.storage_key.or(ex.storage_key),content_type:b.content_type.or(ex.content_type),size_bytes:b.size_bytes.or(ex.size_bytes),uploaded_by_id:b.uploaded_by_id.or(ex.uploaded_by_id),status:b.status.unwrap_or(ex.status),remarks:b.remarks.or(ex.remarks),created_at:ex.created_at,updated_at:Utc::now()};
    match r.update(id,&e).await{
        Ok(Some(c))=>{
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Update,"attachment",Some(c.id),before_value,serde_json::to_value(to_resp(&c)).unwrap_or(serde_json::json!({})),None).await;
            (StatusCode::OK,Json(to_resp(&c))).into_response()
        }
        Ok(None)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()
    }
}
#[utoipa::path(delete, path = "/api/v1/attachments/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 204), (status = 404, body = ErrorResponse)), tag = "Attachment", security(("bearer_auth" = [])))]
pub async fn delete_attachment(auth:AuthUser,State(r):State<Arc<dyn AttachmentRepository>>,Extension(upload_dir):Extension<String>,Extension(change_log):Extension<ChangeLogState>,Path(id):Path<Uuid>)->impl IntoResponse{
    let ex=match r.find_by_id(id).await{Ok(Some(e))=>e,Ok(None)=>return(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>return(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()};
    match r.delete(id).await{
        Ok(true)=>{
            if let Some(key)=ex.storage_key{
                let _=tokio::fs::remove_file(format!("{}/{}",upload_dir,key)).await;
            }
            record_change(&change_log,&auth,rzops_domain::enums::ChangeType::Delete,"attachment",Some(id),serde_json::json!({}),serde_json::json!(null),None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false)=>(StatusCode::NOT_FOUND,Json(ErrorResponse{error:"not found".into()})).into_response(),Err(e)=>(StatusCode::INTERNAL_SERVER_ERROR,Json(ErrorResponse{error:e.to_string()})).into_response()
    }
}
