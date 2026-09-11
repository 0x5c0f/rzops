SET client_encoding = 'UTF8';
\echo '=== 各表数据量 ==='
SELECT 'provider' AS t, count(*) AS n FROM cmdb_provider
UNION ALL SELECT 'data_center', count(*) FROM cmdb_data_center
UNION ALL SELECT 'server', count(*) FROM cmdb_server
UNION ALL SELECT 'server_ip', count(*) FROM cmdb_server_ip
UNION ALL SELECT 'server_port', count(*) FROM cmdb_server_port
UNION ALL SELECT 'database_instance', count(*) FROM cmdb_database_instance
UNION ALL SELECT 'ops_site', count(*) FROM cmdb_ops_site
UNION ALL SELECT 'domain', count(*) FROM cmdb_domain
UNION ALL SELECT 'certificate', count(*) FROM cmdb_certificate
UNION ALL SELECT 'certificate_domain', count(*) FROM cmdb_certificate_domain
UNION ALL SELECT 'monitor_target', count(*) FROM cmdb_monitor_target
UNION ALL SELECT 'backup_plan', count(*) FROM cmdb_backup_plan
UNION ALL SELECT 'contract', count(*) FROM cmdb_contract
UNION ALL SELECT 'attachment', count(*) FROM cmdb_attachment
UNION ALL SELECT 'site_server', count(*) FROM cmdb_ops_site_server
UNION ALL SELECT 'site_database', count(*) FROM cmdb_ops_site_database
UNION ALL SELECT 'site_domain', count(*) FROM cmdb_ops_site_domain
UNION ALL SELECT 'change_record', count(*) FROM cmdb_change_record
UNION ALL SELECT 'audit_log', count(*) FROM cmdb_audit_log
UNION ALL SELECT 'user', count(*) FROM "user"
ORDER BY t;

\echo '=== 悬挂外键检查（应全为 0）==='
SELECT 'server->provider' chk, count(*) FROM cmdb_server s LEFT JOIN cmdb_provider p ON s.server_provider_id=p.id WHERE s.server_provider_id IS NOT NULL AND p.id IS NULL
UNION ALL SELECT 'server->dc', count(*) FROM cmdb_server s LEFT JOIN cmdb_data_center d ON s.data_center_id=d.id WHERE s.data_center_id IS NOT NULL AND d.id IS NULL
UNION ALL SELECT 'db->server', count(*) FROM cmdb_database_instance x LEFT JOIN cmdb_server s ON x.server_id=s.id WHERE x.server_id IS NOT NULL AND s.id IS NULL
UNION ALL SELECT 'db->backup', count(*) FROM cmdb_database_instance x LEFT JOIN cmdb_backup_plan b ON x.backup_plan_id=b.id WHERE x.backup_plan_id IS NOT NULL AND b.id IS NULL
UNION ALL SELECT 'db->monitor', count(*) FROM cmdb_database_instance x LEFT JOIN cmdb_monitor_target m ON x.monitor_target_id=m.id WHERE x.monitor_target_id IS NOT NULL AND m.id IS NULL
UNION ALL SELECT 'site->backup', count(*) FROM cmdb_ops_site s LEFT JOIN cmdb_backup_plan b ON s.backup_plan_id=b.id WHERE s.backup_plan_id IS NOT NULL AND b.id IS NULL
UNION ALL SELECT 'site->monitor', count(*) FROM cmdb_ops_site s LEFT JOIN cmdb_monitor_target m ON s.monitor_target_id=m.id WHERE s.monitor_target_id IS NOT NULL AND m.id IS NULL
UNION ALL SELECT 'cert->provider', count(*) FROM cmdb_certificate c LEFT JOIN cmdb_provider p ON c.provider_id=p.id WHERE c.provider_id IS NOT NULL AND p.id IS NULL
UNION ALL SELECT 'ip->server', count(*) FROM cmdb_server_ip i LEFT JOIN cmdb_server s ON i.server_id=s.id WHERE i.server_id IS NOT NULL AND s.id IS NULL
UNION ALL SELECT 'port_server->port', count(*) FROM cmdb_server_port_server x LEFT JOIN cmdb_server_port p ON x.server_port_id=p.id WHERE p.id IS NULL
UNION ALL SELECT 'port_server->server', count(*) FROM cmdb_server_port_server x LEFT JOIN cmdb_server s ON x.server_id=s.id WHERE s.id IS NULL
UNION ALL SELECT 'site_server->site', count(*) FROM cmdb_ops_site_server x LEFT JOIN cmdb_ops_site s ON x.site_id=s.id WHERE s.id IS NULL
UNION ALL SELECT 'site_server->server', count(*) FROM cmdb_ops_site_server x LEFT JOIN cmdb_server s ON x.server_id=s.id WHERE s.id IS NULL
UNION ALL SELECT 'site_db->site', count(*) FROM cmdb_ops_site_database x LEFT JOIN cmdb_ops_site s ON x.site_id=s.id WHERE s.id IS NULL
UNION ALL SELECT 'site_db->db', count(*) FROM cmdb_ops_site_database x LEFT JOIN cmdb_database_instance d ON x.database_instance_id=d.id WHERE d.id IS NULL
UNION ALL SELECT 'site_domain->site', count(*) FROM cmdb_ops_site_domain x LEFT JOIN cmdb_ops_site s ON x.site_id=s.id WHERE s.id IS NULL
UNION ALL SELECT 'site_domain->domain', count(*) FROM cmdb_ops_site_domain x LEFT JOIN cmdb_domain d ON x.domain_id=d.id WHERE d.id IS NULL
UNION ALL SELECT 'cert_domain->cert', count(*) FROM cmdb_certificate_domain x LEFT JOIN cmdb_certificate c ON x.certificate_id=c.id WHERE c.id IS NULL
UNION ALL SELECT 'cert_domain->domain', count(*) FROM cmdb_certificate_domain x LEFT JOIN cmdb_domain d ON x.domain_id=d.id WHERE x.domain_id IS NOT NULL AND d.id IS NULL
UNION ALL SELECT 'att->user', count(*) FROM cmdb_attachment a LEFT JOIN "user" u ON a.uploaded_by_id=u.id WHERE u.id IS NULL
UNION ALL SELECT 'change->user', count(*) FROM cmdb_change_record c LEFT JOIN "user" u ON c.actor_id=u.id WHERE c.actor_id IS NOT NULL AND u.id IS NULL
UNION ALL SELECT 'audit->user', count(*) FROM cmdb_audit_log a LEFT JOIN "user" u ON a.actor_id=u.id WHERE a.actor_id IS NOT NULL AND u.id IS NULL;
