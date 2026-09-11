use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use rzops_domain::models::certificate_domain::CertificateDomain;
use rzops_domain::ports::certificate_domain_repository::{
    CertificateDomainFilter, CertificateDomainRepository,
};

use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::certificate_domain_dto::*;

fn to_response(cd: &CertificateDomain) -> CertificateDomainResponse {
    CertificateDomainResponse {
        id: cd.id,
        certificate_id: cd.certificate_id,
        domain_id: cd.domain_id,
        domain_pattern: cd.domain_pattern.clone(),
        is_primary: cd.is_primary,
        created_at: cd.created_at,
    }
}

/// GET /certificate-domains/:id
#[utoipa::path(get, path = "/api/v1/certificate-domains/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = CertificateDomainResponse), (status = 404, body = ErrorResponse)), tag = "CertificateDomain", security(("bearer_auth" = [])))]
pub async fn get_certificate_domain(
    _auth: AuthUser,
    State(repo): State<Arc<dyn CertificateDomainRepository>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.find_by_id(id).await {
        Ok(Some(cd)) => (StatusCode::OK, Json(to_response(&cd))).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "certificate domain binding not found".to_string() }),
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("database error: {}", e) }),
        ).into_response(),
    }
}

/// GET /certificate-domains
#[utoipa::path(get, path = "/api/v1/certificate-domains", params(ListCertificateDomainsQuery), responses((status = 200, body = CertificateDomainListResponse)), tag = "CertificateDomain", security(("bearer_auth" = [])))]
pub async fn list_certificate_domains(
    _auth: AuthUser,
    State(repo): State<Arc<dyn CertificateDomainRepository>>,
    Query(query): Query<ListCertificateDomainsQuery>,
) -> impl IntoResponse {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(100).min(200);
    let offset = (page - 1) * per_page;

    let filter = CertificateDomainFilter {
        certificate_id: query.certificate_id,
        domain_id: query.domain_id,
        q: query.q,
        limit: Some(per_page),
        offset: Some(offset),
    };

    match repo.find_all(filter.clone()).await {
        Ok(list) => {
            let count = repo.count(filter).await.unwrap_or(0);
            let data = list.iter().map(to_response).collect();
            (StatusCode::OK, Json(CertificateDomainListResponse { data, count })).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("database error: {}", e) }),
        ).into_response(),
    }
}

/// POST /certificate-domains
#[utoipa::path(post, path = "/api/v1/certificate-domains", request_body = CreateCertificateDomainRequest, responses((status = 201, body = CertificateDomainResponse), (status = 400, body = ErrorResponse)), tag = "CertificateDomain", security(("bearer_auth" = [])))]
pub async fn create_certificate_domain(
    auth: AuthUser,
    State(repo): State<Arc<dyn CertificateDomainRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Json(body): Json<CreateCertificateDomainRequest>,
) -> impl IntoResponse {
    let now = Utc::now();
    let cd = CertificateDomain {
        id: Uuid::new_v4(),
        certificate_id: body.certificate_id,
        domain_id: body.domain_id,
        domain_pattern: body.domain_pattern,
        is_primary: body.is_primary.unwrap_or(false),
        created_at: now,
    };

    match repo.create(&cd).await {
        Ok(created) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Create, "certificate_domain", Some(created.id), serde_json::json!(null), serde_json::to_value(to_response(&created)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::CREATED, Json(to_response(&created))).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: format!("failed to create certificate domain binding: {}", e) }),
        ).into_response(),
    }
}

/// PUT /certificate-domains/:id
#[utoipa::path(put, path = "/api/v1/certificate-domains/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateCertificateDomainRequest, responses((status = 200, body = CertificateDomainResponse), (status = 404, body = ErrorResponse)), tag = "CertificateDomain", security(("bearer_auth" = [])))]
pub async fn update_certificate_domain(
    auth: AuthUser,
    State(repo): State<Arc<dyn CertificateDomainRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateCertificateDomainRequest>,
) -> impl IntoResponse {
    let existing = match repo.find_by_id(id).await {
        Ok(Some(cd)) => cd,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "certificate domain binding not found".to_string() })).into_response()
        }
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response()
        }
    };

    let before_value = serde_json::to_value(to_response(&existing)).unwrap_or(serde_json::json!({}));
    let cd = CertificateDomain {
        id: existing.id,
        certificate_id: existing.certificate_id,
        domain_id: body.domain_id.or(existing.domain_id),
        domain_pattern: body.domain_pattern.unwrap_or(existing.domain_pattern),
        is_primary: body.is_primary.unwrap_or(existing.is_primary),
        created_at: existing.created_at,
    };

    match repo.update(id, &cd).await {
        Ok(Some(updated)) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Update, "certificate_domain", Some(updated.id), before_value, serde_json::to_value(to_response(&updated)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::OK, Json(to_response(&updated))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "certificate domain binding not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to update certificate domain binding: {}", e) })).into_response(),
    }
}

/// DELETE /certificate-domains/:id
#[utoipa::path(delete, path = "/api/v1/certificate-domains/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "CertificateDomain", security(("bearer_auth" = [])))]
pub async fn delete_certificate_domain(
    auth: AuthUser,
    State(repo): State<Arc<dyn CertificateDomainRepository>>,
    Extension(change_log): Extension<ChangeLogState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(true) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Delete, "certificate_domain", Some(id), serde_json::json!({}), serde_json::json!(null), None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "certificate domain binding not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to delete certificate domain binding: {}", e) })).into_response(),
    }
}
