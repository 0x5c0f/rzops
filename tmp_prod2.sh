#!/bin/bash
echo '=== unbind URL 模板（容器内） ==='
for ent in server-ips database-instances backup-plans monitor-targets; do
  echo "-- $ent --"
  docker exec rzops-web-1 sh -c "grep -oh '${ent}/[a-zA-Z{]*/unbind' /usr/share/nginx/html/_app/immutable/chunks/ | head -2"
done
