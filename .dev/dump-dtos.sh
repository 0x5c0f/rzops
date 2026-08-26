#!/bin/sh
cd /mnt/c/workspace/RzOps/rzops-api/api/src/dto || exit 1
for f in provider_dto datacenter_dto server_dto server_ip_dto server_port_dto domain_dto certificate_dto database_instance_dto ops_site_dto credential_dto backup_plan_dto monitor_target_dto contract_dto attachment_dto site_relation_dto; do
  echo "=== $f ==="
  awk '/struct Create.*Request/,/^}/' "$f.rs" | grep 'pub ' | head -30
done
