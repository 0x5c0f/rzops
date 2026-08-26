use std::sync::Arc;
use axum::{extract::{Extension, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;
use rzops_domain::models::domain_asset::DomainAsset;
use rzops_domain::ports::domain_repository::{DomainFilter, DomainRepository};
use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::domain_dto::*;

fn to_response(d: &DomainAsset) -> DomainResponse {
    DomainResponse { id: d.id, domain_name: d.domain_name.clone(), business_unit_id: d.business_unit_id, company_id: d.company_id, expiry_date: d.expiry_date, renewal_amount: d.renewal_amount.map(|v| v.to_string()), renewal_currency: d.renewal_currency.clone(), provider_id: d.provider_id, account_credential_id: d.account_credential_id, platform_phone: d.platform_phone.clone(), domain_email: d.domain_email.clone(), privacy_status: d.privacy_status.clone(), is_enabled: d.is_enabled, remarks: d.remarks.clone(), created_at: d.created_at, updated_at: d.updated_at }
}
fn build_from_create(b: CreateDomainRequest) -> DomainAsset {
    let now = Utc::now();
    DomainAsset { id: Uuid::new_v4(), domain_name: b.domain_name, business_unit_id: b.business_unit_id, company_id: b.company_id, expiry_date: b.expiry_date, renewal_amount: b.renewal_amount.and_then(|v| v.parse::<Decimal>().ok()), renewal_currency: b.renewal_currency.unwrap_or_else(|| "CNY".to_string()), provider_id: b.provider_id, account_credential_id: b.account_credential_id, platform_phone: b.platform_phone, domain_email: b.domain_email, privacy_status: b.privacy_status, is_enabled: b.is_enabled.unwrap_or(true), remarks: b.remarks, created_at: now, updated_at: now }
}

#[utoipa::path(get, path = "/api/v1/domains/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = DomainResponse), (status = 404, body = ErrorResponse)), tag = "Domain", security(("bearer_auth" = [])))]
pub async fn get_domain(_auth: AuthUser, State(repo): State<Arc<dyn DomainRepository>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repo.find_by_id(id).await {
        Ok(Some(d)) => (StatusCode::OK, Json(to_response(&d))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "domain not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}
#[utoipa::path(get, path = "/api/v1/domains", params(ListDomainsQuery), responses((status = 200, body = DomainListResponse)), tag = "Domain", security(("bearer_auth" = [])))]
pub async fn list_domains(_auth: AuthUser, State(repo): State<Arc<dyn DomainRepository>>, Query(q): Query<ListDomainsQuery>) -> impl IntoResponse {
    let page = q.page.unwrap_or(1).max(1); let per_page = q.per_page.unwrap_or(20).min(100);
    let filter = DomainFilter { is_enabled: q.is_enabled, q: q.q, limit: Some(per_page), offset: Some((page - 1) * per_page) };
    match repo.find_all(filter.clone()).await {
        Ok(ds) => { let count = repo.count(filter).await.unwrap_or(0); (StatusCode::OK, Json(DomainListResponse { data: ds.iter().map(to_response).collect(), count })).into_response() }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response(),
    }
}
#[utoipa::path(post, path = "/api/v1/domains", request_body = CreateDomainRequest, responses((status = 201, body = DomainResponse), (status = 400, body = ErrorResponse)), tag = "Domain", security(("bearer_auth" = [])))]
pub async fn create_domain(auth: AuthUser, State(repo): State<Arc<dyn DomainRepository>>, Extension(change_log): Extension<ChangeLogState>, Json(body): Json<CreateDomainRequest>) -> impl IntoResponse {
    let d = build_from_create(body);
    match repo.create(&d).await {
        Ok(created) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Create, "domain", Some(created.id), serde_json::json!(null), serde_json::to_value(to_response(&created)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::CREATED, Json(to_response(&created))).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to create domain: {}", e) })).into_response(),
    }
}
#[utoipa::path(put, path = "/api/v1/domains/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateDomainRequest, responses((status = 200, body = DomainResponse), (status = 404, body = ErrorResponse)), tag = "Domain", security(("bearer_auth" = [])))]
pub async fn update_domain(auth: AuthUser, State(repo): State<Arc<dyn DomainRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id): Path<Uuid>, Json(body): Json<UpdateDomainRequest>) -> impl IntoResponse {
    let existing = match repo.find_by_id(id).await { Ok(Some(d)) => d, Ok(None) => return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "domain not found".to_string() })).into_response(), Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response() };
    let before_value = serde_json::to_value(to_response(&existing)).unwrap_or(serde_json::json!({}));
    let d = DomainAsset { id: existing.id, domain_name: body.domain_name.unwrap_or(existing.domain_name), business_unit_id: body.business_unit_id.or(existing.business_unit_id), company_id: body.company_id.or(existing.company_id), expiry_date: body.expiry_date.or(existing.expiry_date), renewal_amount: body.renewal_amount.and_then(|v| v.parse::<Decimal>().ok()).or(existing.renewal_amount), renewal_currency: body.renewal_currency.unwrap_or(existing.renewal_currency), provider_id: body.provider_id.or(existing.provider_id), account_credential_id: body.account_credential_id.or(existing.account_credential_id), platform_phone: body.platform_phone.or(existing.platform_phone), domain_email: body.domain_email.or(existing.domain_email), privacy_status: body.privacy_status.or(existing.privacy_status), is_enabled: body.is_enabled.unwrap_or(existing.is_enabled), remarks: body.remarks.or(existing.remarks), created_at: existing.created_at, updated_at: Utc::now() };
    match repo.update(id, &d).await {
        Ok(Some(updated)) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Update, "domain", Some(updated.id), before_value, serde_json::to_value(to_response(&updated)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::OK, Json(to_response(&updated))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "domain not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to update domain: {}", e) })).into_response(),
    }
}
#[utoipa::path(delete, path = "/api/v1/domains/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "Domain", security(("bearer_auth" = [])))]
pub async fn delete_domain(auth: AuthUser, State(repo): State<Arc<dyn DomainRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(true) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Delete, "domain", Some(id), serde_json::json!({}), serde_json::json!(null), None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "domain not found".to_string() })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to delete domain: {}", e) })).into_response(),
    }
}
