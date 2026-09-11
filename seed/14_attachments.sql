-- ============================================
-- 14_附件（各资源关联文档，storage_key 为模拟路径）
-- ============================================
SET client_encoding = 'UTF8';

INSERT INTO public.cmdb_attachment
(id, filename, target_type, target_id, storage_key, content_type, size_bytes, uploaded_by_id,
 status, remarks, created_at, updated_at) VALUES
('d1000000-0000-4000-8000-000000000001', '服务器资产验收单-db-mysql-master.pdf', 'server',
 '30000000-0000-4000-8000-000000000004', 'server/2023/05/acceptance-db-mysql-master.pdf',
 'application/pdf', 5242880, '5a630c9c-5cee-4d37-9a7b-c7eecc614ffd', 'active',
 'Dell R740xd 到货验收单与序列号清单', '2023-05-20 03:00:00+00', '2023-05-20 03:00:00+00'),
('d1000000-0000-4000-8000-000000000002', '阿里云服务合同-HP-2025-001.pdf', 'contract',
 'c1000000-0000-4000-8000-000000000001', 'contract/2025/06/HT-2025-001.pdf',
 'application/pdf', 8388608, '5a630c9c-5cee-4d37-9a7b-c7eecc614ffd', 'active',
 '2025 年度云服务合同盖章扫描件', '2025-06-20 03:00:00+00', '2025-06-20 03:00:00+00'),
('d1000000-0000-4000-8000-000000000003', 'wildcard-raycloud-cn.pem', 'certificate',
 '90000000-0000-4000-8000-000000000001', 'certificate/2025/08/wildcard-raycloud-cn.pem',
 'application/x-x509-ca-cert', 16384, '00000000-0000-4000-8000-0000000000a1', 'active',
 '通配符证书 PEM 文件（测试数据，非真实密钥）', '2025-08-01 03:00:00+00', '2025-08-01 03:00:00+00'),
('d1000000-0000-4000-8000-000000000004', '官网ICP备案证明.pdf', 'site',
 '70000000-0000-4000-8000-000000000001', 'site/2023/03/icp-www-raycloud.pdf',
 'application/pdf', 1572864, '5a630c9c-5cee-4d37-9a7b-c7eecc614ffd', 'active',
 'www.raycloud.cn ICP 备案证明扫描件', '2023-03-05 03:00:00+00', '2023-03-05 03:00:00+00'),
('d1000000-0000-4000-8000-000000000005', '电信IDC机房拓扑图.png', 'data_center',
 '20000000-0000-4000-8000-000000000005', 'data_center/2023/05/dc-topology.png',
 'image/png', 4194304, '00000000-0000-4000-8000-0000000000a3', 'active',
 '重庆电信机房 3F 机柜与网络拓扑图', '2023-05-16 03:00:00+00', '2023-05-16 03:00:00+00'),
('d1000000-0000-4000-8000-000000000006', 'raycloud-cn-域名管理截图.png', 'domain',
 '80000000-0000-4000-8000-000000000001', 'domain/2025/06/raycloud-dns.png',
 'image/png', 1048576, '5a630c9c-5cee-4d37-9a7b-c7eecc614ffd', 'active',
 '西部数码后台域名解析配置截图', '2025-06-01 03:00:00+00', '2025-06-01 03:00:00+00');
