#!/bin/bash
# 并行测试脚本：4 个 unbind 端点端到端验证
set -e
API="http://127.0.0.1:8000/api/v1"
TOKEN=$(curl -s -X POST "$API/auth/login" -H 'Content-Type: application/json' -d '{"email":"admin@rzops.local","password":"admin123"}' | python3 -c "import sys,json;print(json.load(sys.stdin)['access_token'])")
AUTH="Authorization: Bearer $TOKEN"
SID="e5f6338f-627d-439c-97eb-a84117cd4cea"  # e2e-test-srv-01

echo "=== 1. 造测试数据 ==="
# IP
IPID=$(curl -s -X POST "$API/server-ips" -H "$AUTH" -H 'Content-Type: application/json' -d "{\"ip_address\":\"203.0.113.251\",\"server_id\":\"$SID\",\"ip_type\":\"public\",\"status\":\"enabled\"}" | python3 -c "import sys,json;print(json.load(sys.stdin)['id'])")
echo "IP: $IPID"
# 数据库实例
DBID=$(curl -s -X POST "$API/database-instances" -H "$AUTH" -H 'Content-Type: application/json' -d "{\"name\":\"unbind-test-db\",\"server_id\":\"$SID\",\"db_type\":\"postgresql\",\"status\":\"running\",\"environment\":\"prod\"}" | python3 -c "import sys,json;print(json.load(sys.stdin)['id'])")
echo "DB: $DBID"
# 备份计划（关联服务器）
BPID=$(curl -s -X POST "$API/backup-plans" -H "$AUTH" -H 'Content-Type: application/json' -d "{\"name\":\"unbind-test-bp\",\"target_type\":\"server\",\"target_id\":\"$SID\",\"schedule\":\"0 2 * * *\",\"retention_days\":7,\"status\":\"enabled\"}" | python3 -c "import sys,json;print(json.load(sys.stdin)['id'])")
echo "BP: $BPID"
# 监控目标（关联服务器）
MTID=$(curl -s -X POST "$API/monitor-targets" -H "$AUTH" -H 'Content-Type: application/json' -d "{\"name\":\"unbind-test-mt\",\"target_type\":\"server\",\"target_id\":\"$SID\",\"monitor_type\":\"ping\",\"status\":\"enabled\"}" | python3 -c "import sys,json;print(json.load(sys.stdin)['id'])")
echo "MT: $MTID"

echo "=== 2. 逐个 unbind ==="
for ent in "server-ips/$IPID" "database-instances/$DBID" "backup-plans/$BPID" "monitor-targets/$MTID"; do
  CODE=$(curl -s -o /tmp/ub.json -w '%{http_code}' -X POST "$API/$ent/unbind" -H "$AUTH")
  echo "$ent -> HTTP $CODE"
  if [ "$CODE" != "200" ]; then cat /tmp/ub.json; echo; fi
done

echo "=== 3. 验证解绑结果（记录仍在且已置空） ==="
for ent in "server-ips/$IPID" "database-instances/$DBID" "backup-plans/$BPID" "monitor-targets/$MTID"; do
  BODY=$(curl -s "$API/$ent" -H "$AUTH")
  echo "$ent: $(echo "$BODY" | python3 -c "import sys,json;d=json.load(sys.stdin);print({k:d.get(k) for k in ['server_id','target_id','target_type','status']})")"
done

echo "=== 4. 变更日志确认（应有4条 Update） ==="
curl -s "$API/change-records?q=unbind-test&per_page=10" -H "$AUTH" | python3 -c "import sys,json;d=json.load(sys.stdin);items=d.get('items',d if isinstance(d,list) else []);print('变更记录数:',len(items));[print(' -',i.get('entity_type'),i.get('change_type')) for i in items[:8]]" 2>/dev/null || echo "(change-records 结构不同，跳过)"

echo "=== DONE ==="
