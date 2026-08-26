#!/bin/bash
set -e
BASE=http://localhost:8000/api/v1
ADMIN_TOKEN=$(curl -s -X POST $BASE/auth/login -H 'Content-Type: application/json' -d '{"email":"admin@rzops.local","password":"admin123"}' | python3 -c 'import sys,json;print(json.load(sys.stdin)["access_token"])')
echo "== admin token ok =="

# 1. admin (superuser) registers a normal operator account -> expect 201
EMAIL="opuser-$(date +%s)@rzops.local"
REG=$(curl -s -o /tmp/reg.json -w "%{http_code}" -X POST $BASE/auth/register -H "Authorization: Bearer $ADMIN_TOKEN" -H 'Content-Type: application/json' -d "{\"email\":\"$EMAIL\",\"password\":\"op123456\",\"full_name\":\"Operator\"}")
echo "1) admin register -> HTTP $REG (expect 201)"
[ "$REG" = "201" ] && echo "   PASS" || { echo "   FAIL: $(cat /tmp/reg.json)"; exit 1; }

# 2. operator login
OP_TOKEN=$(curl -s -X POST $BASE/auth/login -H 'Content-Type: application/json' -d "{\"email\":\"$EMAIL\",\"password\":\"op123456\"}" | python3 -c 'import sys,json;print(json.load(sys.stdin)["access_token"])')
echo "== operator token ok =="

# 3. operator accesses audit-logs -> expect 403
CODE=$(curl -s -o /tmp/al.json -w "%{http_code}" $BASE/audit-logs -H "Authorization: Bearer $OP_TOKEN")
echo "2) operator GET /audit-logs -> HTTP $CODE (expect 403)"
[ "$CODE" = "403" ] && echo "   PASS" || { echo "   FAIL: $(cat /tmp/al.json)"; exit 1; }

# 4. operator tries to register another account -> expect 403
CODE2=$(curl -s -o /tmp/reg2.json -w "%{http_code}" -X POST $BASE/auth/register -H "Authorization: Bearer $OP_TOKEN" -H 'Content-Type: application/json' -d "{\"email\":\"hacker-$(date +%s)@rzops.local\",\"password\":\"x123456\"}")
echo "3) operator register -> HTTP $CODE2 (expect 403)"
[ "$CODE2" = "403" ] && echo "   PASS" || { echo "   FAIL: $(cat /tmp/reg2.json)"; exit 1; }

# 5. admin accesses audit-logs -> expect 200
CODE3=$(curl -s -o /tmp/al2.json -w "%{http_code}" $BASE/audit-logs -H "Authorization: Bearer $ADMIN_TOKEN")
echo "4) admin GET /audit-logs -> HTTP $CODE3 (expect 200)"
[ "$CODE3" = "200" ] && echo "   PASS" || { echo "   FAIL"; exit 1; }

echo "ALL PERMISSION CHECKS PASSED"
