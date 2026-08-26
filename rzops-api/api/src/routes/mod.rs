pub mod auth_handlers;
pub mod provider_handlers;
pub mod datacenter_handlers;
pub mod server_handlers;
pub mod server_ip_handlers;
pub mod server_port_handlers;
pub mod domain_handlers;
pub mod certificate_handlers;
pub mod database_instance_handlers;
pub mod ops_site_handlers;
pub mod credential_handlers;
pub mod backup_plan_handlers;
pub mod monitor_target_handlers;
pub mod contract_handlers;
pub mod attachment_handlers;
pub mod audit_log_handlers;
pub mod change_record_handlers;
pub mod site_relation_handlers;

use std::sync::Arc;
use axum::Router;
use rzops_domain::ports::*;

use auth_handlers::*;
use provider_handlers::*;
use datacenter_handlers::*;
use server_handlers::*;
use server_ip_handlers::*;
use server_port_handlers::*;
use domain_handlers::*;
use certificate_handlers::*;
use database_instance_handlers::*;
use ops_site_handlers::*;
use credential_handlers::*;
use backup_plan_handlers::*;
use monitor_target_handlers::*;
use contract_handlers::*;
use attachment_handlers::*;
use audit_log_handlers::*;
use change_record_handlers::*;
use site_relation_handlers::*;

pub fn auth_routes(state: auth_handlers::AuthState) -> Router {
    Router::new()
        .route("/login", axum::routing::post(login))
        .route("/register", axum::routing::post(register))
        .route("/me", axum::routing::get(me))
        .with_state(state)
}

pub fn provider_routes(repo: Arc<dyn provider_repository::ProviderRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_providers).post(create_provider))
        .route("/{id}", axum::routing::get(get_provider).put(update_provider).delete(delete_provider)).with_state(repo)
}
pub fn datacenter_routes(repo: Arc<dyn datacenter_repository::DataCenterRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_data_centers).post(create_data_center))
        .route("/{id}", axum::routing::get(get_data_center).put(update_data_center).delete(delete_data_center)).with_state(repo)
}
pub fn server_routes(repo: Arc<dyn server_repository::ServerRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_servers).post(create_server))
        .route("/{id}", axum::routing::get(get_server).put(update_server).delete(delete_server)).with_state(repo)
}
pub fn server_ip_routes(repo: Arc<dyn server_ip_repository::ServerIpRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_server_ips).post(create_server_ip))
        .route("/{id}", axum::routing::get(get_server_ip).put(update_server_ip).delete(delete_server_ip)).with_state(repo)
}
pub fn server_port_routes(repo: Arc<dyn server_port_repository::ServerPortRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_server_ports).post(create_server_port))
        .route("/{id}", axum::routing::get(get_server_port).put(update_server_port).delete(delete_server_port)).with_state(repo)
}
pub fn domain_routes(repo: Arc<dyn domain_repository::DomainRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_domains).post(create_domain))
        .route("/{id}", axum::routing::get(get_domain).put(update_domain).delete(delete_domain)).with_state(repo)
}
pub fn certificate_routes(repo: Arc<dyn certificate_repository::CertificateRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_certificates).post(create_certificate))
        .route("/{id}", axum::routing::get(get_certificate).put(update_certificate).delete(delete_certificate)).with_state(repo)
}
pub fn database_instance_routes(repo: Arc<dyn database_instance_repository::DatabaseInstanceRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_database_instances).post(create_database_instance))
        .route("/{id}", axum::routing::get(get_database_instance).put(update_database_instance).delete(delete_database_instance)).with_state(repo)
}
pub fn ops_site_routes(repo: Arc<dyn ops_site_repository::OpsSiteRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_ops_sites).post(create_ops_site))
        .route("/{id}", axum::routing::get(get_ops_site).put(update_ops_site).delete(delete_ops_site)).with_state(repo)
}
pub fn credential_routes(repo: Arc<dyn credential_repository::CredentialRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_credentials).post(create_credential))
        .route("/{id}", axum::routing::get(get_credential).put(update_credential).delete(delete_credential)).with_state(repo)
}
pub fn backup_plan_routes(repo: Arc<dyn backup_plan_repository::BackupPlanRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_backup_plans).post(create_backup_plan))
        .route("/{id}", axum::routing::get(get_backup_plan).put(update_backup_plan).delete(delete_backup_plan)).with_state(repo)
}
pub fn monitor_target_routes(repo: Arc<dyn monitor_target_repository::MonitorTargetRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_monitor_targets).post(create_monitor_target))
        .route("/{id}", axum::routing::get(get_monitor_target).put(update_monitor_target).delete(delete_monitor_target)).with_state(repo)
}
pub fn contract_routes(repo: Arc<dyn contract_repository::ContractRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_contracts).post(create_contract))
        .route("/{id}", axum::routing::get(get_contract).put(update_contract).delete(delete_contract)).with_state(repo)
}
pub fn attachment_routes(repo: Arc<dyn attachment_repository::AttachmentRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_attachments).post(create_attachment))
        .route("/{id}", axum::routing::get(get_attachment).put(update_attachment).delete(delete_attachment)).with_state(repo)
}
pub fn audit_log_routes(repo: Arc<dyn audit_log_repository::AuditLogRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_audit_logs))
        .route("/{id}", axum::routing::get(get_audit_log)).with_state(repo)
}
pub fn change_record_routes(repo: Arc<dyn change_record_repository::ChangeRecordRepository>) -> Router {
    Router::new().route("/", axum::routing::get(list_change_records))
        .route("/{id}", axum::routing::get(get_change_record)).with_state(repo)
}
/// Shared state for site relation routes.
#[derive(Clone)]
pub struct SiteRelationState {
    pub site_server: Arc<dyn site_relation_repository::SiteServerRelationRepository>,
    pub site_database: Arc<dyn site_relation_repository::SiteDatabaseRelationRepository>,
    pub site_domain: Arc<dyn site_relation_repository::SiteDomainRelationRepository>,
}

pub fn site_relation_routes(state: SiteRelationState) -> Router {
    Router::new()
        .route("/site-servers/{site_id}", axum::routing::get(list_site_servers))
        .route("/site-servers", axum::routing::post(create_site_server))
        .route("/site-servers/by-id/{id}", axum::routing::delete(delete_site_server))
        .route("/site-databases/{site_id}", axum::routing::get(list_site_databases))
        .route("/site-databases", axum::routing::post(create_site_database))
        .route("/site-databases/by-id/{id}", axum::routing::delete(delete_site_database))
        .route("/site-domains/{site_id}", axum::routing::get(list_site_domains))
        .route("/site-domains", axum::routing::post(create_site_domain))
        .route("/site-domains/by-id/{id}", axum::routing::delete(delete_site_domain))
        .with_state(state)
}
