-- 008: 移除关联表is_primary，增加environment和domain_role字段

-- 服务器增加环境字段
ALTER TABLE cmdb_server ADD COLUMN IF NOT EXISTS environment VARCHAR(50);
COMMENT ON COLUMN cmdb_server.environment IS '环境：prod生产/test测试/staging预发布/dev开发';

-- 数据库实例增加环境字段
ALTER TABLE cmdb_database_instance ADD COLUMN IF NOT EXISTS environment VARCHAR(50);
COMMENT ON COLUMN cmdb_database_instance.environment IS '环境：prod生产/test测试/staging预发布/dev开发';

-- 站点增加环境字段（保留is_test_site兼容）
ALTER TABLE cmdb_ops_site ADD COLUMN IF NOT EXISTS environment VARCHAR(50);
COMMENT ON COLUMN cmdb_ops_site.environment IS '环境：prod生产/test测试/staging预发布/dev开发';

-- 站点-服务器关联：移除is_primary
ALTER TABLE cmdb_ops_site_server DROP COLUMN IF EXISTS is_primary;

-- 站点-数据库关联：移除is_primary
ALTER TABLE cmdb_ops_site_database DROP COLUMN IF EXISTS is_primary;

-- 站点-域名关联：移除is_primary，增加domain_role
ALTER TABLE cmdb_ops_site_domain DROP COLUMN IF EXISTS is_primary;
ALTER TABLE cmdb_ops_site_domain ADD COLUMN IF NOT EXISTS domain_role VARCHAR(50);
COMMENT ON COLUMN cmdb_ops_site_domain.domain_role IS '域名角色：primary主域名/alias别名/redirect跳转';

-- 移除唯一约束中的is_primary（如果存在）
ALTER TABLE cmdb_ops_site_server DROP CONSTRAINT IF EXISTS uq_cmdb_ops_site_server_site_server_role;
ALTER TABLE cmdb_ops_site_server ADD CONSTRAINT uq_cmdb_ops_site_server_site_server_role UNIQUE (site_id, server_id, deploy_role);

ALTER TABLE cmdb_ops_site_database DROP CONSTRAINT IF EXISTS uq_cmdb_ops_site_database_site_database_usage;
ALTER TABLE cmdb_ops_site_database ADD CONSTRAINT uq_cmdb_ops_site_database_site_database_usage UNIQUE (site_id, database_instance_id, usage_type);

-- 站点-域名唯一约束
ALTER TABLE cmdb_ops_site_domain DROP CONSTRAINT IF EXISTS uq_cmdb_ops_site_domain_site_domain;
ALTER TABLE cmdb_ops_site_domain ADD CONSTRAINT uq_cmdb_ops_site_domain_site_domain UNIQUE (site_id, domain_id);
