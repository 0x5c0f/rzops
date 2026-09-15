#!/bin/bash
cd /mnt/c/workspace/RzOps
R=rzops-api/infra/src/db/repositories
echo '=== server_ip_repo update ==='
grep -n -B2 -A18 'async fn update' $R/server_ip_repo.rs | head -35
echo '=== database_instance_repo update ==='
grep -n -B2 -A18 'async fn update' $R/database_instance_repo.rs | head -35
echo '=== backup_plan_repo update ==='
grep -n -B2 -A18 'async fn update' $R/backup_plan_repo.rs | head -35
echo '=== monitor_target_repo update ==='
grep -n -B2 -A18 'async fn update' $R/monitor_target_repo.rs | head -35
