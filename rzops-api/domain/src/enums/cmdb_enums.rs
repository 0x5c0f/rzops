use serde::{Deserialize, Serialize};

// ──────────────────────────────────────────────
// 通用
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CommonStatus {
    Active,
    Inactive,
    Archived,
}

// ──────────────────────────────────────────────
// 供应商
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    Isp,
    Idc,
    Domain,
    Certificate,
    Hardware,
    Software,
    Cloud,
    Other,
}

// ──────────────────────────────────────────────
// 服务器
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServerStatus {
    Active,
    Retired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HostingType {
    Colocation,
    Rental,
    Cloud,
    SelfOwned,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServerType {
    Physical,
    Virtual,
    Cloud,
    Container,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServerRole {
    Web,
    Db,
    Cache,
    Worker,
    File,
    Monitor,
    Backup,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WebServerSoftware {
    Nginx,
    Apache,
    Iis,
    OpenResty,
    Caddy,
    Traefik,
    Tomcat,
    Other,
}

// ──────────────────────────────────────────────
// IP
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IpStatus {
    Enabled,
    Disabled,
    Reserved,
}

// ──────────────────────────────────────────────
// 域名
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DomainPrivacyStatus {
    Enabled,
    Disabled,
    Unknown,
}

// ──────────────────────────────────────────────
// 证书
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CertificateStatus {
    Active,
    Expired,
    Archived,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CertificateType {
    Single,
    MultiDomain,
    Wildcard,
    Other,
}

// ──────────────────────────────────────────────
// 数据库
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseStatus {
    Active,
    Retired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseType {
    Mysql,
    Postgresql,
    Sqlserver,
    Oracle,
    Redis,
    Mongodb,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Importance {
    Critical,
    High,
    Medium,
    Low,
}

// ──────────────────────────────────────────────
// 站点
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SiteStatus {
    Active,
    TemporaryOffline,
    PermanentOffline,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServiceTarget {
    Internal,
    External,
    Partner,
    Mixed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CodeRepoType {
    Svn,
    Git,
    None,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WebFramework {
    Django,
    Flask,
    Fastapi,
    #[serde(rename = "spring_boot")]
    SpringBoot,
    Express,
    Rails,
    Laravel,
    #[serde(rename = "asp_net_mvc")]
    AspNetMvc,
    #[serde(rename = "asp_net_core")]
    AspNetCore,
    Gin,
    Echo,
    Nextjs,
    Nuxtjs,
    #[serde(rename = "ant_design_pro")]
    AntDesignPro,
    Other,
}

// ──────────────────────────────────────────────
// 网络
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    Tcp,
    Udp,
    Http,
    Https,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LineType {
    SingleLine,
    DualLine,
    MultiLine,
    Other,
}

// ──────────────────────────────────────────────
// 站点关系
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SiteServerRole {
    Web,
    Api,
    Worker,
    Static,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SiteDatabaseUsage {
    Primary,
    Replica,
    Analytics,
    Archive,
    Other,
}

// ──────────────────────────────────────────────
// 预留模块
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CredentialType {
    Password,
    SshKey,
    ApiToken,
    Certificate,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReservedStatus {
    Draft,
    Active,
    Inactive,
    Archived,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssetTargetType {
    Server,
    Database,
    Site,
    Domain,
    Certificate,
    Provider,
    DataCenter,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MonitorType {
    Ping,
    Http,
    Tcp,
    Tls,
    Custom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContractStatus {
    Draft,
    Active,
    Expiring,
    Expired,
    Archived,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    Create,
    Update,
    StatusChange,
    Delete,
    Bind,
    Unbind,
}
