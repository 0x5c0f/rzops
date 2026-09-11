# RzOps Rust 重写 — DB 层设计计划

## Context

RzOps 是一个综合运维资产管理平台 (CMDB)，原后端使用 Python (FastAPI + SQLModel + PostgreSQL) 实现。当前任务是用 Rust 重写后端，第一步聚焦数据库层：workspace 骨架 + 表结构定义 + 迁移脚本。

设计原则：
- 数据库架构以 Python 版为参考，允许微调
- 字典系统不保留表驱动，用 Rust enum 原生实现
- 遵循 Rust 7-crate workspace 分层架构
- 使用 sqlx + sqlx-cli 管理迁移

---

## 1. Workspace 结构

```
rzops-api/
├── Cargo.toml                    ← workspace manifest
├── .env                          ← DATABASE_URL 等环境变量
├── migrations/                   ← sqlx-cli 迁移脚本 (workspace 级)
│   └── 001_initial_schema.sql
├── docs/plans/                   ← 设计文档
├── app/
│   ├── Cargo.toml
│   └── src/main.rs               ← 纯组装，无业务逻辑
├── server/
│   ├── Cargo.toml
│   └── src/lib.rs                ← router, state (后续阶段)
├── api/
│   ├── Cargo.toml
│   └── src/lib.rs                ← handlers + DTOs (后续阶段)
├── domain/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── models/               ← 领域实体
│       │   ├── mod.rs
│       │   ├── user.rs
│       │   ├── provider.rs
│       │   ├── data_center.rs
│       │   ├── server.rs
│       │   ├── server_ip.rs
│       │   ├── server_port.rs
│       │   ├── domain_asset.rs   ← 避免与 Rust `domain` crate 同名
│       │   ├── certificate.rs
│       │   ├── certificate_domain.rs
│       │   ├── database_instance.rs
│       │   ├── ops_site.rs
│       │   ├── ops_site_relation.rs
│       │   ├── credential.rs     ← 预留模块
│       │   ├── backup_plan.rs
│       │   ├── monitor_target.rs
│       │   ├── contract.rs
│       │   ├── attachment.rs
│       │   ├── audit_log.rs
│       │   └── change_record.rs
│       └── enums/                ← 枚举类型（替代字典表）
│           ├── mod.rs
│           └── cmdb_enums.rs
├── infra/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── db/
│           ├── mod.rs
│           └── pool.rs           ← PgPool 构建
├── config/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── settings.rs
└── common/
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── errors.rs
        └── types.rs
```

---

## 2. 数据库表结构设计

### 2.1 用户表 (模板原生)

```sql
CREATE TABLE "user" (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email           VARCHAR(255) NOT NULL UNIQUE,
    hashed_password VARCHAR(255) NOT NULL,
    is_active       BOOLEAN NOT NULL DEFAULT TRUE,
    is_superuser    BOOLEAN NOT NULL DEFAULT FALSE,
    full_name       VARCHAR(255),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ix_user_email ON "user" (email);
```

### 2.2 供应商表

```sql
CREATE TABLE cmdb_provider (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(100) NOT NULL UNIQUE,
    provider_types  JSONB NOT NULL DEFAULT '[]',  -- ["isp", "idc", ...]
    contact_name    VARCHAR(100),
    contact_phone   VARCHAR(50),
    contact_qq      VARCHAR(50),
    fax             VARCHAR(50),
    address         VARCHAR(500),
    website         VARCHAR(500),
    country         VARCHAR(50),
    description     TEXT,
    status          VARCHAR(30) NOT NULL DEFAULT 'active',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ix_cmdb_provider_name ON cmdb_provider (name);
CREATE INDEX ix_cmdb_provider_country ON cmdb_provider (country);
CREATE INDEX ix_cmdb_provider_status ON cmdb_provider (status);
```

### 2.3 IDC 机房表

```sql
CREATE TABLE cmdb_data_center (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(100) NOT NULL,
    provider_id     UUID REFERENCES cmdb_provider(id) ON DELETE SET NULL,
    phone           VARCHAR(50),
    address         VARCHAR(500),
    country         VARCHAR(50),
    province        VARCHAR(100),
    city            VARCHAR(100),
    line_type       VARCHAR(50),
    description     TEXT,
    status          VARCHAR(30) NOT NULL DEFAULT 'active',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_cmdb_data_center_provider_name UNIQUE (provider_id, name)
);

CREATE INDEX ix_cmdb_data_center_name ON cmdb_data_center (name);
CREATE INDEX ix_cmdb_data_center_provider_id ON cmdb_data_center (provider_id);
CREATE INDEX ix_cmdb_data_center_country ON cmdb_data_center (country);
CREATE INDEX ix_cmdb_data_center_province ON cmdb_data_center (province);
CREATE INDEX ix_cmdb_data_center_city ON cmdb_data_center (city);
CREATE INDEX ix_cmdb_data_center_line_type ON cmdb_data_center (line_type);
CREATE INDEX ix_cmdb_data_center_status ON cmdb_data_center (status);
```

### 2.4 服务器表

```sql
CREATE TABLE cmdb_server (
    id                   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_code           VARCHAR(100) UNIQUE,
    name                 VARCHAR(100) NOT NULL,
    primary_ip           VARCHAR(45),
    location             VARCHAR(255),
    isp_provider_id      UUID REFERENCES cmdb_provider(id) ON DELETE SET NULL,
    data_center_id       UUID REFERENCES cmdb_data_center(id) ON DELETE SET NULL,
    hosting_type         VARCHAR(50),
    is_dual_line         BOOLEAN NOT NULL DEFAULT FALSE,
    lease_start_date     DATE,
    lease_end_date       DATE,
    price                NUMERIC(12,2),
    price_currency       VARCHAR(3) NOT NULL DEFAULT 'CNY',
    server_type          VARCHAR(50),
    role_tags            JSONB NOT NULL DEFAULT '[]',
    is_database_server   BOOLEAN NOT NULL DEFAULT FALSE,
    cpu                  VARCHAR(255),
    memory_gb            INTEGER,
    is_raid              BOOLEAN NOT NULL DEFAULT FALSE,
    raid_level           VARCHAR(50),
    disk_layout          TEXT,
    hardware_config      TEXT,
    architecture         VARCHAR(20),
    maintainer_id        UUID REFERENCES "user"(id) ON DELETE SET NULL,
    brand                VARCHAR(100),
    warranty_info        TEXT,
    operating_system     VARCHAR(100),
    web_server_type      JSONB NOT NULL DEFAULT '[]',
    server_provider_id   UUID REFERENCES cmdb_provider(id) ON DELETE SET NULL,
    software_provider_id UUID REFERENCES cmdb_provider(id) ON DELETE SET NULL,
    status               VARCHAR(30) NOT NULL DEFAULT 'active',
    offline_time         TIMESTAMPTZ,
    offline_reason       TEXT,
    remarks              TEXT,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ix_cmdb_server_asset_code ON cmdb_server (asset_code);
CREATE INDEX ix_cmdb_server_name ON cmdb_server (name);
CREATE INDEX ix_cmdb_server_primary_ip ON cmdb_server (primary_ip);
CREATE INDEX ix_cmdb_server_isp_provider_id ON cmdb_server (isp_provider_id);
CREATE INDEX ix_cmdb_server_data_center_id ON cmdb_server (data_center_id);
CREATE INDEX ix_cmdb_server_hosting_type ON cmdb_server (hosting_type);
CREATE INDEX ix_cmdb_server_lease_end_date ON cmdb_server (lease_end_date);
CREATE INDEX ix_cmdb_server_server_type ON cmdb_server (server_type);
CREATE INDEX ix_cmdb_server_is_database_server ON cmdb_server (is_database_server);
CREATE INDEX ix_cmdb_server_maintainer_id ON cmdb_server (maintainer_id);
CREATE INDEX ix_cmdb_server_operating_system ON cmdb_server (operating_system);
CREATE INDEX ix_cmdb_server_server_provider_id ON cmdb_server (server_provider_id);
CREATE INDEX ix_cmdb_server_software_provider_id ON cmdb_server (software_provider_id);
CREATE INDEX ix_cmdb_server_status ON cmdb_server (status);
CREATE INDEX ix_cmdb_server_offline_time ON cmdb_server (offline_time);
```

### 2.5 服务器 IP 表

```sql
CREATE TABLE cmdb_server_ip (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    server_id       UUID NOT NULL REFERENCES cmdb_server(id) ON DELETE CASCADE,
    ip_address      VARCHAR(45) NOT NULL UNIQUE,
    ip_type         VARCHAR(50) NOT NULL DEFAULT 'public',
    is_primary      BOOLEAN NOT NULL DEFAULT FALSE,
    isp_provider_id UUID REFERENCES cmdb_provider(id) ON DELETE SET NULL,
    description     TEXT,
    status          VARCHAR(30) NOT NULL DEFAULT 'enabled',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ix_cmdb_server_ip_server_id ON cmdb_server_ip (server_id);
CREATE INDEX ix_cmdb_server_ip_ip_address ON cmdb_server_ip (ip_address);
CREATE INDEX ix_cmdb_server_ip_ip_type ON cmdb_server_ip (ip_type);
CREATE INDEX ix_cmdb_server_ip_is_primary ON cmdb_server_ip (is_primary);
CREATE INDEX ix_cmdb_server_ip_isp_provider_id ON cmdb_server_ip (isp_provider_id);
CREATE INDEX ix_cmdb_server_ip_status ON cmdb_server_ip (status);
```

### 2.6 服务器端口表

```sql
CREATE TABLE cmdb_server_port (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    server_id       UUID NOT NULL REFERENCES cmdb_server(id) ON DELETE CASCADE,
    protocol        VARCHAR(20) NOT NULL,
    port            INTEGER NOT NULL CHECK (port >= 1 AND port <= 65535),
    service_name    VARCHAR(100) NOT NULL,
    access_scope    VARCHAR(50),
    is_enabled      BOOLEAN NOT NULL DEFAULT TRUE,
    description     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_cmdb_server_port_server_protocol_port UNIQUE (server_id, protocol, port)
);

CREATE INDEX ix_cmdb_server_port_server_id ON cmdb_server_port (server_id);
CREATE INDEX ix_cmdb_server_port_protocol ON cmdb_server_port (protocol);
CREATE INDEX ix_cmdb_server_port_port ON cmdb_server_port (port);
CREATE INDEX ix_cmdb_server_port_service_name ON cmdb_server_port (service_name);
CREATE INDEX ix_cmdb_server_port_access_scope ON cmdb_server_port (access_scope);
CREATE INDEX ix_cmdb_server_port_is_enabled ON cmdb_server_port (is_enabled);
```

### 2.7 域名表

```sql
CREATE TABLE cmdb_domain (
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    domain_name             VARCHAR(255) NOT NULL UNIQUE,
    business_unit_id        UUID,
    company_id              UUID,
    expiry_date             DATE,
    renewal_amount          NUMERIC(12,2),
    renewal_currency        VARCHAR(3) NOT NULL DEFAULT 'CNY',
    provider_id             UUID REFERENCES cmdb_provider(id) ON DELETE SET NULL,
    account_credential_id   UUID,
    platform_phone          VARCHAR(50),
    domain_email            VARCHAR(255),
    privacy_status          VARCHAR(50),
    is_enabled              BOOLEAN NOT NULL DEFAULT TRUE,
    remarks                 TEXT,
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ix_cmdb_domain_domain_name ON cmdb_domain (domain_name);
CREATE INDEX ix_cmdb_domain_business_unit_id ON cmdb_domain (business_unit_id);
CREATE INDEX ix_cmdb_domain_company_id ON cmdb_domain (company_id);
CREATE INDEX ix_cmdb_domain_expiry_date ON cmdb_domain (expiry_date);
CREATE INDEX ix_cmdb_domain_provider_id ON cmdb_domain (provider_id);
CREATE INDEX ix_cmdb_domain_account_credential_id ON cmdb_domain (account_credential_id);
CREATE INDEX ix_cmdb_domain_privacy_status ON cmdb_domain (privacy_status);
CREATE INDEX ix_cmdb_domain_is_enabled ON cmdb_domain (is_enabled);
```

### 2.8 证书表

```sql
CREATE TABLE cmdb_certificate (
    id                          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name                        VARCHAR(100) NOT NULL,
    provider_id                 UUID REFERENCES cmdb_provider(id) ON DELETE SET NULL,
    lease_start_date            DATE,
    lease_end_date              DATE,
    certificate_type            VARCHAR(50),
    status                      VARCHAR(30) NOT NULL DEFAULT 'active',
    private_key_credential_id   UUID,
    remarks                     TEXT,
    created_at                  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at                  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ix_cmdb_certificate_name ON cmdb_certificate (name);
CREATE INDEX ix_cmdb_certificate_provider_id ON cmdb_certificate (provider_id);
CREATE INDEX ix_cmdb_certificate_lease_end_date ON cmdb_certificate (lease_end_date);
CREATE INDEX ix_cmdb_certificate_certificate_type ON cmdb_certificate (certificate_type);
CREATE INDEX ix_cmdb_certificate_status ON cmdb_certificate (status);
CREATE INDEX ix_cmdb_certificate_private_key_credential_id ON cmdb_certificate (private_key_credential_id);
```

### 2.9 证书-域名关系表

```sql
CREATE TABLE cmdb_certificate_domain (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    certificate_id  UUID NOT NULL REFERENCES cmdb_certificate(id) ON DELETE CASCADE,
    domain_id       UUID REFERENCES cmdb_domain(id) ON DELETE SET NULL,
    domain_pattern  VARCHAR(255) NOT NULL,
    is_primary      BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_cmdb_certificate_domain_certificate_pattern UNIQUE (certificate_id, domain_pattern)
);

CREATE INDEX ix_cmdb_certificate_domain_certificate_id ON cmdb_certificate_domain (certificate_id);
CREATE INDEX ix_cmdb_certificate_domain_domain_id ON cmdb_certificate_domain (domain_id);
CREATE INDEX ix_cmdb_certificate_domain_domain_pattern ON cmdb_certificate_domain (domain_pattern);
CREATE INDEX ix_cmdb_certificate_domain_is_primary ON cmdb_certificate_domain (is_primary);
```

### 2.10 数据库实例表

```sql
CREATE TABLE cmdb_database_instance (
    id                      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    server_id               UUID REFERENCES cmdb_server(id) ON DELETE SET NULL,
    name                    VARCHAR(100) NOT NULL,
    db_type                 VARCHAR(50) NOT NULL,
    description             TEXT,
    status                  VARCHAR(30) NOT NULL DEFAULT 'active',
    offline_time            TIMESTAMPTZ,
    is_self_installed       BOOLEAN NOT NULL DEFAULT FALSE,
    importance              VARCHAR(50),
    is_ops_managed          BOOLEAN NOT NULL DEFAULT TRUE,
    management_credential_id UUID,
    backup_plan_id          UUID,
    monitor_target_id       UUID,
    port                    INTEGER CHECK (port >= 1 AND port <= 65535),
    instance_name           VARCHAR(100),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_cmdb_database_instance_server_type_port_instance
        UNIQUE (server_id, db_type, port, instance_name)
);

CREATE INDEX ix_cmdb_database_instance_server_id ON cmdb_database_instance (server_id);
CREATE INDEX ix_cmdb_database_instance_name ON cmdb_database_instance (name);
CREATE INDEX ix_cmdb_database_instance_db_type ON cmdb_database_instance (db_type);
CREATE INDEX ix_cmdb_database_instance_status ON cmdb_database_instance (status);
CREATE INDEX ix_cmdb_database_instance_offline_time ON cmdb_database_instance (offline_time);
CREATE INDEX ix_cmdb_database_instance_is_self_installed ON cmdb_database_instance (is_self_installed);
CREATE INDEX ix_cmdb_database_instance_importance ON cmdb_database_instance (importance);
CREATE INDEX ix_cmdb_database_instance_is_ops_managed ON cmdb_database_instance (is_ops_managed);
CREATE INDEX ix_cmdb_database_instance_management_credential_id ON cmdb_database_instance (management_credential_id);
CREATE INDEX ix_cmdb_database_instance_backup_plan_id ON cmdb_database_instance (backup_plan_id);
CREATE INDEX ix_cmdb_database_instance_monitor_target_id ON cmdb_database_instance (monitor_target_id);
CREATE INDEX ix_cmdb_database_instance_port ON cmdb_database_instance (port);
CREATE INDEX ix_cmdb_database_instance_instance_name ON cmdb_database_instance (instance_name);
```

### 2.11 运维站点表

```sql
CREATE TABLE cmdb_ops_site (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name                VARCHAR(100) NOT NULL,
    url                 VARCHAR(500),
    business_unit_id    UUID,
    department_id       UUID,
    service_target      VARCHAR(100),
    importance          VARCHAR(50),
    online_time         TIMESTAMPTZ,
    code_repo_type      VARCHAR(50),
    code_repo_url       VARCHAR(500),
    purpose             TEXT,
    is_internal_system  BOOLEAN NOT NULL DEFAULT FALSE,
    language_runtime    VARCHAR(255),
    web_framework       VARCHAR(100),
    uses_cdn            BOOLEAN,
    is_test_site        BOOLEAN NOT NULL DEFAULT FALSE,
    backup_plan_id      UUID,
    last_backup_time    TIMESTAMPTZ,
    monitor_target_id   UUID,
    status              VARCHAR(30) NOT NULL DEFAULT 'active',
    offline_time        TIMESTAMPTZ,
    offline_reason      VARCHAR(500),
    function_summary    TEXT,
    remarks             TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ix_cmdb_ops_site_name ON cmdb_ops_site (name);
CREATE INDEX ix_cmdb_ops_site_url ON cmdb_ops_site (url);
CREATE INDEX ix_cmdb_ops_site_business_unit_id ON cmdb_ops_site (business_unit_id);
CREATE INDEX ix_cmdb_ops_site_department_id ON cmdb_ops_site (department_id);
CREATE INDEX ix_cmdb_ops_site_service_target ON cmdb_ops_site (service_target);
CREATE INDEX ix_cmdb_ops_site_importance ON cmdb_ops_site (importance);
CREATE INDEX ix_cmdb_ops_site_online_time ON cmdb_ops_site (online_time);
CREATE INDEX ix_cmdb_ops_site_is_internal_system ON cmdb_ops_site (is_internal_system);
CREATE INDEX ix_cmdb_ops_site_uses_cdn ON cmdb_ops_site (uses_cdn);
CREATE INDEX ix_cmdb_ops_site_is_test_site ON cmdb_ops_site (is_test_site);
CREATE INDEX ix_cmdb_ops_site_backup_plan_id ON cmdb_ops_site (backup_plan_id);
CREATE INDEX ix_cmdb_ops_site_last_backup_time ON cmdb_ops_site (last_backup_time);
CREATE INDEX ix_cmdb_ops_site_monitor_target_id ON cmdb_ops_site (monitor_target_id);
CREATE INDEX ix_cmdb_ops_site_status ON cmdb_ops_site (status);
CREATE INDEX ix_cmdb_ops_site_web_framework ON cmdb_ops_site (web_framework);
CREATE INDEX ix_cmdb_ops_site_offline_time ON cmdb_ops_site (offline_time);
```

### 2.12 站点-服务器关系表

```sql
CREATE TABLE cmdb_ops_site_server (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id     UUID NOT NULL REFERENCES cmdb_ops_site(id) ON DELETE CASCADE,
    server_id   UUID NOT NULL REFERENCES cmdb_server(id) ON DELETE CASCADE,
    deploy_role VARCHAR(50),
    is_primary  BOOLEAN NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_cmdb_ops_site_server_site_server_role UNIQUE (site_id, server_id, deploy_role)
);

CREATE INDEX ix_cmdb_ops_site_server_site_id ON cmdb_ops_site_server (site_id);
CREATE INDEX ix_cmdb_ops_site_server_server_id ON cmdb_ops_site_server (server_id);
CREATE INDEX ix_cmdb_ops_site_server_deploy_role ON cmdb_ops_site_server (deploy_role);
CREATE INDEX ix_cmdb_ops_site_server_is_primary ON cmdb_ops_site_server (is_primary);
```

### 2.13 站点-数据库关系表

```sql
CREATE TABLE cmdb_ops_site_database (
    id                   UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id              UUID NOT NULL REFERENCES cmdb_ops_site(id) ON DELETE CASCADE,
    database_instance_id UUID NOT NULL REFERENCES cmdb_database_instance(id) ON DELETE CASCADE,
    usage_type           VARCHAR(50),
    is_primary           BOOLEAN NOT NULL DEFAULT FALSE,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_cmdb_ops_site_database_site_database_usage
        UNIQUE (site_id, database_instance_id, usage_type)
);

CREATE INDEX ix_cmdb_ops_site_database_site_id ON cmdb_ops_site_database (site_id);
CREATE INDEX ix_cmdb_ops_site_database_database_instance_id ON cmdb_ops_site_database (database_instance_id);
CREATE INDEX ix_cmdb_ops_site_database_usage_type ON cmdb_ops_site_database (usage_type);
CREATE INDEX ix_cmdb_ops_site_database_is_primary ON cmdb_ops_site_database (is_primary);
```

### 2.14 站点-域名关系表

```sql
CREATE TABLE cmdb_ops_site_domain (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    site_id     UUID NOT NULL REFERENCES cmdb_ops_site(id) ON DELETE CASCADE,
    domain_id   UUID NOT NULL REFERENCES cmdb_domain(id) ON DELETE CASCADE,
    is_primary  BOOLEAN NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT uq_cmdb_ops_site_domain_site_domain UNIQUE (site_id, domain_id)
);

CREATE INDEX ix_cmdb_ops_site_domain_site_id ON cmdb_ops_site_domain (site_id);
CREATE INDEX ix_cmdb_ops_site_domain_domain_id ON cmdb_ops_site_domain (domain_id);
CREATE INDEX ix_cmdb_ops_site_domain_is_primary ON cmdb_ops_site_domain (is_primary);
```

### 2.15 预留模块 — 凭据表

```sql
CREATE TABLE cmdb_credential (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(100) NOT NULL,
    credential_type VARCHAR(50) NOT NULL,
    username        VARCHAR(255),
    secret_ref      VARCHAR(500),
    owner_id        UUID REFERENCES "user"(id) ON DELETE SET NULL,
    status          VARCHAR(30) NOT NULL DEFAULT 'active',
    remarks         TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ix_cmdb_credential_name ON cmdb_credential (name);
CREATE INDEX ix_cmdb_credential_credential_type ON cmdb_credential (credential_type);
CREATE INDEX ix_cmdb_credential_owner_id ON cmdb_credential (owner_id);
CREATE INDEX ix_cmdb_credential_status ON cmdb_credential (status);
```

### 2.16 预留模块 — 备份计划表

```sql
CREATE TABLE cmdb_backup_plan (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(100) NOT NULL,
    target_type     VARCHAR(50),
    target_id       UUID,
    schedule        VARCHAR(100),
    retention_days  INTEGER,
    status          VARCHAR(30) NOT NULL DEFAULT 'draft',
    remarks         TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ix_cmdb_backup_plan_name ON cmdb_backup_plan (name);
CREATE INDEX ix_cmdb_backup_plan_target_type ON cmdb_backup_plan (target_type);
CREATE INDEX ix_cmdb_backup_plan_target_id ON cmdb_backup_plan (target_id);
CREATE INDEX ix_cmdb_backup_plan_status ON cmdb_backup_plan (status);
```

### 2.17 预留模块 — 监控目标表

```sql
CREATE TABLE cmdb_monitor_target (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(100) NOT NULL,
    target_type     VARCHAR(50),
    target_id       UUID,
    monitor_type    VARCHAR(50),
    endpoint        VARCHAR(500),
    interval_seconds INTEGER,
    status          VARCHAR(30) NOT NULL DEFAULT 'draft',
    remarks         TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ix_cmdb_monitor_target_name ON cmdb_monitor_target (name);
CREATE INDEX ix_cmdb_monitor_target_target_type ON cmdb_monitor_target (target_type);
CREATE INDEX ix_cmdb_monitor_target_target_id ON cmdb_monitor_target (target_id);
CREATE INDEX ix_cmdb_monitor_target_monitor_type ON cmdb_monitor_target (monitor_type);
CREATE INDEX ix_cmdb_monitor_target_status ON cmdb_monitor_target (status);
```

### 2.18 预留模块 — 合同表

```sql
CREATE TABLE cmdb_contract (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(100) NOT NULL,
    provider_id     UUID REFERENCES cmdb_provider(id) ON DELETE SET NULL,
    subject_type    VARCHAR(50),
    subject_id      UUID,
    contract_no     VARCHAR(100),
    start_date      DATE,
    end_date        DATE,
    amount          NUMERIC(12,2),
    currency        VARCHAR(3) NOT NULL DEFAULT 'CNY',
    status          VARCHAR(30) NOT NULL DEFAULT 'draft',
    remarks         TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ix_cmdb_contract_name ON cmdb_contract (name);
CREATE INDEX ix_cmdb_contract_provider_id ON cmdb_contract (provider_id);
CREATE INDEX ix_cmdb_contract_subject_type ON cmdb_contract (subject_type);
CREATE INDEX ix_cmdb_contract_subject_id ON cmdb_contract (subject_id);
CREATE INDEX ix_cmdb_contract_contract_no ON cmdb_contract (contract_no);
CREATE INDEX ix_cmdb_contract_end_date ON cmdb_contract (end_date);
CREATE INDEX ix_cmdb_contract_status ON cmdb_contract (status);
```

### 2.19 预留模块 — 附件表

```sql
CREATE TABLE cmdb_attachment (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    filename        VARCHAR(255) NOT NULL,
    target_type     VARCHAR(50),
    target_id       UUID,
    storage_key     VARCHAR(500),
    content_type    VARCHAR(100),
    size_bytes      BIGINT,
    uploaded_by_id  UUID REFERENCES "user"(id) ON DELETE SET NULL,
    status          VARCHAR(30) NOT NULL DEFAULT 'active',
    remarks         TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ix_cmdb_attachment_filename ON cmdb_attachment (filename);
CREATE INDEX ix_cmdb_attachment_target_type ON cmdb_attachment (target_type);
CREATE INDEX ix_cmdb_attachment_target_id ON cmdb_attachment (target_id);
CREATE INDEX ix_cmdb_attachment_uploaded_by_id ON cmdb_attachment (uploaded_by_id);
CREATE INDEX ix_cmdb_attachment_status ON cmdb_attachment (status);
```

### 2.20 审计日志表

```sql
CREATE TABLE cmdb_audit_log (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id        UUID REFERENCES "user"(id) ON DELETE SET NULL,
    action          VARCHAR(100) NOT NULL,
    resource_type   VARCHAR(100) NOT NULL,
    resource_id     UUID,
    ip_address      VARCHAR(45),
    user_agent      VARCHAR(500),
    extra_data      JSONB NOT NULL DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ix_cmdb_audit_log_actor_id ON cmdb_audit_log (actor_id);
CREATE INDEX ix_cmdb_audit_log_action ON cmdb_audit_log (action);
CREATE INDEX ix_cmdb_audit_log_resource_type ON cmdb_audit_log (resource_type);
CREATE INDEX ix_cmdb_audit_log_resource_id ON cmdb_audit_log (resource_id);
CREATE INDEX ix_cmdb_audit_log_created_at ON cmdb_audit_log (created_at);
```

### 2.21 变更记录表

```sql
CREATE TABLE cmdb_change_record (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    actor_id        UUID REFERENCES "user"(id) ON DELETE SET NULL,
    change_type     VARCHAR(50) NOT NULL,
    resource_type   VARCHAR(100) NOT NULL,
    resource_id     UUID,
    before_data     JSONB NOT NULL DEFAULT '{}',
    after_data      JSONB NOT NULL DEFAULT '{}',
    remarks         TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ix_cmdb_change_record_actor_id ON cmdb_change_record (actor_id);
CREATE INDEX ix_cmdb_change_record_change_type ON cmdb_change_record (change_type);
CREATE INDEX ix_cmdb_change_record_resource_type ON cmdb_change_record (resource_type);
CREATE INDEX ix_cmdb_change_record_resource_id ON cmdb_change_record (resource_id);
CREATE INDEX ix_cmdb_change_record_created_at ON cmdb_change_record (created_at);
```

---

## 3. Rust 枚举设计 (替代字典表)

```rust
// domain/src/enums/cmdb_enums.rs

use serde::{Deserialize, Serialize};

// === 通用 ===

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CommonStatus {
    Active,
    Inactive,
    Archived,
}

// === 供应商 ===

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

// === 服务器 ===

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServerStatus {
    Active,
    Retired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HostingType {
    Colocation,
    Rental,
    Cloud,
    SelfOwned,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServerType {
    Physical,
    Virtual,
    Cloud,
    Container,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

// === IP ===

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IpStatus {
    Enabled,
    Disabled,
    Reserved,
}

// === 域名 ===

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DomainPrivacyStatus {
    Enabled,
    Disabled,
    Unknown,
}

// === 证书 ===

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CertificateStatus {
    Active,
    Expired,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CertificateType {
    Single,
    MultiDomain,
    Wildcard,
    Other,
}

// === 数据库 ===

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseStatus {
    Active,
    Retired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Importance {
    Critical,
    High,
    Medium,
    Low,
}

// === 站点 ===

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SiteStatus {
    Active,
    TemporaryOffline,
    PermanentOffline,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ServiceTarget {
    Internal,
    External,
    Partner,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CodeRepoType {
    Svn,
    Git,
    None,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WebFramework {
    Django,
    Flask,
    Fastapi,
    SpringBoot,
    Express,
    Rails,
    Laravel,
    AspNetMvc,
    AspNetCore,
    Gin,
    Echo,
    Nextjs,
    Nuxtjs,
    AntDesignPro,
    Other,
}

// === 网络 ===

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    Tcp,
    Udp,
    Http,
    Https,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LineType {
    SingleLine,
    DualLine,
    MultiLine,
    Other,
}

// === 站点关系 ===

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SiteServerRole {
    Web,
    Api,
    Worker,
    Static,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SiteDatabaseUsage {
    Primary,
    Replica,
    Analytics,
    Archive,
    Other,
}

// === 预留模块 ===

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CredentialType {
    Password,
    SshKey,
    ApiToken,
    Certificate,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReservedStatus {
    Draft,
    Active,
    Inactive,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MonitorType {
    Ping,
    Http,
    Tcp,
    Tls,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContractStatus {
    Draft,
    Active,
    Expiring,
    Expired,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChangeType {
    Create,
    Update,
    StatusChange,
    Delete,
    Bind,
    Unbind,
}
```

---

## 4. 微调点说明

相比 Python 版，Rust 版有以下微调：

| 项目 | Python 版 | Rust 版 | 原因 |
|------|----------|---------|------|
| 字典系统 | `dict_type` + `dict_item` 表 | Rust enum | 类型安全，编译期检查，无需运行时查询 |
| `dict_item.extra_data` | VARCHAR (JSON 字符串) | 不需要 | 枚举的附加信息（如颜色）在前端处理 |
| `Item` 表 | 模板示例表 | 删除 | 不属于 CMDB 业务 |
| `cmdb_domain` 命名 | `domain` (与表名冲突) | 模块名 `domain_asset` | 避免与 workspace `domain` crate 同名 |
| 预留模块外键 | 部分有约束 | 保持一致 | `target_type` + `target_id` 多态关联保留灵活设计 |
| 枚举存储 | VARCHAR + 字典表关联 | VARCHAR + Rust enum 验证 | 存储格式相同，验证层在 Rust |

---

## 5. 实施步骤

### Step 1: 初始化 Workspace
- 创建 `Cargo.toml` workspace manifest
- 创建 7 个子 crate 的 `Cargo.toml`
- 配置依赖关系

### Step 2: 创建迁移脚本
- 编写 `migrations/001_initial_schema.sql`（上述所有 DDL）
- 配置 `.env` 中的 `DATABASE_URL`

### Step 3: 实现 domain 层
- 创建所有枚举类型 (`enums/cmdb_enums.rs`)
- 创建所有领域模型 (`models/*.rs`)
- 每个模型对应一张数据库表

### Step 4: 实现 infra 层
- 创建 `db/pool.rs` — PgPool 构建
- 后续阶段实现 repository

### Step 5: 实现 config 层
- 创建 `settings.rs` — 环境变量配置

### Step 6: 实现 common 层
- 创建 `errors.rs` — 基础错误类型
- 创建 `types.rs` — 共享类型

### Step 7: 验证
- `cargo check` 确保编译通过
- `sqlx migrate run` 确保迁移成功
- 检查数据库表结构正确性

---

## 6. 验证方式

```bash
# 1. 编译检查
cargo check --workspace

# 2. 数据库迁移
sqlx migrate run

# 3. 验证表结构
psql -d rzops -c "\dt cmdb_*"
psql -d rzops -c "\d cmdb_server"

# 4. 验证索引
psql -d rzops -c "\di ix_cmdb_*"
```

---

## 7. 后续阶段 (不在本次范围内)

- **API 层**: 实现 Axum handlers + DTOs
- **Server 层**: 路由注册、中间件、认证
- **App 层**: 组装启动
- **CRUD 操作**: Repository 实现
- **种子数据**: 初始管理员用户创建
