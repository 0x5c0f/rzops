#!/bin/bash
TOKEN=$(curl -s -X POST http://localhost:8000/api/v1/auth/login -H 'Content-Type: application/json' -d '{"email":"admin@rzops.local","password":"admin123"}' | python3 -c 'import sys,json; d=json.load(sys.stdin); print(d.get("token") or d.get("access_token") or "")' 2>/dev/null)
echo "TOKEN_LEN=${#TOKEN}"
echo "$TOKEN" > /tmp/rzops_token.txt
echo "--- 变更记录筛选 ---"
curl -s "http://localhost:8000/api/v1/change-records?change_type=create&page=1&per_page=5" -H "Authorization: Bearer $TOKEN" | head -c 300
echo ""
echo "--- 字典分页 ---"
curl -s "http://localhost:8000/api/v1/dicts?page=1&per_page=5" -H "Authorization: Bearer $TOKEN" | head -c 300
echo ""
echo "--- 监控目标 site_id ---"
curl -s "http://localhost:8000/api/v1/monitor-targets?per_page=3" -H "Authorization: Bearer $TOKEN" | head -c 500
