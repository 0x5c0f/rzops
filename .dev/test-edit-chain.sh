#!/bin/sh
# RzOps 编辑链路端到端测试（对齐后端 DTO 字段，可重复运行：名称带时间戳）
set -e
BASE="http://localhost:8000/api/v1"
STAMP=$(date +%s)

echo "=== 1. login ==="
TOKEN=$(curl -s -X POST "$BASE/auth/login" -H 'Content-Type: application/json' \
  -d '{"email":"admin@rzops.local","password":"admin123"}' | python3 -c "import sys,json;print(json.load(sys.stdin)['access_token'])")
echo "token len: ${#TOKEN}"

AUTH="Authorization: Bearer $TOKEN"
CT="Content-Type: application/json"

PNAME="测试供应商-$STAMP"
DNAME="测试机房-$STAMP"
SNAME="web-$STAMP"
DNAME_DOM="test$STAMP.com"
SITENAME="测试站点-$STAMP"

echo "=== 2. create provider (provider_types) ==="
PROVIDER=$(curl -s -X POST "$BASE/providers" -H "$AUTH" -H "$CT" \
  -d "{\"name\":\"$PNAME\",\"provider_types\":[\"isp\"],\"contact_name\":\"张三\",\"status\":\"active\"}")
echo "$PROVIDER"
PROVIDER_ID=$(echo "$PROVIDER" | python3 -c "import sys,json;print(json.load(sys.stdin)['id'])")
echo "provider_id: $PROVIDER_ID"

echo "=== 3. create datacenter (city) ==="
DC=$(curl -s -X POST "$BASE/data-centers" -H "$AUTH" -H "$CT" \
  -d "{\"name\":\"$DNAME\",\"city\":\"重庆\",\"country\":\"CN\",\"provider_id\":\"$PROVIDER_ID\",\"status\":\"active\"}")
echo "$DC"
DC_ID=$(echo "$DC" | python3 -c "import sys,json;print(json.load(sys.stdin)['id'])")
echo "dc_id: $DC_ID"

echo "=== 4. create server ==="
SERVER=$(curl -s -X POST "$BASE/servers" -H "$AUTH" -H "$CT" \
  -d "{\"name\":\"$SNAME\",\"primary_ip\":\"10.0.0.1\",\"server_type\":\"physical\",\"status\":\"active\",\"data_center_id\":\"$DC_ID\",\"isp_provider_id\":\"$PROVIDER_ID\",\"location\":\"重庆\"}")
echo "$SERVER"
SERVER_ID=$(echo "$SERVER" | python3 -c "import sys,json;print(json.load(sys.stdin)['id'])")
echo "server_id: $SERVER_ID"

echo "=== 5. update server (rename + clear location) ==="
curl -s -X PUT "$BASE/servers/$SERVER_ID" -H "$AUTH" -H "$CT" \
  -d "{\"name\":\"$SNAME-renamed\",\"server_type\":\"virtual\",\"location\":\"\",\"remarks\":\"测试备注\"}" | python3 -m json.tool

echo "=== 6. verify server update persisted ==="
EXPECT_SNAME="$SNAME-renamed"
curl -s "$BASE/servers/$SERVER_ID" -H "$AUTH" | python3 -c "
import sys,json
s=json.load(sys.stdin)
print('name:', s['name'])
print('server_type:', s['server_type'])
print('location:', repr(s['location']))
print('remarks:', repr(s['remarks']))
print('data_center_id kept:', s['data_center_id'])
assert s['name']=='$EXPECT_SNAME', 'name not updated'
assert s['server_type']=='virtual', 'server_type not updated'
assert s['location']=='', 'location not cleared'
assert s['data_center_id'], 'data_center_id lost!'
print('SERVER CHECKS PASSED')
"

echo "=== 7. create domain ==="
DOMAIN=$(curl -s -X POST "$BASE/domains" -H "$AUTH" -H "$CT" \
  -d "{\"domain_name\":\"$DNAME_DOM\",\"expiry_date\":\"2027-12-31\",\"provider_id\":\"$PROVIDER_ID\",\"renewal_currency\":\"CNY\",\"is_enabled\":true,\"remarks\":\"测试域名\"}")
echo "$DOMAIN"
DOMAIN_ID=$(echo "$DOMAIN" | python3 -c "import sys,json;print(json.load(sys.stdin)['id'])")
echo "domain_id: $DOMAIN_ID"

echo "=== 8. update domain (rename + expiry) ==="
curl -s -X PUT "$BASE/domains/$DOMAIN_ID" -H "$AUTH" -H "$CT" \
  -d "{\"domain_name\":\"renamed-$STAMP.com\",\"expiry_date\":\"2028-06-30\",\"is_enabled\":false}" | python3 -m json.tool

echo "=== 9. verify domain update persisted ==="
EXPECT_DOM="renamed-$STAMP.com"
curl -s "$BASE/domains/$DOMAIN_ID" -H "$AUTH" | python3 -c "
import sys,json
d=json.load(sys.stdin)
print('domain_name:', d['domain_name'])
print('expiry_date:', repr(d['expiry_date']))
print('is_enabled:', d['is_enabled'])
assert d['domain_name']=='$EXPECT_DOM', 'domain not updated'
assert d['expiry_date']=='2028-06-30', 'expiry not updated'
assert d['is_enabled'] is False, 'is_enabled not updated'
print('DOMAIN CHECKS PASSED')
"

echo "=== 10. create ops-site ==="
SITE=$(curl -s -X POST "$BASE/ops-sites" -H "$AUTH" -H "$CT" \
  -d "{\"name\":\"$SITENAME\",\"url\":\"https://example.com\",\"service_target\":\"对外\",\"importance\":\"高\",\"status\":\"active\"}")
echo "$SITE"
SITE_ID=$(echo "$SITE" | python3 -c "import sys,json;print(json.load(sys.stdin)['id'])")
echo "site_id: $SITE_ID"

echo "=== 11. create site-server relation ==="
REL=$(curl -s -X POST "$BASE/site-relations/site-servers" -H "$AUTH" -H "$CT" \
  -d "{\"site_id\":\"$SITE_ID\",\"server_id\":\"$SERVER_ID\",\"deploy_role\":\"web\",\"is_primary\":true}")
echo "$REL"

echo "=== 12. list site-servers ==="
curl -s "$BASE/site-relations/site-servers/$SITE_ID" -H "$AUTH" | python3 -c "
import sys,json
data=json.load(sys.stdin)
items = data if isinstance(data, list) else data.get('data', [])
print('relations count:', len(items))
assert len(items) >= 1, 'no site-server relation found'
r = items[0]
print('deploy_role:', r.get('deploy_role'))
print('is_primary:', r.get('is_primary'))
assert r.get('server_id') == '$SERVER_ID', 'server_id mismatch'
print('SITE-RELATION CHECKS PASSED')
"

echo "=== 13. create site-domain relation ==="
curl -s -X POST "$BASE/site-relations/site-domains" -H "$AUTH" -H "$CT" \
  -d "{\"site_id\":\"$SITE_ID\",\"domain_id\":\"$DOMAIN_ID\",\"is_primary\":true}" | python3 -m json.tool

echo "=== 14. verify site-domain relation ==="
curl -s "$BASE/site-relations/site-domains/$SITE_ID" -H "$AUTH" | python3 -c "
import sys,json
data=json.load(sys.stdin)
items = data if isinstance(data, list) else data.get('data', [])
print('domain relations count:', len(items))
assert len(items) >= 1, 'no site-domain relation found'
print('domain_id:', items[0].get('domain_id'))
print('SITE-DOMAIN CHECKS PASSED')
"

echo ""
echo "ALL END-TO-END CHECKS PASSED"
