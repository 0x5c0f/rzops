-- ============================================
-- 01_用户：运维团队（admin 已存在，新增 3 名运维人员）
-- 密码统一为 admin123（沿用 admin 哈希，仅测试用）
-- ============================================
SET client_encoding = 'UTF8';

INSERT INTO public."user" (id, email, hashed_password, is_active, is_superuser, full_name, created_at) VALUES
('00000000-0000-4000-8000-0000000000a1', 'zhang.wei@rzops.local', '$2b$12$MLFgEfcr3Ix96qrj4eyHR.4uRKLh42f8TiitMBHNjy6XkLnVrRENq', true, false, '张伟（系统运维）', '2025-03-12 02:00:00+00'),
('00000000-0000-4000-8000-0000000000a2', 'li.na@rzops.local',      '$2b$12$MLFgEfcr3Ix96qrj4eyHR.4uRKLh42f8TiitMBHNjy6XkLnVrRENq', true, false, '李娜（数据库DBA）', '2025-03-12 02:05:00+00'),
('00000000-0000-4000-8000-0000000000a3', 'wang.qiang@rzops.local', '$2b$12$MLFgEfcr3Ix96qrj4eyHR.4uRKLh42f8TiitMBHNjy6XkLnVrRENq', true, false, '王强（网络工程师）', '2025-03-12 02:10:00+00');
