#!/bin/bash
TOKEN=$(cat /tmp/rzops_token.txt)
echo "--- servers ---"
curl -s 'http://localhost:8000/api/v1/servers?page=1&per_page=20' -H "Authorization: Bearer $TOKEN" | head -c 600
echo ""
echo "--- server-ips (server_ip count) ---"
curl -s 'http://localhost:8000/api/v1/server-ips?page=1&per_page=5' -H "Authorization: Bearer $TOKEN" | head -c 400
