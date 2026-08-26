#!/bin/bash
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "SELECT conrelid::regclass AS tbl, conname, pg_get_constraintdef(oid) FROM pg_constraint WHERE contype='u' ORDER BY 1;"
