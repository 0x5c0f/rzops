#!/bin/bash
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "DELETE FROM cmdb_certificate WHERE name='test-cert-domain';"
docker exec rzops-postgres psql -U rzops -d rzopsdb -t -c "SELECT count(*) FROM cmdb_certificate;"
docker exec rzops-postgres psql -U rzops -d rzopsdb -t -c "SELECT count(*) FROM cmdb_certificate_domain;"
