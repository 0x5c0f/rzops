#!/bin/bash
cd /mnt/c/workspace/RzOps
echo '=== server_ip_repo update SQL ==='
grep -rn -B3 -A15 'async fn update' rzops-api/infra/src/db/server_ip_repo.rs 2>/dev/null | head -30
echo '=== 找其他 repo 文件 ==='
find rzops-api/infra/src -name '*repo*.rs' | head -20
echo '=== database_instance update SQL ==='
grep -rn -B2 -A12 'SET server_id\|server_id = ' rzops-api/infra/src/db/database_instance_repo.rs 2>/dev/null | head -20
