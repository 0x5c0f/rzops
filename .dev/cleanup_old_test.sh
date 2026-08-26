#!/bin/bash
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "DELETE FROM cmdb_server WHERE name='test';"
