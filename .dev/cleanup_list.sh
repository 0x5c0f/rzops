#!/bin/bash
echo "=== servers ==="
docker exec rzops-postgres psql -U rzops -d rzopsdb -t -c "SELECT id, name, asset_code, server_type, status FROM cmdb_server ORDER BY created_at;"
echo "=== dicts (type, code, label, enabled) ==="
docker exec rzops-postgres psql -U rzops -d rzopsdb -t -c "SELECT dict_type, dict_code, dict_label, enabled FROM cmdb_dict WHERE dict_code IN ('test','alpha','test_type','edge_device','bare_metal') OR dict_type='test_type' ORDER BY dict_type, dict_code;"
echo "=== counts ==="
docker exec rzops-postgres psql -U rzops -d rzopsdb -t -c "SELECT count(*) FROM cmdb_dict WHERE enabled = TRUE; SELECT count(*) FROM cmdb_server;"
