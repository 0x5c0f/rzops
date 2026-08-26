#!/bin/bash
# API 冒烟测试: certificate-domains CRUD
BASE="http://localhost:8000/api/v1"
TOKEN=$(curl -s -X POST "$BASE/auth/login" -H "Content-Type: application/json" -d '{"email":"admin@rzops.local","password":"admin123"}' | python3 -c "import sys,json;print(json.load(sys.stdin)['access_token'])")
echo "TOKEN ok: ${TOKEN:0:12}..."

AUTH="Authorization: Bearer $TOKEN"
CT="Content-Type: application/json"

# 1. 创建证书（若没有）
CERT_ID=$(curl -s "$BASE/certificates?page=1&per_page=5" -H "$AUTH" | python3 -c "import sys,json;d=json.load(sys.stdin);print(d['data'][0]['id'] if d.get('data') else '')")
if [ -z "$CERT_ID" ]; then
  CERT_ID=$(curl -s -X POST "$BASE/certificates" -H "$AUTH" -H "$CT" -d '{"name":"smoke-cert"}' | python3 -c "import sys,json;print(json.load(sys.stdin)['id'])")
  echo "created cert: $CERT_ID"
else
  echo "existing cert: $CERT_ID"
fi

# 2. 创建证书-域名绑定
RESP=$(curl -s -X POST "$BASE/certificate-domains" -H "$AUTH" -H "$CT" -d "{\"certificate_id\":\"$CERT_ID\",\"domain_pattern\":\"*.smoke.example.com\",\"is_primary\":true}")
echo "create resp: $RESP"
CD_ID=$(echo "$RESP" | python3 -c "import sys,json;print(json.load(sys.stdin)['id'])")

# 3. 列表（按 certificate_id 过滤）
echo "list:"
curl -s "$BASE/certificate-domains?certificate_id=$CERT_ID" -H "$AUTH" | python3 -m json.tool

# 4. 更新
echo "update:"
curl -s -X PUT "$BASE/certificate-domains/$CD_ID" -H "$AUTH" -H "$CT" -d '{"domain_pattern":"*.smoke2.example.com","is_primary":false}' | python3 -m json.tool

# 5. 删除
echo "delete:"
curl -s -o /dev/null -w "%{http_code}\n" -X DELETE "$BASE/certificate-domains/$CD_ID" -H "$AUTH"

# 清理证书
curl -s -o /dev/null -w "cleanup cert: %{http_code}\n" -X DELETE "$BASE/certificates/$CERT_ID" -H "$AUTH"
