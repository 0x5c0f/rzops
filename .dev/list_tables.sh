#!/bin/bash
echo "=== 所有 cmdb_ 表 ==="
docker exec rzops-postgres psql -U rzops -d rzopsdb -t -c "SELECT tablename FROM pg_tables WHERE schemaname='public' AND tablename LIKE 'cmdb_%' ORDER BY tablename;"
