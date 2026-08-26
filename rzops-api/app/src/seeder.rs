use sqlx::{Pool, Postgres};
use uuid::Uuid;
use chrono::Utc;

/// Seed the database with an initial admin user.
/// Reads credentials from environment variables:
///   - RZOPS_SEED_ADMIN_EMAIL (default: admin@rzops.local)
///   - RZOPS_SEED_ADMIN_PASSWORD (default: admin123)
pub async fn seed_admin_user(pool: &Pool<Postgres>) -> Result<(), anyhow::Error> {
    let email = std::env::var("RZOPS_SEED_ADMIN_EMAIL")
        .unwrap_or_else(|_| "admin@rzops.local".to_string());
    let password = std::env::var("RZOPS_SEED_ADMIN_PASSWORD")
        .unwrap_or_else(|_| "admin123".to_string());

    // Check if user already exists
    let existing: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM \"user\" WHERE email = $1"
    )
    .bind(&email)
    .fetch_optional(pool)
    .await?;

    if existing.is_some() {
        tracing::info!("admin user '{}' already exists, skipping seed", email);
        return Ok(());
    }

    let hashed = bcrypt::hash(&password, bcrypt::DEFAULT_COST)?;
    let now = Utc::now();

    sqlx::query(
        r#"INSERT INTO "user" (id, email, hashed_password, is_active, is_superuser, full_name, created_at)
           VALUES ($1, $2, $3, true, true, 'Administrator', $4)"#
    )
    .bind(Uuid::new_v4())
    .bind(&email)
    .bind(&hashed)
    .bind(now)
    .execute(pool)
    .await?;

    tracing::info!("seeded admin user: {}", email);
    Ok(())
}

/// (dict_type, dict_code, dict_label) 初始字典种子数据。
/// 替代原先硬编码的 Rust 枚举，作为各表单下拉选项的可维护来源。
const DICT_SEEDS: &[(&str, &str, &str)] = &[
    // ── 通用状态（provider / data_center / monitor_target 等）──
    ("common_status", "active", "活跃"),
    ("common_status", "inactive", "停用"),
    ("common_status", "archived", "已归档"),
    // ── 预留状态（credential / attachment / backup_plan）──
    ("reserved_status", "draft", "草稿"),
    ("reserved_status", "active", "活跃"),
    ("reserved_status", "inactive", "停用"),
    ("reserved_status", "archived", "已归档"),
    // ── 服务器 ──
    ("server_status", "active", "运行中"),
    ("server_status", "retired", "已退役"),
    ("hosting_type", "colocation", "托管"),
    ("hosting_type", "rental", "租赁"),
    ("hosting_type", "cloud", "云"),
    ("hosting_type", "self_owned", "自有"),
    ("hosting_type", "other", "其他"),
    ("server_type", "physical", "物理机"),
    ("server_type", "virtual", "虚拟机"),
    ("server_type", "cloud", "云服务器"),
    ("server_type", "container", "容器"),
    ("server_type", "other", "其他"),
    ("server_role", "web", "Web"),
    ("server_role", "db", "数据库"),
    ("server_role", "cache", "缓存"),
    ("server_role", "worker", "Worker"),
    ("server_role", "file", "文件"),
    ("server_role", "monitor", "监控"),
    ("server_role", "backup", "备份"),
    ("server_role", "other", "其他"),
    ("architecture", "x86_64", "x86_64"),
    ("architecture", "arm64", "arm64"),
    ("raid_level", "raid0", "RAID 0"),
    ("raid_level", "raid1", "RAID 1"),
    ("raid_level", "raid5", "RAID 5"),
    ("raid_level", "raid6", "RAID 6"),
    ("raid_level", "raid10", "RAID 10"),
    ("raid_level", "raid50", "RAID 50"),
    ("raid_level", "raid60", "RAID 60"),
    ("raid_level", "jbod", "JBOD"),
    ("raid_level", "soft_raid", "软RAID"),
    ("raid_level", "hardware_raid", "硬RAID"),
    ("web_server_type", "nginx", "Nginx"),
    ("web_server_type", "apache", "Apache"),
    ("web_server_type", "iis", "IIS"),
    ("web_server_type", "openresty", "OpenResty"),
    ("web_server_type", "caddy", "Caddy"),
    ("web_server_type", "traefik", "Traefik"),
    ("web_server_type", "tomcat", "Tomcat"),
    ("web_server_type", "other", "其他"),
    // ── 服务器 IP / 端口 ──
    ("ip_status", "enabled", "启用"),
    ("ip_status", "disabled", "停用"),
    ("ip_status", "reserved", "预留"),
    ("ip_type", "public", "公网"),
    ("ip_type", "private", "内网"),
    ("ip_type", "management", "管理"),
    ("ip_type", "backup", "备份"),
    ("ip_type", "vip", "VIP"),
    ("protocol", "tcp", "TCP"),
    ("protocol", "udp", "UDP"),
    ("protocol", "http", "HTTP"),
    ("protocol", "https", "HTTPS"),
    // ── 供应商 ──
    ("provider_type", "isp", "ISP"),
    ("provider_type", "idc", "IDC"),
    ("provider_type", "domain", "域名"),
    ("provider_type", "certificate", "证书"),
    ("provider_type", "hardware", "硬件"),
    ("provider_type", "software", "软件"),
    ("provider_type", "cloud", "云"),
    ("provider_type", "other", "其他"),
    // ── 证书 ──
    ("certificate_status", "active", "有效"),
    ("certificate_status", "expired", "已过期"),
    ("certificate_status", "archived", "已归档"),
    ("certificate_type", "single", "单域名"),
    ("certificate_type", "multi_domain", "多域名"),
    ("certificate_type", "wildcard", "通配符"),
    ("certificate_type", "other", "其他"),
    // ── 数据库 ──
    ("db_type", "mysql", "MySQL"),
    ("db_type", "postgresql", "PostgreSQL"),
    ("db_type", "sqlserver", "SQL Server"),
    ("db_type", "oracle", "Oracle"),
    ("db_type", "redis", "Redis"),
    ("db_type", "mongodb", "MongoDB"),
    ("db_type", "other", "其他"),
    ("db_status", "active", "运行中"),
    ("db_status", "retired", "已退役"),
    ("importance", "critical", "关键"),
    ("importance", "high", "高"),
    ("importance", "medium", "中"),
    ("importance", "low", "低"),
    // ── 站点 ──
    ("site_status", "active", "运行中"),
    ("site_status", "temporary_offline", "临时下线"),
    ("site_status", "permanent_offline", "永久下线"),
    ("service_target", "internal", "内部"),
    ("service_target", "external", "外部"),
    ("service_target", "partner", "合作伙伴"),
    ("service_target", "mixed", "混合"),
    ("code_repo_type", "git", "Git"),
    ("code_repo_type", "svn", "SVN"),
    ("code_repo_type", "none", "无"),
    ("code_repo_type", "other", "其他"),
    ("web_framework", "django", "Django"),
    ("web_framework", "flask", "Flask"),
    ("web_framework", "fastapi", "FastAPI"),
    ("web_framework", "spring_boot", "Spring Boot"),
    ("web_framework", "express", "Express"),
    ("web_framework", "rails", "Rails"),
    ("web_framework", "laravel", "Laravel"),
    ("web_framework", "asp_net_mvc", "ASP.NET MVC"),
    ("web_framework", "asp_net_core", "ASP.NET Core"),
    ("web_framework", "gin", "Gin"),
    ("web_framework", "echo", "Echo"),
    ("web_framework", "nextjs", "Next.js"),
    ("web_framework", "nuxtjs", "Nuxt.js"),
    ("web_framework", "ant_design_pro", "Ant Design Pro"),
    ("web_framework", "other", "其他"),
    // ── 凭据 ──
    ("credential_type", "password", "密码"),
    ("credential_type", "ssh_key", "SSH密钥"),
    ("credential_type", "api_token", "API Token"),
    ("credential_type", "certificate", "证书"),
    ("credential_type", "other", "其他"),
    // ── 监控 ──
    ("monitor_type", "ping", "Ping"),
    ("monitor_type", "http", "HTTP"),
    ("monitor_type", "tcp", "TCP"),
    ("monitor_type", "tls", "TLS"),
    ("monitor_type", "custom", "自定义"),
    // ── 合同 ──
    ("contract_status", "draft", "草稿"),
    ("contract_status", "active", "生效中"),
    ("contract_status", "expiring", "即将到期"),
    ("contract_status", "expired", "已过期"),
    ("contract_status", "archived", "已归档"),
    // ── 数据中心 ──
    ("line_type", "single_line", "单线"),
    ("line_type", "dual_line", "双线"),
    ("line_type", "multi_line", "多线"),
    ("line_type", "other", "其他"),
    // ── 域名 ──
    ("domain_privacy_status", "enabled", "启用"),
    ("domain_privacy_status", "disabled", "停用"),
    ("domain_privacy_status", "unknown", "未知"),
    // ── 站点关系 ──
    ("site_server_role", "web", "Web"),
    ("site_server_role", "api", "API"),
    ("site_server_role", "worker", "Worker"),
    ("site_server_role", "static", "静态"),
    ("site_server_role", "other", "其他"),
    ("site_database_usage", "primary", "主"),
    ("site_database_usage", "replica", "副本"),
    ("site_database_usage", "analytics", "分析"),
    ("site_database_usage", "archive", "归档"),
    ("site_database_usage", "other", "其他"),
    // ── 附件/备份/监控目标类型 ──
    ("asset_target_type", "server", "服务器"),
    ("asset_target_type", "database", "数据库"),
    ("asset_target_type", "site", "站点"),
    ("asset_target_type", "domain", "域名"),
    ("asset_target_type", "certificate", "证书"),
    ("asset_target_type", "provider", "供应商"),
    ("asset_target_type", "data_center", "数据中心"),
    ("asset_target_type", "other", "其他"),
];

/// Seed 数据字典（cmdb_dict）。幂等：ON CONFLICT DO NOTHING。
pub async fn seed_dicts(pool: &Pool<Postgres>) -> Result<(), anyhow::Error> {
    let now = Utc::now();
    let mut sort_by_type: std::collections::HashMap<&str, i32> = Default::default();
    let mut inserted = 0usize;
    for (ty, code, label) in DICT_SEEDS {
        let order = {
            let e = sort_by_type.entry(ty).or_insert(0);
            *e += 1;
            *e
        };
        let res = sqlx::query(
            r#"INSERT INTO cmdb_dict (id, dict_type, dict_code, dict_label, sort_order, enabled, remark, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, TRUE, NULL, $6, $6)
               ON CONFLICT (dict_type, dict_code) DO NOTHING"#,
        )
        .bind(Uuid::new_v4())
        .bind(ty)
        .bind(code)
        .bind(label)
        .bind(order)
        .bind(now)
        .execute(pool)
        .await?;
        if res.rows_affected() > 0 {
            inserted += 1;
        }
    }
    tracing::info!("dict seed complete: {} new items inserted", inserted);
    Ok(())
}
