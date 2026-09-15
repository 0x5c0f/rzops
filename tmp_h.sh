#!/bin/bash
cd /mnt/c/workspace/RzOps
echo '=== database_instance update handler ==='
grep -n -B2 -A30 'pub async fn update_database_instance' rzops-api/api/src/routes/database_instance_handlers.rs | head -45
echo '=== backup_plan update handler ==='
grep -n -B2 -A30 'pub async fn update_backup_plan' rzops-api/api/src/routes/backup_plan_handlers.rs | head -40
echo '=== monitor_target update handler ==='
grep -n -B2 -A30 'pub async fn update_monitor_target' rzops-api/api/src/routes/monitor_target_handlers.rs | head -40
