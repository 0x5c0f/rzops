#!/bin/bash
# 重置 test-inline-v2: primary_ip=192.168.50.20, IP 行 is_primary 全 false
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "UPDATE cmdb_server SET primary_ip='192.168.50.20' WHERE name='test-inline-v2'; UPDATE cmdb_server_ip SET is_primary=false WHERE server_id=(SELECT id FROM cmdb_server WHERE name='test-inline-v2');"
