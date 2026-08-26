-- 数据字典表：替代硬编码枚举，维护各表单下拉选项（类型/状态/架构等）
CREATE TABLE IF NOT EXISTS cmdb_dict (
    id          UUID PRIMARY KEY,
    dict_type   VARCHAR(100) NOT NULL,          -- 分组键，如 server_type / server_status / architecture
    dict_code   VARCHAR(100) NOT NULL,          -- 存储到业务表的值，如 physical / active
    dict_label  VARCHAR(200) NOT NULL,          -- 展示文本，如 物理机 / 运行中
    sort_order  INT NOT NULL DEFAULT 0,         -- 排序，越小越靠前
    enabled     BOOLEAN NOT NULL DEFAULT TRUE,  -- 是否启用（停用=软删，历史数据 label 不丢）
    remark      VARCHAR(500),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT uq_cmdb_dict_type_code UNIQUE (dict_type, dict_code)
);

CREATE INDEX IF NOT EXISTS ix_cmdb_dict_type ON cmdb_dict (dict_type);
