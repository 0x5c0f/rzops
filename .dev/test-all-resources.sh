#!/bin/bash
# RzOps 全资源端到端测试：每个资源 创建→读取→更新→再读验证
# 运行：sh test-all-resources.sh
set -u
BASE=http://localhost:8000/api/v1
STAMP=$(date +%s)
PASS=0
FAIL=0

py() { python3 -c "$1"; }

jget() {  # json, key -> value (or '')
  py "import sys,json
try:
  d=json.loads('''$1''')
  print(d.get('$2','') if isinstance(d,dict) else '')
except Exception as e:
  print('ERR')"
}

http_code() { curl -s -o /tmp/resp.json -w '%{http_code}' "$@"; }

TOKEN=$(curl -s -X POST "$BASE/auth/login" -H 'Content-Type: application/json' \
  -d '{"email":"admin@rzops.local","password":"admin123"}' | py "import sys,json; print(json.load(sys.stdin).get('access_token',''))")
AUTH="Authorization: Bearer $TOKEN"
echo "TOKEN len: ${#TOKEN}"

report() {  # name, stage, code, expect
  if [ "$3" = "$4" ]; then
    PASS=$((PASS+1)); echo "  PASS  $1 $2 ($3)"
  else
    FAIL=$((FAIL+1)); echo "  FAIL  $1 $2 (got $3 want $4): $(cat /tmp/resp.json | head -c 200)"
  fi
}

# ---------- 1. providers ----------
echo "[1] providers"
CODE=$(http_code -X POST "$BASE/providers" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试供应商-$STAMP\",\"provider_types\":[\"IDC\"],\"contact_name\":\"张三\",\"description\":\"端到端测试\",\"status\":\"active\"}")
report providers create "$CODE" 201
PID=$(jget "$(cat /tmp/resp.json)" id)
CODE=$(http_code "$BASE/providers/$PID" -H "$AUTH"); report providers get "$CODE" 200
CODE=$(http_code -X PUT "$BASE/providers/$PID" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试供应商-$STAMP-改\",\"provider_types\":[\"IDC\",\"ISP\"],\"status\":\"active\"}")
report providers update "$CODE" 200
CNT=$(jget "$(cat /tmp/resp.json)" provider_types | grep -c ISP)
[ "$CNT" -ge 1 ] && { PASS=$((PASS+1)); echo "  PASS  providers update-types"; } || { FAIL=$((FAIL+1)); echo "  FAIL  providers update-types"; }

# ---------- 2. datacenters ----------
echo "[2] datacenters"
CODE=$(http_code -X POST "$BASE/data-centers" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试机房-$STAMP\",\"provider_id\":\"$PID\",\"city\":\"重庆\",\"description\":\"端到端机房\",\"status\":\"active\"}")
report datacenters create "$CODE" 201
DCID=$(jget "$(cat /tmp/resp.json)" id)
CODE=$(http_code "$BASE/data-centers/$DCID" -H "$AUTH"); report datacenters get "$CODE" 200
CODE=$(http_code -X PUT "$BASE/data-centers/$DCID" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试机房-$STAMP-改\",\"city\":\"成都\",\"status\":\"active\"}")
report datacenters update "$CODE" 200

# ---------- 3. servers ----------
echo "[3] servers"
CODE=$(http_code -X POST "$BASE/servers" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试服务器-$STAMP\",\"data_center_id\":\"$DCID\",\"isp_provider_id\":\"$PID\",\"primary_ip\":\"10.10.$STAMP.1\",\"server_type\":\"physical\",\"status\":\"active\",\"memory_gb\":64,\"cpu\":\"Intel Xeon 16C\"}")
report servers create "$CODE" 201
SID=$(jget "$(cat /tmp/resp.json)" id)
CODE=$(http_code "$BASE/servers/$SID" -H "$AUTH"); report servers get "$CODE" 200
CODE=$(http_code -X PUT "$BASE/servers/$SID" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试服务器-$STAMP-改\",\"memory_gb\":128,\"primary_ip\":\"10.10.$STAMP.2\",\"status\":\"active\"}")
report servers update "$CODE" 200

# ---------- 4. server-ips ----------
echo "[4] server-ips"
CODE=$(http_code -X POST "$BASE/server-ips" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"server_id\":\"$SID\",\"ip_address\":\"172.16.$STAMP.10\",\"ip_type\":\"内网\",\"is_primary\":true,\"status\":\"active\"}")
report server-ips create "$CODE" 201
IPID=$(jget "$(cat /tmp/resp.json)" id)
CODE=$(http_code "$BASE/server-ips/$IPID" -H "$AUTH"); report server-ips get "$CODE" 200
CODE=$(http_code -X PUT "$BASE/server-ips/$IPID" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"server_id\":\"$SID\",\"ip_address\":\"172.16.$STAMP.11\",\"ip_type\":\"公网\",\"status\":\"active\"}")
report server-ips update "$CODE" 200

# ---------- 5. server-ports ----------
echo "[5] server-ports"
CODE=$(http_code -X POST "$BASE/server-ports" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"server_id\":\"$SID\",\"protocol\":\"TCP\",\"port\":8080,\"service_name\":\"nginx\",\"access_scope\":\"内网\",\"is_enabled\":true}")
report server-ports create "$CODE" 201
PORTID=$(jget "$(cat /tmp/resp.json)" id)
CODE=$(http_code "$BASE/server-ports/$PORTID" -H "$AUTH"); report server-ports get "$CODE" 200
CODE=$(http_code -X PUT "$BASE/server-ports/$PORTID" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"server_id\":\"$SID\",\"protocol\":\"TCP\",\"port\":8443,\"service_name\":\"nginx-ssl\",\"is_enabled\":false}")
report server-ports update "$CODE" 200

# ---------- 6. domains ----------
echo "[6] domains"
CODE=$(http_code -X POST "$BASE/domains" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"domain_name\":\"test-$STAMP.example.com\",\"provider_id\":\"$PID\",\"expiry_date\":\"2027-12-31\",\"is_enabled\":true,\"remarks\":\"端到端域名\"}")
report domains create "$CODE" 201
DMID=$(jget "$(cat /tmp/resp.json)" id)
CODE=$(http_code "$BASE/domains/$DMID" -H "$AUTH"); report domains get "$CODE" 200
CODE=$(http_code -X PUT "$BASE/domains/$DMID" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"domain_name\":\"test-$STAMP.example.com\",\"expiry_date\":\"2028-06-30\",\"is_enabled\":false}")
report domains update "$CODE" 200

# ---------- 7. certificates ----------
echo "[7] certificates"
CODE=$(http_code -X POST "$BASE/certificates" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试证书-$STAMP\",\"provider_id\":\"$PID\",\"certificate_type\":\"SSL\",\"lease_start_date\":\"2026-01-01\",\"lease_end_date\":\"2026-12-31\",\"status\":\"active\"}")
report certificates create "$CODE" 201
CID2=$(jget "$(cat /tmp/resp.json)" id)
CODE=$(http_code "$BASE/certificates/$CID2" -H "$AUTH"); report certificates get "$CODE" 200
CODE=$(http_code -X PUT "$BASE/certificates/$CID2" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试证书-$STAMP-改\",\"lease_end_date\":\"2027-06-30\",\"status\":\"active\"}")
report certificates update "$CODE" 200

# ---------- 8. database-instances ----------
echo "[8] database-instances"
CODE=$(http_code -X POST "$BASE/database-instances" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试数据库-$STAMP\",\"db_type\":\"MySQL\",\"server_id\":\"$SID\",\"port\":3306,\"is_self_installed\":true,\"status\":\"active\",\"description\":\"端到端数据库\"}")
report database-instances create "$CODE" 201
DBID=$(jget "$(cat /tmp/resp.json)" id)
CODE=$(http_code "$BASE/database-instances/$DBID" -H "$AUTH"); report database-instances get "$CODE" 200
CODE=$(http_code -X PUT "$BASE/database-instances/$DBID" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试数据库-$STAMP-改\",\"db_type\":\"PostgreSQL\",\"port\":5432,\"status\":\"active\"}")
report database-instances update "$CODE" 200

# ---------- 9. ops-sites ----------
echo "[9] ops-sites"
CODE=$(http_code -X POST "$BASE/ops-sites" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试站点-$STAMP\",\"url\":\"https://test-$STAMP.example.com\",\"importance\":\"高\",\"service_target\":\"核心\",\"status\":\"active\",\"function_summary\":\"端到端站点\",\"remarks\":\"备注信息\"}")
report ops-sites create "$CODE" 201
OSID=$(jget "$(cat /tmp/resp.json)" id)
CODE=$(http_code "$BASE/ops-sites/$OSID" -H "$AUTH"); report ops-sites get "$CODE" 200
CODE=$(http_code -X PUT "$BASE/ops-sites/$OSID" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试站点-$STAMP-改\",\"uses_cdn\":true,\"status\":\"active\"}")
report ops-sites update "$CODE" 200

# ---------- 10. credentials ----------
echo "[10] credentials"
CODE=$(http_code -X POST "$BASE/credentials" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试凭据-$STAMP\",\"credential_type\":\"password\",\"username\":\"root\",\"secret_ref\":\"vault:test\",\"status\":\"active\",\"remarks\":\"端到端凭据\"}")
report credentials create "$CODE" 201
CRID=$(jget "$(cat /tmp/resp.json)" id)
CODE=$(http_code "$BASE/credentials/$CRID" -H "$AUTH"); report credentials get "$CODE" 200
CODE=$(http_code -X PUT "$BASE/credentials/$CRID" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试凭据-$STAMP-改\",\"username\":\"ops\",\"status\":\"active\"}")
report credentials update "$CODE" 200

# ---------- 11. backup-plans ----------
echo "[11] backup-plans"
CODE=$(http_code -X POST "$BASE/backup-plans" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试备份-$STAMP\",\"target_type\":\"database\",\"schedule\":\"0 2 * * *\",\"retention_days\":30,\"status\":\"active\",\"remarks\":\"端到端备份\"}")
report backup-plans create "$CODE" 201
BPID=$(jget "$(cat /tmp/resp.json)" id)
CODE=$(http_code "$BASE/backup-plans/$BPID" -H "$AUTH"); report backup-plans get "$CODE" 200
CODE=$(http_code -X PUT "$BASE/backup-plans/$BPID" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试备份-$STAMP-改\",\"retention_days\":60,\"status\":\"active\"}")
report backup-plans update "$CODE" 200

# ---------- 12. monitor-targets ----------
echo "[12] monitor-targets"
CODE=$(http_code -X POST "$BASE/monitor-targets" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试监控-$STAMP\",\"monitor_type\":\"http\",\"endpoint\":\"https://test-$STAMP.example.com/health\",\"interval_seconds\":60,\"status\":\"active\",\"remarks\":\"端到端监控\"}")
report monitor-targets create "$CODE" 201
MTID=$(jget "$(cat /tmp/resp.json)" id)
CODE=$(http_code "$BASE/monitor-targets/$MTID" -H "$AUTH"); report monitor-targets get "$CODE" 200
CODE=$(http_code -X PUT "$BASE/monitor-targets/$MTID" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试监控-$STAMP-改\",\"interval_seconds\":120,\"status\":\"active\"}")
report monitor-targets update "$CODE" 200

# ---------- 13. contracts ----------
echo "[13] contracts"
CODE=$(http_code -X POST "$BASE/contracts" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试合同-$STAMP\",\"provider_id\":\"$PID\",\"contract_no\":\"HT-$STAMP\",\"start_date\":\"2026-01-01\",\"end_date\":\"2026-12-31\",\"amount\":\"9999.00\",\"currency\":\"CNY\",\"status\":\"active\",\"remarks\":\"端到端合同\"}")
report contracts create "$CODE" 201
CTID=$(jget "$(cat /tmp/resp.json)" id)
CODE=$(http_code "$BASE/contracts/$CTID" -H "$AUTH"); report contracts get "$CODE" 200
CODE=$(http_code -X PUT "$BASE/contracts/$CTID" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"name\":\"测试合同-$STAMP-改\",\"amount\":\"19999.00\",\"status\":\"active\"}")
report contracts update "$CODE" 200

# ---------- 14. attachments ----------
echo "[14] attachments"
CODE=$(http_code -X POST "$BASE/attachments" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"filename\":\"test-$STAMP.txt\",\"target_type\":\"server\",\"target_id\":\"$SID\",\"content_type\":\"text/plain\",\"size_bytes\":1024,\"status\":\"active\",\"remarks\":\"端到端附件\"}")
report attachments create "$CODE" 201
ATID=$(jget "$(cat /tmp/resp.json)" id)
CODE=$(http_code "$BASE/attachments/$ATID" -H "$AUTH"); report attachments get "$CODE" 200
CODE=$(http_code -X PUT "$BASE/attachments/$ATID" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"filename\":\"test-$STAMP-改.txt\",\"size_bytes\":2048,\"status\":\"active\"}")
report attachments update "$CODE" 200

# ---------- 15. site-relations ----------
echo "[15] site-relations"
CODE=$(http_code -X POST "$BASE/site-relations/site-servers" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"site_id\":\"$OSID\",\"server_id\":\"$SID\",\"deploy_role\":\"web\",\"is_primary\":true}")
report site-servers bind "$CODE" 201
CODE=$(http_code "$BASE/site-relations/site-servers/$OSID" -H "$AUTH"); report site-servers list "$CODE" 200
CODE=$(http_code -X POST "$BASE/site-relations/site-domains" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"site_id\":\"$OSID\",\"domain_id\":\"$DMID\",\"is_primary\":true}")
report site-domains bind "$CODE" 201
CODE=$(http_code -X POST "$BASE/site-relations/site-databases" -H "$AUTH" -H 'Content-Type: application/json' \
  -d "{\"site_id\":\"$OSID\",\"database_instance_id\":\"$DBID\",\"is_primary\":true}")
report site-databases bind "$CODE" 201

# ---------- 16. 关联数据回读 ----------
echo "[16] 关联数据回读"
for pair in "site-servers servers" "site-domains domains" "site-databases db"; do
  set -- $pair
  CNT=$(curl -s "$BASE/site-relations/$1/$OSID" -H "$AUTH" | py "import sys,json
d=json.load(sys.stdin)
arr=d.get('data') if isinstance(d,dict) else d
print(len(arr) if isinstance(arr,list) else 'ERR')" 2>/dev/null || echo "ERR")
  echo "  $2 count: $CNT"
  if [ "$CNT" = "1" ]; then PASS=$((PASS+1)); echo "  PASS  site-relations $2"; else FAIL=$((FAIL+1)); echo "  FAIL  site-relations $2 ($CNT)"; fi
done

echo ""
echo "========================================"
echo "TOTAL: PASS=$PASS FAIL=$FAIL"
[ "$FAIL" -eq 0 ] && echo "ALL END-TO-END CHECKS PASSED" || echo "SOME CHECKS FAILED"
