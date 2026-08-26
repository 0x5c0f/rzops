use std::sync::Arc;
use axum::{extract::{Extension, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;
use rzops_domain::models::certificate::Certificate;
use rzops_domain::ports::certificate_repository::{CertificateFilter, CertificateRepository};
use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::certificate_dto::*;

fn to_response(c: &Certificate) -> CertificateResponse {
    CertificateResponse { id: c.id, name: c.name.clone(), provider_id: c.provider_id, lease_start_date: c.lease_start_date, lease_end_date: c.lease_end_date, certificate_type: c.certificate_type.clone(), status: c.status.clone(), private_key_credential_id: c.private_key_credential_id, remarks: c.remarks.clone(), created_at: c.created_at, updated_at: c.updated_at }
}

#[utoipa::path(get, path = "/api/v1/certificates/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = CertificateResponse), (status = 404, body = ErrorResponse)), tag = "Certificate", security(("bearer_auth" = [])))]
pub async fn get_certificate(_auth: AuthUser, State(repo): State<Arc<dyn CertificateRepository>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repo.find_by_id(id).await { Ok(Some(c)) => (StatusCode::OK, Json(to_response(&c))).into_response(), Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "certificate not found".to_string() })).into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response() }
}
#[utoipa::path(get, path = "/api/v1/certificates", params(ListCertificatesQuery), responses((status = 200, body = CertificateListResponse)), tag = "Certificate", security(("bearer_auth" = [])))]
pub async fn list_certificates(_auth: AuthUser, State(repo): State<Arc<dyn CertificateRepository>>, Query(q): Query<ListCertificatesQuery>) -> impl IntoResponse {
    let page = q.page.unwrap_or(1).max(1); let per_page = q.per_page.unwrap_or(20).min(100);
    let filter = CertificateFilter { status: q.status, q: q.q, limit: Some(per_page), offset: Some((page - 1) * per_page) };
    match repo.find_all(filter.clone()).await { Ok(cs) => { let count = repo.count(filter).await.unwrap_or(0); (StatusCode::OK, Json(CertificateListResponse { data: cs.iter().map(to_response).collect(), count })).into_response() }, Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response() }
}
#[utoipa::path(post, path = "/api/v1/certificates", request_body = CreateCertificateRequest, responses((status = 201, body = CertificateResponse), (status = 400, body = ErrorResponse)), tag = "Certificate", security(("bearer_auth" = [])))]
pub async fn create_certificate(auth: AuthUser, State(repo): State<Arc<dyn CertificateRepository>>, Extension(change_log): Extension<ChangeLogState>, Json(body): Json<CreateCertificateRequest>) -> impl IntoResponse {
    let now = Utc::now();
    let c = Certificate { id: Uuid::new_v4(), name: body.name, provider_id: body.provider_id, lease_start_date: body.lease_start_date, lease_end_date: body.lease_end_date, certificate_type: body.certificate_type, status: body.status.unwrap_or_else(|| "active".to_string()), private_key_credential_id: body.private_key_credential_id, remarks: body.remarks, created_at: now, updated_at: now };
    match repo.create(&c).await {
        Ok(created) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Create, "certificate", Some(created.id), serde_json::json!(null), serde_json::to_value(to_response(&created)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::CREATED, Json(to_response(&created))).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to create certificate: {}", e) })).into_response()
    }
}
#[utoipa::path(put, path = "/api/v1/certificates/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateCertificateRequest, responses((status = 200, body = CertificateResponse), (status = 404, body = ErrorResponse)), tag = "Certificate", security(("bearer_auth" = [])))]
pub async fn update_certificate(auth: AuthUser, State(repo): State<Arc<dyn CertificateRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id): Path<Uuid>, Json(body): Json<UpdateCertificateRequest>) -> impl IntoResponse {
    let existing = match repo.find_by_id(id).await { Ok(Some(c)) => c, Ok(None) => return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "certificate not found".to_string() })).into_response(), Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response() };
    let before_value = serde_json::to_value(to_response(&existing)).unwrap_or(serde_json::json!({}));
    let c = Certificate { id: existing.id, name: body.name.unwrap_or(existing.name), provider_id: body.provider_id.or(existing.provider_id), lease_start_date: body.lease_start_date.or(existing.lease_start_date), lease_end_date: body.lease_end_date.or(existing.lease_end_date), certificate_type: body.certificate_type.or(existing.certificate_type), status: body.status.unwrap_or(existing.status), private_key_credential_id: body.private_key_credential_id.or(existing.private_key_credential_id), remarks: body.remarks.or(existing.remarks), created_at: existing.created_at, updated_at: Utc::now() };
    match repo.update(id, &c).await {
        Ok(Some(updated)) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Update, "certificate", Some(updated.id), before_value, serde_json::to_value(to_response(&updated)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::OK, Json(to_response(&updated))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "certificate not found".to_string() })).into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to update certificate: {}", e) })).into_response()
    }
}
#[utoipa::path(delete, path = "/api/v1/certificates/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "Certificate", security(("bearer_auth" = [])))]
pub async fn delete_certificate(auth: AuthUser, State(repo): State<Arc<dyn CertificateRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(true) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Delete, "certificate", Some(id), serde_json::json!({}), serde_json::json!(null), None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "certificate not found".to_string() })).into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to delete certificate: {}", e) })).into_response()
    }
}
