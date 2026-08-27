use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "RzOps API",
        description = "CMDB / 运维资产管理平台 API",
        version = "0.1.0",
        contact(name = "RzOps Team"),
    ),
    paths(
        // Auth
        crate::routes::auth_handlers::login,
        crate::routes::auth_handlers::register,
        crate::routes::auth_handlers::me,
        // Provider
        crate::routes::provider_handlers::get_provider,
        crate::routes::provider_handlers::list_providers,
        crate::routes::provider_handlers::create_provider,
        crate::routes::provider_handlers::update_provider,
        crate::routes::provider_handlers::delete_provider,
        // DataCenter
        crate::routes::datacenter_handlers::get_data_center,
        crate::routes::datacenter_handlers::list_data_centers,
        crate::routes::datacenter_handlers::create_data_center,
        crate::routes::datacenter_handlers::update_data_center,
        crate::routes::datacenter_handlers::delete_data_center,
        // Server
        crate::routes::server_handlers::get_server,
        crate::routes::server_handlers::list_servers,
        crate::routes::server_handlers::create_server,
        crate::routes::server_handlers::update_server,
        crate::routes::server_handlers::delete_server,
        // ServerIP
        crate::routes::server_ip_handlers::get_server_ip,
        crate::routes::server_ip_handlers::list_server_ips,
        crate::routes::server_ip_handlers::create_server_ip,
        crate::routes::server_ip_handlers::update_server_ip,
        crate::routes::server_ip_handlers::delete_server_ip,
        // ServerPort
        crate::routes::server_port_handlers::get_server_port,
        crate::routes::server_port_handlers::list_server_ports,
        crate::routes::server_port_handlers::create_server_port,
        crate::routes::server_port_handlers::update_server_port,
        crate::routes::server_port_handlers::delete_server_port,
        // Domain
        crate::routes::domain_handlers::get_domain,
        crate::routes::domain_handlers::list_domains,
        crate::routes::domain_handlers::create_domain,
        crate::routes::domain_handlers::update_domain,
        crate::routes::domain_handlers::delete_domain,
        // Certificate
        crate::routes::certificate_handlers::get_certificate,
        crate::routes::certificate_handlers::list_certificates,
        crate::routes::certificate_handlers::create_certificate,
        crate::routes::certificate_handlers::update_certificate,
        crate::routes::certificate_handlers::delete_certificate,
        // DatabaseInstance
        crate::routes::database_instance_handlers::get_database_instance,
        crate::routes::database_instance_handlers::list_database_instances,
        crate::routes::database_instance_handlers::create_database_instance,
        crate::routes::database_instance_handlers::update_database_instance,
        crate::routes::database_instance_handlers::delete_database_instance,
        // OpsSite
        crate::routes::ops_site_handlers::get_ops_site,
        crate::routes::ops_site_handlers::list_ops_sites,
        crate::routes::ops_site_handlers::create_ops_site,
        crate::routes::ops_site_handlers::update_ops_site,
        crate::routes::ops_site_handlers::delete_ops_site,
        // BackupPlan
        crate::routes::backup_plan_handlers::get_backup_plan,
        crate::routes::backup_plan_handlers::list_backup_plans,
        crate::routes::backup_plan_handlers::create_backup_plan,
        crate::routes::backup_plan_handlers::update_backup_plan,
        crate::routes::backup_plan_handlers::delete_backup_plan,
        // MonitorTarget
        crate::routes::monitor_target_handlers::get_monitor_target,
        crate::routes::monitor_target_handlers::list_monitor_targets,
        crate::routes::monitor_target_handlers::create_monitor_target,
        crate::routes::monitor_target_handlers::update_monitor_target,
        crate::routes::monitor_target_handlers::delete_monitor_target,
        // Contract
        crate::routes::contract_handlers::get_contract,
        crate::routes::contract_handlers::list_contracts,
        crate::routes::contract_handlers::create_contract,
        crate::routes::contract_handlers::update_contract,
        crate::routes::contract_handlers::delete_contract,
        // Attachment
        crate::routes::attachment_handlers::get_attachment,
        crate::routes::attachment_handlers::list_attachments,
        crate::routes::attachment_handlers::create_attachment,
        crate::routes::attachment_handlers::update_attachment,
        crate::routes::attachment_handlers::delete_attachment,
        // AuditLog (read-only)
        crate::routes::audit_log_handlers::get_audit_log,
        crate::routes::audit_log_handlers::list_audit_logs,
        // ChangeRecord (read-only)
        crate::routes::change_record_handlers::get_change_record,
        crate::routes::change_record_handlers::list_change_records,
        // SiteRelation
        crate::routes::site_relation_handlers::list_site_servers,
        crate::routes::site_relation_handlers::create_site_server,
        crate::routes::site_relation_handlers::delete_site_server,
        crate::routes::site_relation_handlers::list_site_databases,
        crate::routes::site_relation_handlers::create_site_database,
        crate::routes::site_relation_handlers::delete_site_database,
        crate::routes::site_relation_handlers::list_site_domains,
        crate::routes::site_relation_handlers::create_site_domain,
        crate::routes::site_relation_handlers::delete_site_domain,
    ),
    components(schemas(
        // Auth
        crate::dto::auth_dto::LoginRequest,
        crate::dto::auth_dto::RegisterRequest,
        crate::dto::auth_dto::AuthResponse,
        crate::dto::auth_dto::UserInfo,
        crate::dto::auth_dto::MeResponse,
        // Common
        crate::dto::provider_dto::ErrorResponse,
        // Provider
        crate::dto::provider_dto::CreateProviderRequest,
        crate::dto::provider_dto::UpdateProviderRequest,
        crate::dto::provider_dto::ListProvidersQuery,
        crate::dto::provider_dto::ProviderResponse,
        crate::dto::provider_dto::ProviderListResponse,
        // DataCenter
        crate::dto::datacenter_dto::CreateDataCenterRequest,
        crate::dto::datacenter_dto::UpdateDataCenterRequest,
        crate::dto::datacenter_dto::ListDataCentersQuery,
        crate::dto::datacenter_dto::DataCenterResponse,
        crate::dto::datacenter_dto::DataCenterListResponse,
        // Server
        crate::dto::server_dto::CreateServerRequest,
        crate::dto::server_dto::UpdateServerRequest,
        crate::dto::server_dto::ListServersQuery,
        crate::dto::server_dto::ServerResponse,
        crate::dto::server_dto::ServerListResponse,
        // ServerIP
        crate::dto::server_ip_dto::CreateServerIpRequest,
        crate::dto::server_ip_dto::UpdateServerIpRequest,
        crate::dto::server_ip_dto::ListServerIpsQuery,
        crate::dto::server_ip_dto::ServerIpResponse,
        crate::dto::server_ip_dto::ServerIpListResponse,
        // ServerPort
        crate::dto::server_port_dto::CreateServerPortRequest,
        crate::dto::server_port_dto::UpdateServerPortRequest,
        crate::dto::server_port_dto::ListServerPortsQuery,
        crate::dto::server_port_dto::ServerPortResponse,
        crate::dto::server_port_dto::ServerPortListResponse,
        // Domain
        crate::dto::domain_dto::CreateDomainRequest,
        crate::dto::domain_dto::UpdateDomainRequest,
        crate::dto::domain_dto::ListDomainsQuery,
        crate::dto::domain_dto::DomainResponse,
        crate::dto::domain_dto::DomainListResponse,
        // Certificate
        crate::dto::certificate_dto::CreateCertificateRequest,
        crate::dto::certificate_dto::UpdateCertificateRequest,
        crate::dto::certificate_dto::ListCertificatesQuery,
        crate::dto::certificate_dto::CertificateResponse,
        crate::dto::certificate_dto::CertificateListResponse,
        // DatabaseInstance
        crate::dto::database_instance_dto::CreateDatabaseInstanceRequest,
        crate::dto::database_instance_dto::UpdateDatabaseInstanceRequest,
        crate::dto::database_instance_dto::ListDatabaseInstancesQuery,
        crate::dto::database_instance_dto::DatabaseInstanceResponse,
        crate::dto::database_instance_dto::DatabaseInstanceListResponse,
        // OpsSite
        crate::dto::ops_site_dto::CreateOpsSiteRequest,
        crate::dto::ops_site_dto::UpdateOpsSiteRequest,
        crate::dto::ops_site_dto::ListOpsSitesQuery,
        crate::dto::ops_site_dto::OpsSiteResponse,
        crate::dto::ops_site_dto::OpsSiteListResponse,
        // BackupPlan
        crate::dto::backup_plan_dto::CreateBackupPlanRequest,
        crate::dto::backup_plan_dto::UpdateBackupPlanRequest,
        crate::dto::backup_plan_dto::ListBackupPlansQuery,
        crate::dto::backup_plan_dto::BackupPlanResponse,
        crate::dto::backup_plan_dto::BackupPlanListResponse,
        // MonitorTarget
        crate::dto::monitor_target_dto::CreateMonitorTargetRequest,
        crate::dto::monitor_target_dto::UpdateMonitorTargetRequest,
        crate::dto::monitor_target_dto::ListMonitorTargetsQuery,
        crate::dto::monitor_target_dto::MonitorTargetResponse,
        crate::dto::monitor_target_dto::MonitorTargetListResponse,
        // Contract
        crate::dto::contract_dto::CreateContractRequest,
        crate::dto::contract_dto::UpdateContractRequest,
        crate::dto::contract_dto::ListContractsQuery,
        crate::dto::contract_dto::ContractResponse,
        crate::dto::contract_dto::ContractListResponse,
        // Attachment
        crate::dto::attachment_dto::CreateAttachmentRequest,
        crate::dto::attachment_dto::ListAttachmentsQuery,
        crate::dto::attachment_dto::AttachmentResponse,
        crate::dto::attachment_dto::AttachmentListResponse,
        // AuditLog
        crate::dto::audit_log_dto::ListAuditLogsQuery,
        crate::dto::audit_log_dto::AuditLogResponse,
        crate::dto::audit_log_dto::AuditLogListResponse,
        // ChangeRecord
        crate::dto::change_record_dto::ListChangeRecordsQuery,
        crate::dto::change_record_dto::ChangeRecordResponse,
        crate::dto::change_record_dto::ChangeRecordListResponse,
        // SiteRelation
        crate::dto::site_relation_dto::CreateSiteServerRelationRequest,
        crate::dto::site_relation_dto::SiteServerRelationResponse,
        crate::dto::site_relation_dto::CreateSiteDatabaseRelationRequest,
        crate::dto::site_relation_dto::SiteDatabaseRelationResponse,
        crate::dto::site_relation_dto::CreateSiteDomainRelationRequest,
        crate::dto::site_relation_dto::SiteDomainRelationResponse,
    )),
    modifiers(&SecurityAddon),
    tags(
        (name = "Auth", description = "认证端点"),
        (name = "Provider", description = "供应商管理"),
        (name = "DataCenter", description = "数据中心管理"),
        (name = "Server", description = "服务器管理"),
        (name = "ServerIP", description = "服务器IP管理"),
        (name = "ServerPort", description = "服务器端口管理"),
        (name = "Domain", description = "域名管理"),
        (name = "Certificate", description = "证书管理"),
        (name = "DatabaseInstance", description = "数据库实例管理"),
        (name = "OpsSite", description = "站点管理"),
        (name = "Credential", description = "凭据管理"),
        (name = "BackupPlan", description = "备份计划管理"),
        (name = "MonitorTarget", description = "监控目标管理"),
        (name = "Contract", description = "合同管理"),
        (name = "Attachment", description = "附件管理"),
        (name = "AuditLog", description = "审计日志"),
        (name = "ChangeRecord", description = "变更记录"),
        (name = "SiteRelation", description = "站点关联管理"),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_auth",
            utoipa::openapi::security::SecurityScheme::Http(
                utoipa::openapi::security::HttpBuilder::new()
                    .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
