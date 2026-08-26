#!/bin/bash
TOKEN=$(cat /tmp/rzops_token.txt)
SVR_ID=$(cat /tmp/rzops_svr_id.txt)
echo "=== 清理测试附件(e2e_test.txt) ==="
ATT_ID=$(curl -s "http://localhost:8000/api/v1/attachments?q=e2e_test&page=1&per_page=20" -H "Authorization: Bearer $TOKEN" | python3 -c 'import sys,json; d=json.load(sys.stdin); print(d["data"][0]["id"] if d["data"] else "")')
echo "ATT_ID=$ATT_ID"
if [ -n "$ATT_ID" ]; then
  curl -s -X DELETE -w 'delete_attachment_status=%{http_code}\n' "http://localhost:8000/api/v1/attachments/$ATT_ID" -H "Authorization: Bearer $TOKEN"
fi
echo "=== 清理测试服务器 ==="
curl -s -X DELETE -w 'delete_server_status=%{http_code}\n' "http://localhost:8000/api/v1/servers/$SVR_ID" -H "Authorization: Bearer $TOKEN"
echo "=== 确认清空 ==="
curl -s "http://localhost:8000/api/v1/servers?page=1&per_page=5" -H "Authorization: Bearer $TOKEN" | python3 -c 'import sys,json; print("servers count:", json.load(sys.stdin)["count"])'
curl -s "http://localhost:8000/api/v1/server-ips?page=1&per_page=5" -H "Authorization: Bearer $TOKEN" | python3 -c 'import sys,json; print("server-ips count:", json.load(sys.stdin)["count"])'
curl -s "http://localhost:8000/api/v1/server-ports?page=1&per_page=5" -H "Authorization: Bearer $TOKEN" | python3 -c 'import sys,json; print("server-ports count:", json.load(sys.stdin)["count"])'
