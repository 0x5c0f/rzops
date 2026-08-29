SELECT 'cmdb_dict' AS t, count(*) AS n FROM cmdb_dict
UNION ALL SELECT 'cmdb_provider', count(*) FROM cmdb_provider
UNION ALL SELECT 'cmdb_data_center', count(*) FROM cmdb_data_center
UNION ALL SELECT 'cmdb_server', count(*) FROM cmdb_server
UNION ALL SELECT 'cmdb_server_ip', count(*) FROM cmdb_server_ip
UNION ALL SELECT 'cmdb_server_port', count(*) FROM cmdb_server_port
UNION ALL SELECT 'cmdb_server_port_server', count(*) FROM cmdb_server_port_server
UNION ALL SELECT 'cmdb_database_instance', count(*) FROM cmdb_database_instance
UNION ALL SELECT 'cmdb_ops_site', count(*) FROM cmdb_ops_site
UNION ALL SELECT 'cmdb_domain', count(*) FROM cmdb_domain
UNION ALL SELECT 'cmdb_certificate', count(*) FROM cmdb_certificate
UNION ALL SELECT 'cmdb_monitor_target', count(*) FROM cmdb_monitor_target
UNION ALL SELECT 'cmdb_backup_plan', count(*) FROM cmdb_backup_plan
UNION ALL SELECT 'cmdb_attachment', count(*) FROM cmdb_attachment
UNION ALL SELECT 'cmdb_contract', count(*) FROM cmdb_contract
UNION ALL SELECT 'cmdb_change_record', count(*) FROM cmdb_change_record
UNION ALL SELECT 'cmdb_audit_log', count(*) FROM cmdb_audit_log
UNION ALL SELECT 'user', count(*) FROM "user"
ORDER BY t;

SELECT dict_type, count(*) FROM cmdb_dict GROUP BY dict_type ORDER BY dict_type;
