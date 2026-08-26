#!/bin/bash
TOKEN=$(cat /tmp/rzops_token.txt)
# 取一个真实服务器 id 作为 target
SERVER_ID=$(curl -s "http://localhost:8000/api/v1/servers?per_page=1" -H "Authorization: Bearer $TOKEN" | python3 -c 'import sys,json; d=json.load(sys.stdin); print(d["data"][0]["id"] if d["data"] else "")')
echo "SERVER_ID=$SERVER_ID"
# 创建测试文件
echo "rzops attachment test content 附件测试内容" > /tmp/rzops_test.txt
echo "--- upload ---"
UP=$(curl -s -X POST http://localhost:8000/api/v1/attachments/upload \
  -H "Authorization: Bearer $TOKEN" \
  -F "file=@/tmp/rzops_test.txt" \
  -F "target_type=server" \
  -F "target_id=$SERVER_ID" \
  -F "remarks=测试上传")
echo "$UP" | head -c 600
echo ""
ATT_ID=$(echo "$UP" | python3 -c 'import sys,json; print(json.load(sys.stdin)["id"])' 2>/dev/null)
echo "ATT_ID=$ATT_ID"
echo "--- download ---"
curl -s -o /tmp/dl.txt -w 'status=%{http_code}\n' "http://localhost:8000/api/v1/attachments/$ATT_ID/download" -H "Authorization: Bearer $TOKEN"
echo "下载内容: $(cat /tmp/dl.txt)"
echo "--- list ---"
curl -s "http://localhost:8000/api/v1/attachments?target_type=server&target_id=$SERVER_ID" -H "Authorization: Bearer $TOKEN" | python3 -m json.tool 2>/dev/null | head -40
echo "--- delete ---"
curl -s -X DELETE -w 'status=%{http_code}\n' "http://localhost:8000/api/v1/attachments/$ATT_ID" -H "Authorization: Bearer $TOKEN"
echo "--- 删除后磁盘文件检查 ---"
ls -la /mnt/c/workspace/RzOps/rzops-api/uploads/ 2>/dev/null | tail -3 || echo "uploads dir empty or missing"
