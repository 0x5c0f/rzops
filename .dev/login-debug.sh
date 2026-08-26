#!/bin/sh
# 登录调试
BASE="http://localhost:8000/api/v1"
echo "=== raw login response ==="
curl -s -i -X POST "$BASE/auth/login" -H 'Content-Type: application/json' \
  -d '{"email":"admin@rzops.local","password":"admin123"}' | head -30
