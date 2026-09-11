-- 007: 服务器IP增加网卡名称字段
ALTER TABLE cmdb_server_ip ADD COLUMN IF NOT EXISTS nic_name VARCHAR(100);
COMMENT ON COLUMN cmdb_server_ip.nic_name IS '网卡名称，如 eth0 / ens33 / 内网网卡';
