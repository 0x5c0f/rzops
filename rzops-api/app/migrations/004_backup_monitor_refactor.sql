-- ============================================
-- 004: 备份计划 / 监控目标 关系重构
-- 站点、数据库实例不再持有单值外键（backup_plan_id / monitor_target_id），
-- 统一通过 backup_plan / monitor_target 自身的 target_type + target_id 多态反向关联（一对多）。
-- 监控目标移除冗余的 site_id，统一用 target_type='site' + target_id 表达站点归属。
-- ============================================

ALTER TABLE cmdb_ops_site
    DROP COLUMN IF EXISTS backup_plan_id,
    DROP COLUMN IF EXISTS monitor_target_id;

ALTER TABLE cmdb_database_instance
    DROP COLUMN IF EXISTS backup_plan_id,
    DROP COLUMN IF EXISTS monitor_target_id;

ALTER TABLE cmdb_monitor_target
    DROP COLUMN IF EXISTS site_id;
