#!/bin/bash
cd /mnt/c/workspace/RzOps
echo '=== 4 个 handler 文件位置 ==='
ls rzops-api/api/src/routes/ | grep -E 'server_ip|database_instance|backup_plan|monitor_target'
echo '=== server_ip repo update 方法 ==='
grep -n -A12 'async fn update' rzops-api/domain/src/ports/server_ip_repository.rs 2>/dev/null | head -15
grep -n -A12 'async fn update' rzops-api/infra/src/db/server_ip_repo.rs 2>/dev/null | head -20
echo '=== database_instance repo update ==='
grep -n -A10 'async fn update' rzops-api/infra/src/db/database_instance_repo.rs 2>/dev/null | head -18
echo '=== backup_plan repo update ==='
grep -n -A10 'async fn update' rzops-api/infra/src/db/backup_plan_repo.rs 2>/dev/null | head -18
echo '=== monitor_target repo update ==='
grep -n -A10 'async fn update' rzops-api/infra/src/db/monitor_target_repo.rs 2>/dev/null | head -18
