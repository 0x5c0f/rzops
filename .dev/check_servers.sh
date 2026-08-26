#!/bin/bash
docker exec rzops-postgres psql -U rzops -d rzopsdb -t -c "SELECT name, server_type, status, created_at FROM cmdb_server WHERE name LIKE '%测试%' ORDER BY created_at DESC LIMIT 15;"
