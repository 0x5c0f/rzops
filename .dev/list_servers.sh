#!/bin/bash
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "SELECT name, asset_code, primary_ip, created_at FROM cmdb_server;"
