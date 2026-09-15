#!/bin/bash
cd /mnt/c/workspace/RzOps
echo '=== server_ip handler update 完整（模板） ==='
sed -n '157,230p' rzops-api/api/src/routes/server_ip_handlers.rs
echo '=== route 注册（server_ip） ==='
grep -rn 'server-ips' rzops-api/api/src/routes/mod.rs rzops-api/server/src/router.rs 2>/dev/null | head -10
