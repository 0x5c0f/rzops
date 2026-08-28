#!/bin/bash
cd /mnt/c/workspace/RzOps/seed || exit 1
for f in 01_users 02_providers 03_data_centers 04_servers 05_server_ips 06_server_ports 07_database_instances 08_sites 09_domains 10_certificates 11_monitor_targets 12_backup_plans 13_contracts 14_attachments 15_relations 16_logs; do
  echo "=== $f ==="
  docker exec -i rzops-postgres psql -U rzops -d rzopsdb -v ON_ERROR_STOP=1 < /mnt/c/workspace/RzOps/seed/${f}.sql 2>&1 | grep -iE "INSERT 0|ERROR|does not exist|violat" | head -3
done
