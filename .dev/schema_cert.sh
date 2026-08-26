#!/bin/bash
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "\d cmdb_certificate_domain"
echo "---"
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "\d cmdb_certificate"
