-- RzOps CMDB — Refactor: 字典化 / 冗余字段清理 / 凭据下线
-- 基于深度 review 后的结构调整

-- ──────────────────────────────────────────────
-- 1. 数据中心：移除 省份/城市（保留地址与国家）；线路类型改多选(JSONB)
-- ──────────────────────────────────────────────
ALTER TABLE cmdb_data_center DROP COLUMN IF EXISTS province;
ALTER TABLE cmdb_data_center DROP COLUMN IF EXISTS city;

-- line_type: VARCHAR → JSONB 数组（已有值兼容转为单元素数组）
ALTER TABLE cmdb_data_center ALTER COLUMN line_type TYPE JSONB
    USING CASE
        WHEN line_type IS NULL OR line_type = '' THEN '[]'::jsonb
        ELSE to_jsonb(string_to_array(line_type, ','))
    END;
ALTER TABLE cmdb_data_center ALTER COLUMN line_type SET DEFAULT '[]'::jsonb;

-- ──────────────────────────────────────────────
-- 2. 供应商：移除 国家（与地址冗余）
-- ──────────────────────────────────────────────
ALTER TABLE cmdb_provider DROP COLUMN IF EXISTS country;

-- ──────────────────────────────────────────────
-- 3. 域名：移除无实体的预留关联列；新增注册时间
-- ──────────────────────────────────────────────
ALTER TABLE cmdb_domain DROP COLUMN IF EXISTS business_unit_id;
ALTER TABLE cmdb_domain DROP COLUMN IF EXISTS company_id;
ALTER TABLE cmdb_domain DROP COLUMN IF EXISTS account_credential_id;
ALTER TABLE cmdb_domain ADD COLUMN IF NOT EXISTS registered_date DATE;

-- ──────────────────────────────────────────────
-- 4. 证书：移除密钥凭证引用（私钥由证书/密钥管理系统保管）
-- ──────────────────────────────────────────────
ALTER TABLE cmdb_certificate DROP COLUMN IF EXISTS private_key_credential_id;

-- ──────────────────────────────────────────────
-- 5. 数据库实例：移除管理凭证引用
-- ──────────────────────────────────────────────
ALTER TABLE cmdb_database_instance DROP COLUMN IF EXISTS management_credential_id;

-- ──────────────────────────────────────────────
-- 6. 服务器IP：server 改为可空（如 EIP 可解绑不关联服务器）
-- ──────────────────────────────────────────────
ALTER TABLE cmdb_server_ip ALTER COLUMN server_id DROP NOT NULL;

-- ──────────────────────────────────────────────
-- 7. 站点：移除 内部系统 / 使用CDN（意义有限）
-- ──────────────────────────────────────────────
ALTER TABLE cmdb_ops_site DROP COLUMN IF EXISTS is_internal_system;
ALTER TABLE cmdb_ops_site DROP COLUMN IF EXISTS uses_cdn;

-- ──────────────────────────────────────────────
-- 8. 凭据模块整体下线（含上述引用清理）
-- ──────────────────────────────────────────────
DROP TABLE IF EXISTS cmdb_credential;

-- ──────────────────────────────────────────────
-- 9. 字典种子数据：币种 / 国家（供 FormSelect 使用）
-- ──────────────────────────────────────────────
INSERT INTO cmdb_dict (id, dict_type, dict_code, dict_label, sort_order, enabled)
SELECT gen_random_uuid(), 'currency', c.code, c.label, c.sort, true
FROM (VALUES
    ('CNY', '人民币 CNY', 1),
    ('USD', '美元 USD', 2),
    ('HKD', '港币 HKD', 3),
    ('EUR', '欧元 EUR', 4),
    ('JPY', '日元 JPY', 5),
    ('GBP', '英镑 GBP', 6),
    ('SGD', '新加坡元 SGD', 7),
    ('AUD', '澳元 AUD', 8),
    ('CNH', '离岸人民币 CNH', 9)
) AS c(code, label, sort)
WHERE NOT EXISTS (SELECT 1 FROM cmdb_dict d WHERE d.dict_type='currency' AND d.dict_code=c.code);

INSERT INTO cmdb_dict (id, dict_type, dict_code, dict_label, sort_order, enabled)
SELECT gen_random_uuid(), 'country', c.code, c.label, c.sort, true
FROM (VALUES
    ('CN', '中国', 1),
    ('US', '美国', 2),
    ('JP', '日本', 3),
    ('DE', '德国', 4),
    ('GB', '英国', 5),
    ('SG', '新加坡', 6),
    ('HK', '中国香港', 7),
    ('TW', '中国台湾', 8),
    ('KR', '韩国', 9),
    ('AU', '澳大利亚', 10),
    ('NL', '荷兰', 11),
    ('FR', '法国', 12)
) AS c(code, label, sort)
WHERE NOT EXISTS (SELECT 1 FROM cmdb_dict d WHERE d.dict_type='country' AND d.dict_code=c.code);
