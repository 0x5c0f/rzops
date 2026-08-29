-- ============================================================================
-- 005: 服务器端口改为多对多（一个端口定义可关联多台服务器）
-- ----------------------------------------------------------------------------
-- 原设计：cmdb_server_port.server_id 单服务器（每台服务器一条记录）
-- 新设计：端口定义 (protocol, port, service_name) 唯一，多台服务器通过
--         cmdb_server_port_server 关联表共享同一端口定义，减少重复数据量。
-- 迁移：按 (protocol, port, service_name) 分组合并现有记录，保留每组最早一条
--       为端口定义，其余行退化为关联。
-- ============================================================================

-- 1. 关联表
CREATE TABLE cmdb_server_port_server (
    server_port_id UUID NOT NULL REFERENCES cmdb_server_port(id) ON DELETE CASCADE,
    server_id      UUID NOT NULL REFERENCES cmdb_server(id) ON DELETE CASCADE,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (server_port_id, server_id)
);

-- 2. 迁移现有数据到关联表（每组 protocol+port+service_name 保留最早一条为端口定义，其余 server 关联过去）
INSERT INTO cmdb_server_port_server (server_port_id, server_id)
SELECT g.rep_id, p.server_id
FROM cmdb_server_port p
JOIN (
    SELECT DISTINCT ON (protocol, port, service_name) protocol, port, service_name, id AS rep_id
    FROM cmdb_server_port
    ORDER BY protocol, port, service_name, created_at ASC
) g ON g.protocol = p.protocol AND g.port = p.port AND g.service_name = p.service_name;

-- 3. 删除非代表行（其 server 关联已迁移到代表行，可安全删除）
DELETE FROM cmdb_server_port
WHERE id NOT IN (SELECT DISTINCT server_port_id FROM cmdb_server_port_server);

-- 4. 移除单服务器外键列与旧唯一约束，加新的端口定义唯一约束
ALTER TABLE cmdb_server_port DROP CONSTRAINT uq_cmdb_server_port_server_protocol_port;
ALTER TABLE cmdb_server_port DROP COLUMN server_id;
ALTER TABLE cmdb_server_port ADD CONSTRAINT uq_cmdb_server_port_protocol_port_service UNIQUE (protocol, port, service_name);

-- 5. 关联表索引
CREATE INDEX ix_cmdb_server_port_server_port_id ON cmdb_server_port_server (server_port_id);
CREATE INDEX ix_cmdb_server_port_server_server_id ON cmdb_server_port_server (server_id);
