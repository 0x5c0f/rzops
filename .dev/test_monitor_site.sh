#!/bin/bash
TOKEN=$(cat /tmp/rzops_token.txt)
SITE_ID=$(curl -s "http://localhost:8000/api/v1/ops-sites?per_page=1" -H "Authorization: Bearer $TOKEN" | python3 -c 'import sys,json; d=json.load(sys.stdin); print(d["data"][0]["id"] if d["data"] else "")' 2>/dev/null)
echo "SITE_ID=$SITE_ID"
MT_ID=$(curl -s "http://localhost:8000/api/v1/monitor-targets?per_page=1" -H "Authorization: Bearer $TOKEN" | python3 -c 'import sys,json; d=json.load(sys.stdin); print(d["data"][0]["id"] if d["data"] else "")' 2>/dev/null)
echo "MT_ID=$MT_ID"
echo "--- update monitor-target with site_id ---"
curl -s -X PUT "http://localhost:8000/api/v1/monitor-targets/$MT_ID" \
  -H "Authorization: Bearer $TOKEN" -H 'Content-Type: application/json' \
  -d "{\"site_id\":\"$SITE_ID\",\"target_type\":\"site\",\"target_id\":\"$SITE_ID\"}" | python3 -c 'import sys,json; d=json.load(sys.stdin); print("site_id:",d.get("site_id"),"| target_type:",d.get("target_type"),"| target_id:",d.get("target_id"))'
echo "--- list filter by site_id ---"
curl -s "http://localhost:8000/api/v1/monitor-targets?site_id=$SITE_ID" -H "Authorization: Bearer $TOKEN" | python3 -c 'import sys,json; d=json.load(sys.stdin); print("count:",d["count"])'
