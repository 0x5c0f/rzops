#!/bin/bash
echo '=== unbind 上下文（容器内） ==='
docker exec rzops-web-1 sh -c "grep -o '.\{40\}unbind.\{60\}' /usr/share/nginx/html/_app/immutable/chunks/xsSNLzWg.js | head -4"
echo '--- 其他 chunk ---'
docker exec rzops-web-1 sh -c "grep -o '.\{30\}unbind.\{50\}' /usr/share/nginx/html/_app/immutable/chunks/CF2-yNOH.js | head -4"
