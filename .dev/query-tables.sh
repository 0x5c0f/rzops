#!/bin/bash
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "SELECT tablename FROM pg_tables WHERE schemaname='public' AND (tablename LIKE '%user%' OR tablename LIKE '%account%');" 2>&1
