#!/bin/bash
docker exec rzops-postgres psql -U rzops -d rzopsdb -t -c "SELECT DISTINCT dict_type FROM cmdb_dict ORDER BY 1;"
