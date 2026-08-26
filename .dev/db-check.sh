#!/bin/sh
# RzOps 编辑链路测试脚本（WSL 内）
export PGPASSWORD='1E%q1v9rPDyG'
echo "=== DB servers count ==="
psql -h localhost -U rzops -d rzopsdb -t -c "SELECT count(*) FROM servers;"
psql -h localhost -U rzops -d rzopsdb -t -c "SELECT id, name, status FROM servers LIMIT 3;"
