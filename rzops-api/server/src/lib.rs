use std::sync::Arc;
use axum::Router;
use sqlx::{Pool, Postgres};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use rzops_domain::ports::*;
use rzops_infra::db::repositories::*;
use rzops_infra::auth::JwtService;

mod middleware;

/// Application state shared across handlers.
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub user_repo: Arc<dyn user_repository::UserRepository>,
    pub provider_repo: Arc<dyn provider_repository::ProviderRepository>,
    pub datacenter_repo: Arc<dyn datacenter_repository::DataCenterRepository>,
    pub server_repo: Arc<dyn server_repository::ServerRepository>,
    pub server_ip_repo: Arc<dyn server_ip_repository::ServerIpRepository>,
    pub server_port_repo: Arc<dyn server_port_repository::ServerPortRepository>,
    pub certificate_domain_repo: Arc<dyn certificate_domain_repository::CertificateDomainRepository>,
    pub domain_repo: Arc<dyn domain_repository::DomainRepository>,
    pub certificate_repo: Arc<dyn certificate_repository::CertificateRepository>,
    pub database_instance_repo: Arc<dyn database_instance_repository::DatabaseInstanceRepository>,
    pub ops_site_repo: Arc<dyn ops_site_repository::OpsSiteRepository>,
    pub backup_plan_repo: Arc<dyn backup_plan_repository::BackupPlanRepository>,
    pub monitor_target_repo: Arc<dyn monitor_target_repository::MonitorTargetRepository>,
    pub contract_repo: Arc<dyn contract_repository::ContractRepository>,
    pub attachment_repo: Arc<dyn attachment_repository::AttachmentRepository>,
    pub audit_log_repo: Arc<dyn audit_log_repository::AuditLogRepository>,
    pub change_record_repo: Arc<dyn change_record_repository::ChangeRecordRepository>,
    pub site_server_relation_repo: Arc<dyn site_relation_repository::SiteServerRelationRepository>,
    pub site_database_relation_repo: Arc<dyn site_relation_repository::SiteDatabaseRelationRepository>,
    pub site_domain_relation_repo: Arc<dyn site_relation_repository::SiteDomainRelationRepository>,
    pub dict_repo: Arc<dyn dict_repository::DictRepository>,
    pub token_service: Arc<dyn token_service::TokenService>,
    pub upload_dir: String,
}

impl AppState {
    pub fn new(pool: Pool<Postgres>, jwt_secret: &[u8], jwt_expiration: u64) -> Self {
        let site_rel = Arc::new(PgSiteRelationRepository::new(pool.clone()));
        let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "uploads".to_string());
        Self {
            user_repo: Arc::new(PgUserRepository::new(pool.clone())),
            provider_repo: Arc::new(PgProviderRepository::new(pool.clone())),
            datacenter_repo: Arc::new(PgDataCenterRepository::new(pool.clone())),
            server_repo: Arc::new(PgServerRepository::new(pool.clone())),
            server_ip_repo: Arc::new(PgServerIpRepository::new(pool.clone())),
            server_port_repo: Arc::new(PgServerPortRepository::new(pool.clone())),
            certificate_domain_repo: Arc::new(PgCertificateDomainRepository::new(pool.clone())),
            domain_repo: Arc::new(PgDomainRepository::new(pool.clone())),
            certificate_repo: Arc::new(PgCertificateRepository::new(pool.clone())),
            database_instance_repo: Arc::new(PgDatabaseInstanceRepository::new(pool.clone())),
            ops_site_repo: Arc::new(PgOpsSiteRepository::new(pool.clone())),
            backup_plan_repo: Arc::new(PgBackupPlanRepository::new(pool.clone())),
            monitor_target_repo: Arc::new(PgMonitorTargetRepository::new(pool.clone())),
            contract_repo: Arc::new(PgContractRepository::new(pool.clone())),
            attachment_repo: Arc::new(PgAttachmentRepository::new(pool.clone())),
            audit_log_repo: Arc::new(PgAuditLogRepository::new(pool.clone())),
            change_record_repo: Arc::new(PgChangeRecordRepository::new(pool.clone())),
            site_server_relation_repo: site_rel.clone(),
            site_database_relation_repo: site_rel.clone(),
            site_domain_relation_repo: site_rel,
            dict_repo: Arc::new(PgDictRepository::new(pool.clone())),
            token_service: Arc::new(JwtService::new(jwt_secret, jwt_expiration)),
            upload_dir,
            pool,
        }
    }
}

/// Build CORS layer from environment or defaults.
fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

/// Create the main application router with middleware.
pub fn create_router(state: AppState) -> Router {
    let auth_state = rzops_api::auth_handlers::AuthState {
        user_repo: state.user_repo.clone(),
        token_service: state.token_service.clone(),
    };

    let change_log = rzops_api::ChangeLogState::new(
        state.change_record_repo.clone(),
        state.user_repo.clone(),
    );

    let audit_state = middleware::audit::AuditState::new(
        state.audit_log_repo.clone(),
        state.token_service.clone(),
    );

    let api_routes = Router::new()
        .nest("/api/v1/auth", rzops_api::auth_routes(auth_state))
        .nest("/api/v1/providers", rzops_api::provider_routes(state.provider_repo.clone()))
        .nest("/api/v1/data-centers", rzops_api::datacenter_routes(state.datacenter_repo.clone()))
        .nest("/api/v1/servers", rzops_api::server_routes(state.server_repo.clone()))
        .nest("/api/v1/server-ips", rzops_api::server_ip_routes(state.server_ip_repo.clone(), state.pool.clone()))
        .nest("/api/v1/server-ports", rzops_api::server_port_routes(state.server_port_repo.clone()))
        .nest("/api/v1/certificate-domains", rzops_api::certificate_domain_routes(state.certificate_domain_repo.clone()))
        .nest("/api/v1/domains", rzops_api::domain_routes(state.domain_repo.clone()))
        .nest("/api/v1/certificates", rzops_api::certificate_routes(state.certificate_repo.clone()))
        .nest("/api/v1/database-instances", rzops_api::database_instance_routes(state.database_instance_repo.clone(), state.pool.clone()))
        .nest("/api/v1/ops-sites", rzops_api::ops_site_routes(state.ops_site_repo.clone()))
        .nest("/api/v1/backup-plans", rzops_api::backup_plan_routes(state.backup_plan_repo.clone(), state.pool.clone()))
        .nest("/api/v1/monitor-targets", rzops_api::monitor_target_routes(state.monitor_target_repo.clone(), state.pool.clone()))
        .nest("/api/v1/contracts", rzops_api::contract_routes(state.contract_repo.clone()))
        .nest("/api/v1/attachments", rzops_api::attachment_routes(state.attachment_repo.clone(), state.pool.clone(), state.upload_dir.clone()))
        .nest("/api/v1/audit-logs", rzops_api::audit_log_routes(state.audit_log_repo.clone(), state.pool.clone()))
        .nest("/api/v1/change-records", rzops_api::change_record_routes(state.change_record_repo.clone(), state.pool.clone()))
        .nest("/api/v1/dicts", rzops_api::dict_routes(state.dict_repo.clone()))
        .nest("/api/v1/site-relations", rzops_api::site_relation_routes(rzops_api::SiteRelationState {
            site_server: state.site_server_relation_repo.clone(),
            site_database: state.site_database_relation_repo.clone(),
            site_domain: state.site_domain_relation_repo.clone(),
        }));

    let openapi = rzops_api::ApiDoc::openapi();

    api_routes
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .layer(axum::middleware::from_fn_with_state(
            audit_state,
            middleware::audit::audit_log_middleware,
        ))
        .layer(axum::Extension(change_log))
        .layer(axum::Extension(state.token_service.clone() as Arc<dyn token_service::TokenService>))
        .layer(axum::Extension(state.user_repo.clone() as Arc<dyn user_repository::UserRepository>))
        .layer(cors_layer())
        .layer(TraceLayer::new_for_http())
}

/// Run the server.
pub async fn run(state: AppState, host: &str, port: u16) -> Result<(), anyhow::Error> {
    let app = create_router(state);
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
