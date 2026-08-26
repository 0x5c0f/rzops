#!/bin/bash
set -e
TOKEN=$(curl -s -X POST http://localhost:8000/api/v1/auth/login -H 'Content-Type: application/json' -d '{"email":"admin@rzops.local","password":"admin123"}' | python3 -c 'import sys,json;print(json.load(sys.stdin)["access_token"])')
echo "token ok: ${#TOKEN} chars"
NAME="chglog-test-$(date +%s)"
RESP=$(curl -s -X POST http://localhost:8000/api/v1/providers -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' -d "{\"name\":\"$NAME\",\"provider_types\":[\"isp\"],\"status\":\"active\"}")
echo "create resp: $RESP"
ID=$(echo "$RESP" | python3 -c 'import sys,json;print(json.load(sys.stdin)["id"])')
echo "created id: $ID"
curl -s -X PUT http://localhost:8000/api/v1/providers/$ID -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' -d "{\"name\":\"${NAME}-renamed\"}" >/dev/null
echo "--- change records ---"
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "SELECT change_type, resource_type, actor_id IS NOT NULL AS has_actor, (before_data IS NOT NULL) AS has_before, (after_data IS NOT NULL) AS has_after FROM cmdb_change_record WHERE resource_type='provider' ORDER BY created_at DESC LIMIT 5;" 2>&1 | head -20
