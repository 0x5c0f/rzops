#!/bin/bash
cd /mnt/c/workspace/RzOps
R=rzops-api/infra/src/db/repositories
echo '=== server_ip_repo 头部 ==='
sed -n '1,30p' $R/server_ip_repo.rs
echo '=== server_ip delete 后（row_to 函数） ==='
grep -n 'fn row_to\|fn to_\|SELECT_COLS\|RETURNING' $R/server_ip_repo.rs | head -8
echo '=== database_instance SELECT_COLS + row_to ==='
grep -n 'SELECT_COLS\|fn row_to' $R/database_instance_repo.rs | head -5
echo '=== backup_plan COLS + row_to ==='
grep -n 'const COLS\|fn row_to' $R/backup_plan_repo.rs | head -5
echo '=== monitor_target COLS + row_to ==='
grep -n 'const COLS\|fn row_to' $R/monitor_target_repo.rs | head -5
echo '=== domain trait 位置 ==='
find rzops-api/domain/src -name '*repository*.rs' | head -8
