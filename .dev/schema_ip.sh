#!/bin/bash
echo "=== cmdb_server ==="
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "\d cmdb_server"
echo "=== cmdb_server_ip ==="
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "\d cmdb_server_ip"
