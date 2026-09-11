-- 010_system_soft_delete.sql
-- 用户表与角色表补齐软删除字段（纳入回收站体系）

ALTER TABLE "user" ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
COMMENT ON COLUMN "user".deleted_at IS '软删除时间，非空表示已删除';

ALTER TABLE role ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
COMMENT ON COLUMN role.deleted_at IS '软删除时间，非空表示已删除';

-- 内置角色不可软删除的约束由应用层保证
