#!/bin/bash
echo "=== cmdb_monitor_target ==="
docker exec rzops-postgres psql -U rzops -d rzopsdb -c '\d cmdb_monitor_target'
echo "=== monitor_target site column ==="
docker exec rzops-postgres psql -U rzops -d rzopsdb -t -c "SELECT column_name FROM information_schema.columns WHERE table_name='cmdb_monitor_target' AND column_name LIKE '%site%';"
echo "=== attachment routes ==="
ls /mnt/c/workspace/RzOps/rzops-api/api/src/routes/ | grep -i attach
echo "=== attachment handlers upload ==="
grep -n 'upload\|storage_key\|multipart\|stream' /mnt/c/workspace/RzOps/rzops-api/api/src/routes/attachment_handlers.rs 2>/dev/null | head -15
echo "=== attachment route registration ==="
grep -n 'attachment' /mnt/c/workspace/RzOps/rzops-api/api/src/routes/mod.rs | head
