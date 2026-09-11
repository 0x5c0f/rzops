-- 009_rbac.sql
-- RBAC 权限体系 + 全表软删除统一

-- ============================================================
-- 1. 角色表
-- ============================================================
CREATE TABLE IF NOT EXISTS role (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code        VARCHAR(50)  NOT NULL UNIQUE,
    name        VARCHAR(100) NOT NULL,
    description TEXT,
    is_builtin  BOOLEAN NOT NULL DEFAULT FALSE,
    is_active   BOOLEAN NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);
COMMENT ON TABLE role IS '角色表';

-- ============================================================
-- 2. 用户-角色关联（多对多）
-- ============================================================
CREATE TABLE IF NOT EXISTS user_role (
    user_id UUID NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES role(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);
COMMENT ON TABLE user_role IS '用户-角色关联表（多对多）';

-- ============================================================
-- 3. 角色-权限关联
-- ============================================================
CREATE TABLE IF NOT EXISTS role_permission (
    role_id          UUID NOT NULL REFERENCES role(id) ON DELETE CASCADE,
    permission_code  VARCHAR(100) NOT NULL,
    PRIMARY KEY (role_id, permission_code)
);
COMMENT ON TABLE role_permission IS '角色-权限关联表';
CREATE INDEX IF NOT EXISTS ix_role_permission_code ON role_permission(permission_code);

-- ============================================================
-- 4. 预置角色种子
-- ============================================================
INSERT INTO role (id, code, name, description, is_builtin) VALUES
  (gen_random_uuid(), 'admin',  '超级管理员', '拥有系统全部权限', TRUE),
  (gen_random_uuid(), 'ops',    '运维工程师', '业务资源增删改查，负责日常运维配置维护', TRUE),
  (gen_random_uuid(), 'audit',  '审计员',     '所有资源只读，可查看审计日志与变更记录', TRUE),
  (gen_random_uuid(), 'viewer', '只读访客',   '所有资源只读，仅可查看', TRUE)
ON CONFLICT (code) DO NOTHING;

-- ============================================================
-- 5. 权限点种子（角色-权限）
-- ============================================================
DO $$
DECLARE
    v_admin  uuid; v_ops uuid; v_audit uuid; v_viewer uuid;
    p text; r text;
    biz_resources text[] := ARRAY['server','datacenter','provider','domain','certificate','server_ip','server_port','server_port_template','ops_site','database_instance','backup_plan','monitor_target','attachment','dict'];
    actions text[] := ARRAY['read','create','update','delete'];
    sys_perms text[] := ARRAY['system:user','system:role','system:recycle','system:audit','system:change'];
BEGIN
    SELECT id INTO v_admin  FROM role WHERE code='admin';
    SELECT id INTO v_ops    FROM role WHERE code='ops';
    SELECT id INTO v_audit  FROM role WHERE code='audit';
    SELECT id INTO v_viewer FROM role WHERE code='viewer';

    -- admin：全部业务权限 + 全部系统权限
    FOREACH r IN ARRAY biz_resources LOOP
        FOREACH p IN ARRAY actions LOOP
            INSERT INTO role_permission(role_id, permission_code) VALUES (v_admin, r||':'||p) ON CONFLICT DO NOTHING;
        END LOOP;
    END LOOP;
    FOREACH p IN ARRAY sys_perms LOOP
        INSERT INTO role_permission(role_id, permission_code) VALUES (v_admin, p) ON CONFLICT DO NOTHING;
    END LOOP;

    -- ops：业务资源全部 CRUD（含附件/字典），不含系统级
    FOREACH r IN ARRAY biz_resources LOOP
        FOREACH p IN ARRAY actions LOOP
            INSERT INTO role_permission(role_id, permission_code) VALUES (v_ops, r||':'||p) ON CONFLICT DO NOTHING;
        END LOOP;
    END LOOP;

    -- audit：全部业务资源只读 + 系统审计/变更查看
    FOREACH r IN ARRAY biz_resources LOOP
        INSERT INTO role_permission(role_id, permission_code) VALUES (v_audit, r||':read') ON CONFLICT DO NOTHING;
    END LOOP;
    FOREACH p IN ARRAY ARRAY['system:audit','system:change'] LOOP
        INSERT INTO role_permission(role_id, permission_code) VALUES (v_audit, p) ON CONFLICT DO NOTHING;
    END LOOP;

    -- viewer：全部业务资源只读
    FOREACH r IN ARRAY biz_resources LOOP
        INSERT INTO role_permission(role_id, permission_code) VALUES (v_viewer, r||':read') ON CONFLICT DO NOTHING;
    END LOOP;
END $$;

-- ============================================================
-- 6. 全表软删除统一（补齐 deleted_at）
-- ============================================================
ALTER TABLE cmdb_provider             ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE cmdb_data_center          ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE cmdb_server_ip            ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE cmdb_server_port          ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE cmdb_server_port_template ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE cmdb_backup_plan          ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE cmdb_monitor_target       ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE cmdb_attachment           ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE cmdb_contract             ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;
ALTER TABLE cmdb_dict                 ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMPTZ;

-- updated_at 触发器补齐（新增表 + dict）
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
    tables text[] := ARRAY['role','cmdb_dict','cmdb_provider','cmdb_data_center','cmdb_server_ip','cmdb_server_port','cmdb_server_port_template','cmdb_backup_plan','cmdb_monitor_target','cmdb_attachment','cmdb_contract'];
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
