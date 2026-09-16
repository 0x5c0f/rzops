#!/bin/bash
cd /mnt/c/workspace/RzOps
docker compose build api web 2>&1 | tail -1
docker compose up -d api web 2>&1 | tail -1
sleep 8
API="http://127.0.0.1:8000/api/v1"
TOKEN=$(curl -s -X POST "$API/auth/login" -H 'Content-Type: application/json' -d '{"email":"admin@rzops.local","password":"admin123"}' | python3 -c "import sys,json;print(json.load(sys.stdin)['access_token'])")
AUTH="Authorization: Bearer $TOKEN"

echo '=== 找一个未关联 IP 和一台服务器 ==='
IPID=$(curl -s "$API/server-ips?per_page=20&server_bound=false" -H "$AUTH" | python3 -c "import sys,json;print(json.load(sys.stdin)['data'][0]['id'])")
IPADDR=$(curl -s "$API/server-ips?per_page=20&server_bound=false" -H "$AUTH" | python3 -c "import sys,json;print(json.load(sys.stdin)['data'][0]['ip_address'])")
SRVID=$(curl -s "$API/servers?per_page=1" -H "$AUTH" | python3 -c "import sys,json;print(json.load(sys.stdin)['data'][0]['id'])")
echo "IP=$IPADDR($IPID) -> 目标服务器=$SRVID"

echo '=== 1) 改绑（传 server_id） ==='
curl -s -X PUT "$API/server-ips/$IPID" -H "$AUTH" -H 'Content-Type: application/json' -d "{\"server_id\":\"$SRVID\",\"description\":\"改绑测试\"}" | python3 -c "import sys,json;d=json.load(sys.stdin);print('server_id=',d.get('server_id'),'desc=',d.get('description'))"

echo '=== 2) 清空（传 null） ==='
curl -s -X PUT "$API/server-ips/$IPID" -H "$AUTH" -H 'Content-Type: application/json' -d '{"server_id":null,"description":"清空测试"}' | python3 -c "import sys,json;d=json.load(sys.stdin);print('server_id=',d.get('server_id'),'desc=',d.get('description'))"

echo '=== 3) 不传 server_id（应保持不变） ==='
curl -s -X PUT "$API/server-ips/$IPID" -H "$AUTH" -H 'Content-Type: application/json' -d '{"description":"不变测试"}' | python3 -c "import sys,json;d=json.load(sys.stdin);print('server_id=',d.get('server_id'),'desc=',d.get('description'))"

echo '=== 4) 再改绑恢复 ==='
curl -s -X PUT "$API/server-ips/$IPID" -H "$AUTH" -H 'Content-Type: application/json' -d "{\"server_id\":\"$SRVID\"}" | python3 -c "import sys,json;d=json.load(sys.stdin);print('server_id=',d.get('server_id'))"

echo '=== 5) 筛选计数回归 ==='
echo -n '已关联: '
curl -s "$API/server-ips?per_page=20&server_bound=true" -H "$AUTH" | python3 -c "import sys,json;print(json.load(sys.stdin)['count'])"
echo -n '未关联: '
curl -s "$API/server-ips?per_page=20&server_bound=false" -H "$AUTH" | python3 -c "import sys,json;print(json.load(sys.stdin)['count'])"
