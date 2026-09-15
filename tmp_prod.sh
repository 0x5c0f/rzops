#!/bin/bash
echo '=== 容器内 unbind 产物 ==='
docker exec rzops-web-1 sh -c "grep -rl 'unbind' /usr/share/nginx/html/_app/immutable/ | head -6"
echo '=== 包含 unbind(id: string) 的 chunk 数 ==='
docker exec rzops-web-1 sh -c "grep -rl 'unbind(id: string)' /usr/share/nginx/html/_app/immutable/ | wc -l"
echo '=== 表单 chunk 含 unbind( 调用 ==='
docker exec rzops-web-1 sh -c "grep -rl '\.unbind(' /usr/share/nginx/html/_app/immutable/ | wc -l"
