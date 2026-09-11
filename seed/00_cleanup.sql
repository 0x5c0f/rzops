-- ============================================
-- 清理脚本：保留字典数据与 admin 登录账号，清理其余所有业务数据
-- ============================================
SET client_encoding = 'UTF8';

-- 先删除依赖子表
TRUNCATE TABLE cmdb_ops_site_server CASCADE;
TRUNCATE TABLE cmdb_ops_site_database CASCADE;
TRUNCATE TABLE cmdb_ops_site_domain CASCADE;
TRUNCATE TABLE cmdb_certificate_domain CASCADE;

-- 业务主表
TRUNCATE TABLE cmdb_attachment CASCADE;
TRUNCATE TABLE cmdb_audit_log CASCADE;
TRUNCATE TABLE cmdb_backup_plan CASCADE;
TRUNCATE TABLE cmdb_certificate CASCADE;
TRUNCATE TABLE cmdb_change_record CASCADE;
TRUNCATE TABLE cmdb_contract CASCADE;
TRUNCATE TABLE cmdb_data_center CASCADE;
TRUNCATE TABLE cmdb_database_instance CASCADE;
TRUNCATE TABLE cmdb_domain CASCADE;
TRUNCATE TABLE cmdb_monitor_target CASCADE;
TRUNCATE TABLE cmdb_ops_site CASCADE;
TRUNCATE TABLE cmdb_provider CASCADE;
TRUNCATE TABLE cmdb_server CASCADE;
TRUNCATE TABLE cmdb_server_ip CASCADE;
TRUNCATE TABLE cmdb_server_port_server CASCADE;
TRUNCATE TABLE cmdb_server_port CASCADE;

-- 清理测试残留用户（保留 admin）
DELETE FROM "user" WHERE email <> 'admin@rzops.local';

-- 重置序列（PG 无序列依赖 uuid，无需处理）

SELECT 'cleanup done' AS status;
