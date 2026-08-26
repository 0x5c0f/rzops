#!/bin/bash
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "SELECT resource_type, change_type, COUNT(*) FROM cmdb_change_record GROUP BY resource_type, change_type ORDER BY resource_type, change_type;" 2>&1
