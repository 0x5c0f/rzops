-- ============================================
-- 17_字典调整：监控目标范围收窄 + 备份目标类型
-- 1) asset_target_type 移除 provider / data_center（监控目标范围=服务器/数据库/站点/域名/证书）
-- 2) 新增 backup_target_type（备份目标范围=数据库/服务器/站点）
-- ============================================
SET client_encoding = 'UTF8';

-- 监控目标类型收窄
DELETE FROM public.cmdb_dict WHERE dict_type = 'asset_target_type' AND dict_code IN ('provider', 'data_center');

-- 备份目标类型
INSERT INTO public.cmdb_dict (id, dict_type, dict_code, dict_label, sort_order, enabled)
VALUES ('a2000000-0000-4000-8000-000000000001', 'backup_target_type', 'database', '数据库实例', 1, true)
ON CONFLICT (dict_type, dict_code) DO NOTHING;
INSERT INTO public.cmdb_dict (id, dict_type, dict_code, dict_label, sort_order, enabled)
VALUES ('a2000000-0000-4000-8000-000000000002', 'backup_target_type', 'server', '服务器', 2, true)
ON CONFLICT (dict_type, dict_code) DO NOTHING;
INSERT INTO public.cmdb_dict (id, dict_type, dict_code, dict_label, sort_order, enabled)
VALUES ('a2000000-0000-4000-8000-000000000003', 'backup_target_type', 'site', '站点', 3, true)
ON CONFLICT (dict_type, dict_code) DO NOTHING;
