use std::sync::Arc;
use axum::{extract::{Extension, Path, Query, State}, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;
use rzops_domain::enums::{CodeRepoType, Importance, ServiceTarget, SiteStatus, WebFramework};
use rzops_domain::models::ops_site::OpsSite;
use rzops_domain::ports::ops_site_repository::{OpsSiteFilter, OpsSiteRepository};
use crate::dto::provider_dto::ErrorResponse;
use crate::dto::ops_site_dto::*;
use crate::auth_extractor::AuthUser;
use crate::change_log::{record_change, ChangeLogState};

fn parse_site_status(s: &str) -> SiteStatus { match s { "active" => SiteStatus::Active, "temporary_offline" => SiteStatus::TemporaryOffline, "permanent_offline" => SiteStatus::PermanentOffline, _ => SiteStatus::Active } }
fn site_status_to_string(s: &SiteStatus) -> String { match s { SiteStatus::Active => "active", SiteStatus::TemporaryOffline => "temporary_offline", SiteStatus::PermanentOffline => "permanent_offline" }.to_string() }
fn parse_importance(s: &str) -> Importance { match s { "critical" => Importance::Critical, "high" => Importance::High, "medium" => Importance::Medium, "low" => Importance::Low, _ => Importance::Medium } }
fn importance_to_string(i: &Importance) -> String { match i { Importance::Critical => "critical", Importance::High => "high", Importance::Medium => "medium", Importance::Low => "low" }.to_string() }
fn parse_service_target(s: &str) -> ServiceTarget { match s { "internal" => ServiceTarget::Internal, "external" => ServiceTarget::External, "partner" => ServiceTarget::Partner, "mixed" => ServiceTarget::Mixed, _ => ServiceTarget::Internal } }
fn service_target_to_string(t: &ServiceTarget) -> String { match t { ServiceTarget::Internal => "internal", ServiceTarget::External => "external", ServiceTarget::Partner => "partner", ServiceTarget::Mixed => "mixed" }.to_string() }
fn parse_code_repo_type(s: &str) -> CodeRepoType { match s { "svn" => CodeRepoType::Svn, "git" => CodeRepoType::Git, "none" => CodeRepoType::None, _ => CodeRepoType::Other } }
fn code_repo_type_to_string(t: &CodeRepoType) -> String { match t { CodeRepoType::Svn => "svn", CodeRepoType::Git => "git", CodeRepoType::None => "none", CodeRepoType::Other => "other" }.to_string() }
fn parse_web_framework(s: &str) -> WebFramework { match s { "django" => WebFramework::Django, "flask" => WebFramework::Flask, "fastapi" => WebFramework::Fastapi, "spring_boot" => WebFramework::SpringBoot, "express" => WebFramework::Express, "rails" => WebFramework::Rails, "laravel" => WebFramework::Laravel, "asp_net_mvc" => WebFramework::AspNetMvc, "asp_net_core" => WebFramework::AspNetCore, "gin" => WebFramework::Gin, "echo" => WebFramework::Echo, "nextjs" => WebFramework::Nextjs, "nuxtjs" => WebFramework::Nuxtjs, "ant_design_pro" => WebFramework::AntDesignPro, _ => WebFramework::Other } }
fn web_framework_to_string(f: &WebFramework) -> String { match f { WebFramework::Django => "django", WebFramework::Flask => "flask", WebFramework::Fastapi => "fastapi", WebFramework::SpringBoot => "spring_boot", WebFramework::Express => "express", WebFramework::Rails => "rails", WebFramework::Laravel => "laravel", WebFramework::AspNetMvc => "asp_net_mvc", WebFramework::AspNetCore => "asp_net_core", WebFramework::Gin => "gin", WebFramework::Echo => "echo", WebFramework::Nextjs => "nextjs", WebFramework::Nuxtjs => "nuxtjs", WebFramework::AntDesignPro => "ant_design_pro", WebFramework::Other => "other" }.to_string() }

fn to_response(s: &OpsSite) -> OpsSiteResponse {
    OpsSiteResponse { id: s.id, name: s.name.clone(), url: s.url.clone(), business_unit_id: s.business_unit_id, department_id: s.department_id, service_target: s.service_target.as_ref().map(service_target_to_string), importance: s.importance.as_ref().map(importance_to_string), online_time: s.online_time, code_repo_type: s.code_repo_type.as_ref().map(code_repo_type_to_string), code_repo_url: s.code_repo_url.clone(), purpose: s.purpose.clone(), is_internal_system: s.is_internal_system, language_runtime: s.language_runtime.clone(), web_framework: s.web_framework.as_ref().map(web_framework_to_string), uses_cdn: s.uses_cdn, is_test_site: s.is_test_site, backup_plan_id: s.backup_plan_id, last_backup_time: s.last_backup_time, monitor_target_id: s.monitor_target_id, status: site_status_to_string(&s.status), offline_time: s.offline_time, offline_reason: s.offline_reason.clone(), function_summary: s.function_summary.clone(), remarks: s.remarks.clone(), created_at: s.created_at, updated_at: s.updated_at }
}

#[utoipa::path(get, path = "/api/v1/ops-sites/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200, body = OpsSiteResponse), (status = 404, body = ErrorResponse)), tag = "OpsSite", security(("bearer_auth" = [])))]
pub async fn get_ops_site(_auth: AuthUser, State(repo): State<Arc<dyn OpsSiteRepository>>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repo.find_by_id(id).await { Ok(Some(s)) => (StatusCode::OK, Json(to_response(&s))).into_response(), Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "ops site not found".to_string() })).into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response() }
}
#[utoipa::path(get, path = "/api/v1/ops-sites", params(ListOpsSitesQuery), responses((status = 200, body = OpsSiteListResponse)), tag = "OpsSite", security(("bearer_auth" = [])))]
pub async fn list_ops_sites(_auth: AuthUser, State(repo): State<Arc<dyn OpsSiteRepository>>, Query(q): Query<ListOpsSitesQuery>) -> impl IntoResponse {
    let page = q.page.unwrap_or(1).max(1); let per_page = q.per_page.unwrap_or(20).min(100);
    let filter = OpsSiteFilter { status: q.status, importance: q.importance, q: q.q, limit: Some(per_page), offset: Some((page - 1) * per_page) };
    match repo.find_all(filter.clone()).await { Ok(ss) => { let count = repo.count(filter).await.unwrap_or(0); (StatusCode::OK, Json(OpsSiteListResponse { data: ss.iter().map(to_response).collect(), count })).into_response() }, Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response() }
}
#[utoipa::path(post, path = "/api/v1/ops-sites", request_body = CreateOpsSiteRequest, responses((status = 201, body = OpsSiteResponse), (status = 400, body = ErrorResponse)), tag = "OpsSite", security(("bearer_auth" = [])))]
pub async fn create_ops_site(auth: AuthUser, State(repo): State<Arc<dyn OpsSiteRepository>>, Extension(change_log): Extension<ChangeLogState>, Json(body): Json<CreateOpsSiteRequest>) -> impl IntoResponse {
    let now = Utc::now();
    let s = OpsSite { id: Uuid::new_v4(), name: body.name, url: body.url, business_unit_id: body.business_unit_id, department_id: body.department_id, service_target: body.service_target.as_deref().map(parse_service_target), importance: body.importance.as_deref().map(parse_importance), online_time: body.online_time, code_repo_type: body.code_repo_type.as_deref().map(parse_code_repo_type), code_repo_url: body.code_repo_url, purpose: body.purpose, is_internal_system: body.is_internal_system.unwrap_or(false), language_runtime: body.language_runtime, web_framework: body.web_framework.as_deref().map(parse_web_framework), uses_cdn: body.uses_cdn, is_test_site: body.is_test_site.unwrap_or(false), backup_plan_id: body.backup_plan_id, last_backup_time: body.last_backup_time, monitor_target_id: body.monitor_target_id, status: body.status.as_deref().map(parse_site_status).unwrap_or(SiteStatus::Active), offline_time: body.offline_time, offline_reason: body.offline_reason, function_summary: body.function_summary, remarks: body.remarks, created_at: now, updated_at: now };
    match repo.create(&s).await {
        Ok(created) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Create, "ops_site", Some(created.id), serde_json::json!(null), serde_json::to_value(to_response(&created)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::CREATED, Json(to_response(&created))).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to create ops site: {}", e) })).into_response() }
}
#[utoipa::path(put, path = "/api/v1/ops-sites/{id}", params(("id" = uuid::Uuid, Path)), request_body = UpdateOpsSiteRequest, responses((status = 200, body = OpsSiteResponse), (status = 404, body = ErrorResponse)), tag = "OpsSite", security(("bearer_auth" = [])))]
pub async fn update_ops_site(auth: AuthUser, State(repo): State<Arc<dyn OpsSiteRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id): Path<Uuid>, Json(body): Json<UpdateOpsSiteRequest>) -> impl IntoResponse {
    let existing = match repo.find_by_id(id).await { Ok(Some(s)) => s, Ok(None) => return (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "ops site not found".to_string() })).into_response(), Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("database error: {}", e) })).into_response() };
    let before_value = serde_json::to_value(to_response(&existing)).unwrap_or(serde_json::json!({}));
    let s = OpsSite { id: existing.id, name: body.name.unwrap_or(existing.name), url: body.url.or(existing.url), business_unit_id: body.business_unit_id.or(existing.business_unit_id), department_id: body.department_id.or(existing.department_id), service_target: body.service_target.as_deref().map(parse_service_target).or(existing.service_target), importance: body.importance.as_deref().map(parse_importance).or(existing.importance), online_time: body.online_time.or(existing.online_time), code_repo_type: body.code_repo_type.as_deref().map(parse_code_repo_type).or(existing.code_repo_type), code_repo_url: body.code_repo_url.or(existing.code_repo_url), purpose: body.purpose.or(existing.purpose), is_internal_system: body.is_internal_system.unwrap_or(existing.is_internal_system), language_runtime: body.language_runtime.or(existing.language_runtime), web_framework: body.web_framework.as_deref().map(parse_web_framework).or(existing.web_framework), uses_cdn: body.uses_cdn.or(existing.uses_cdn), is_test_site: body.is_test_site.unwrap_or(existing.is_test_site), backup_plan_id: body.backup_plan_id.or(existing.backup_plan_id), last_backup_time: body.last_backup_time.or(existing.last_backup_time), monitor_target_id: body.monitor_target_id.or(existing.monitor_target_id), status: body.status.as_deref().map(parse_site_status).unwrap_or(existing.status), offline_time: body.offline_time.or(existing.offline_time), offline_reason: body.offline_reason.or(existing.offline_reason), function_summary: body.function_summary.or(existing.function_summary), remarks: body.remarks.or(existing.remarks), created_at: existing.created_at, updated_at: Utc::now() };
    match repo.update(id, &s).await {
        Ok(Some(updated)) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Update, "ops_site", Some(updated.id), before_value, serde_json::to_value(to_response(&updated)).unwrap_or(serde_json::json!({})), None).await;
            (StatusCode::OK, Json(to_response(&updated))).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "ops site not found".to_string() })).into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to update ops site: {}", e) })).into_response() }
}
#[utoipa::path(delete, path = "/api/v1/ops-sites/{id}", params(("id" = uuid::Uuid, Path)), responses((status = 200), (status = 404, body = ErrorResponse)), tag = "OpsSite", security(("bearer_auth" = [])))]
pub async fn delete_ops_site(auth: AuthUser, State(repo): State<Arc<dyn OpsSiteRepository>>, Extension(change_log): Extension<ChangeLogState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    match repo.delete(id).await {
        Ok(true) => {
            record_change(&change_log, &auth, rzops_domain::enums::ChangeType::Delete, "ops_site", Some(id), serde_json::json!({}), serde_json::json!(null), None).await;
            StatusCode::NO_CONTENT.into_response()
        }
        Ok(false) => (StatusCode::NOT_FOUND, Json(ErrorResponse { error: "ops site not found".to_string() })).into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: format!("failed to delete ops site: {}", e) })).into_response() }
}
