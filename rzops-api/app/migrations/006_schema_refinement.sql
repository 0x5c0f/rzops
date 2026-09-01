-- 006_schema_refinement.sql
-- 架构优化：冗余索引清理 + updated_at 触发器 + target_type CHECK + 悬空字段清理 + 软删除 + 级联修正

-- ============================================================
-- 1. 清理冗余索引（唯一索引已覆盖，普通索引纯冗余）
-- ============================================================
DROP INDEX IF EXISTS ix_cmdb_domain_domain_name;
DROP INDEX IF EXISTS ix_cmdb_provider_name;
DROP INDEX IF EXISTS ix_cmdb_server_asset_code;
DROP INDEX IF EXISTS ix_cmdb_server_ip_ip_address;

-- ============================================================
-- 2. updated_at 自动更新触发器
-- ============================================================
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DO $$
DECLARE
    t text;
    tables text[] := ARRAY[
        'cmdb_attachment','cmdb_backup_plan','cmdb_certificate','cmdb_contract',
        'cmdb_data_center','cmdb_database_instance','cmdb_dict','cmdb_domain',
        'cmdb_monitor_target','cmdb_ops_site','cmdb_provider','cmdb_server',
        'cmdb_server_ip','cmdb_server_port'
    ];
BEGIN
    FOREACH t IN ARRAY tables LOOP
        EXECUTE format(
            'DROP TRIGGER IF EXISTS trg_%s_updated_at ON %I;
             CREATE TRIGGER trg_%s_updated_at BEFORE UPDATE ON %I
             FOR EACH ROW EXECUTE FUNCTION set_updated_at()',
            t, t, t, t
        );
    END LOOP;
END $$;

-- ============================================================
-- 3. 多态 target_type CHECK 约束（防脏数据）
-- ============================================================
ALTER TABLE cmdb_backup_plan
    DROP CONSTRAINT IF EXISTS chk_backup_plan_target_type,
    ADD CONSTRAINT chk_backup_plan_target_type
    CHECK (target_type IS NULL OR target_type IN ('server','database','site'));

ALTER TABLE cmdb_monitor_target
    DROP CONSTRAINT IF EXISTS chk_monitor_target_target_type,
    ADD CONSTRAINT chk_monitor_target_target_type
    CHECK (target_type IS NULL OR target_type IN ('server','database','site'));

-- ============================================================
-- 4. ops_site 悬空字段清理（business_unit_id / department_id 无对应表）
-- ============================================================
DROP INDEX IF EXISTS ix_cmdb_ops_site_business_unit_id;
DROP INDEX IF EXISTS ix_cmdb_ops_site_department_id;
ALTER TABLE cmdb_ops_site
    DROP COLUMN IF EXISTS business_unit_id,
    DROP COLUMN IF EXISTS department_id;

-- ============================================================
-- 5. 软删除：5 张核心表加 deleted_at
-- ============================================================
ALTER TABLE cmdb_server ADD COLUMN IF NOT EXISTS deleted_at timestamptz;
ALTER TABLE cmdb_ops_site ADD COLUMN IF NOT EXISTS deleted_at timestamptz;
ALTER TABLE cmdb_database_instance ADD COLUMN IF NOT EXISTS deleted_at timestamptz;
ALTER TABLE cmdb_domain ADD COLUMN IF NOT EXISTS deleted_at timestamptz;
ALTER TABLE cmdb_certificate ADD COLUMN IF NOT EXISTS deleted_at timestamptz;

-- 部分索引：只索引未删除记录，优化常用查询
CREATE INDEX IF NOT EXISTS ix_cmdb_server_alive ON cmdb_server (deleted_at) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS ix_cmdb_ops_site_alive ON cmdb_ops_site (deleted_at) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS ix_cmdb_database_instance_alive ON cmdb_database_instance (deleted_at) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS ix_cmdb_domain_alive ON cmdb_domain (deleted_at) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS ix_cmdb_certificate_alive ON cmdb_certificate (deleted_at) WHERE deleted_at IS NULL;

-- ============================================================
-- 6. certificate_domain.domain_id 级联策略：SET NULL → CASCADE
--    域名删除时，证书-域名绑定记录也应删除（绑定关系无独立存在意义）
-- ============================================================
ALTER TABLE cmdb_certificate_domain
    DROP CONSTRAINT IF EXISTS cmdb_certificate_domain_domain_id_fkey,
    ADD CONSTRAINT cmdb_certificate_domain_domain_id_fkey
    FOREIGN KEY (domain_id) REFERENCES cmdb_domain(id) ON DELETE CASCADE;
