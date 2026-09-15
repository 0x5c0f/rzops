#!/bin/bash
cd /mnt/c/workspace/RzOps
echo '=== 重建 api + web ==='
docker compose build --no-cache api web 2>&1 | tail -4
echo '=== 重启 ==='
docker compose up -d api web 2>&1 | tail -3
echo '=== 等待启动 ==='
sleep 8
docker compose ps --format 'table {{.Name}}\t{{.Status}}'
echo '=== 健康检查 ==='
curl -s -o /dev/null -w 'api /health -> %{http_code}\n' http://127.0.0.1:8000/health || curl -s -o /dev/null -w 'api / -> %{http_code}\n' http://127.0.0.1:8000/
curl -s -o /dev/null -w 'web / -> %{http_code}\n' http://127.0.0.1:8080/
