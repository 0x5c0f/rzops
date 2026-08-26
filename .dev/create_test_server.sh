#!/bin/bash
TOKEN=$(cat /tmp/rzops_token.txt)
echo "--- 创建服务器 ---"
SVR=$(curl -s -X POST 'http://localhost:8000/api/v1/servers' \
  -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d '{"name":"e2e-test-srv","hostname":"e2e-test-srv.example.com","server_type":"physical","status":"running","os_type":"linux","os_version":"Ubuntu 22.04","cpu_cores":8,"memory_gb":32,"disk_gb":512,"remarks":"浏览器端到端测试服务器"}')
echo "$SVR" | head -c 400
echo ""
SVR_ID=$(echo "$SVR" | python3 -c 'import sys,json; print(json.load(sys.stdin)["id"])' 2>/dev/null)
echo "SVR_ID=$SVR_ID"
echo "$SVR_ID" > /tmp/rzops_svr_id.txt
echo "--- 创建IP ---"
curl -s -X POST 'http://localhost:8000/api/v1/server-ips' \
  -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d "{\"server_id\":\"$SVR_ID\",\"ip_address\":\"192.168.99.10\",\"ip_type\":\"ipv4\",\"is_primary\":true}" | head -c 200
echo ""
echo "--- 创建端口 ---"
curl -s -X POST 'http://localhost:8000/api/v1/server-ports' \
  -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d "{\"server_id\":\"$SVR_ID\",\"port\":22,\"protocol\":\"tcp\",\"service_name\":\"ssh\",\"is_enabled\":true}" | head -c 200
