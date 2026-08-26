#!/usr/bin/env bash
# 验证审计日志中间件：登录 → 写操作 → 查审计
set -e
API=http://localhost:8000/api/v1

TOKEN=$(curl -s -X POST "$API/auth/login" -H 'Content-Type: application/json' \
  -d '{"email":"admin@rzops.local","password":"admin123"}' \
  | python3 -c 'import sys,json; print(json.load(sys.stdin)["access_token"])')

echo "TOKEN_LEN=${#TOKEN}"

# 1) 更新服务器（PUT）
echo '--- PUT server ---'
curl -s -X PUT "$API/servers/9c5a4dfd-061b-4b44-958e-0e66dc0494d2" \
  -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d '{"memory_gb":257}' | head -c 120; echo

# 2) 创建一条凭据（POST）
echo '--- POST credential ---'
curl -s -X POST "$API/credentials" \
  -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d '{"name":"audit-test-cred","credential_type":"ssh_key","username":"u","secret_ref":"v"}' \
  | head -c 120; echo

# 3) 查审计日志
echo '--- audit-logs (latest 6) ---'
curl -s "$API/audit-logs?limit=6" -H "Authorization: Bearer $TOKEN" \
  | python3 -c '
import sys, json
d = json.load(sys.stdin)
items = d if isinstance(d, list) else d.get("items", d.get("data", []))
print("total:", len(items))
for i in items:
    print(i.get("action"), "|", i.get("resource_type"), "|", str(i.get("resource_id"))[:8], "| actor", str(i.get("actor_id"))[:8], "|", i.get("created_at"))
'
