#!/bin/bash
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "SELECT ip_address, is_primary, ip_type FROM cmdb_server_ip ORDER BY ip_address;"
