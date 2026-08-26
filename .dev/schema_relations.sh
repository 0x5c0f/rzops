#!/bin/bash
echo "=== 所有业务表及其外键（找出从属明细表）==="
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "
SELECT tc.table_name AS child, kcu.column_name AS col, ccu.table_name AS parent
FROM information_schema.table_constraints tc
JOIN information_schema.key_column_usage kcu ON tc.constraint_name = kcu.constraint_name
JOIN information_schema.constraint_column_usage ccu ON tc.constraint_name = ccu.constraint_name
WHERE tc.constraint_type = 'FOREIGN KEY'
  AND tc.table_schema = 'public'
  AND tc.table_name NOT LIKE 'pg_%'
ORDER BY tc.table_name, kcu.column_name;"
echo "=== 服务器端口表 ==="
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "\d cmdb_server_port"
