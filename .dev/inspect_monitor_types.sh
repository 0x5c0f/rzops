#!/bin/bash
echo "=== asset_target_type 字典 ==="
docker exec rzops-postgres psql -U rzops -d rzopsdb -t -c "SELECT dict_code,dict_label FROM cmdb_dict WHERE dict_type='asset_target_type' ORDER BY sort_order;"
echo "=== entity-options 资源函数 ==="
grep -n 'export async function get' /mnt/c/workspace/RzOps/rzops-web/src/lib/utils/entity-options.ts
echo "=== monitor_target types ==="
cat /mnt/c/workspace/RzOps/rzops-web/src/lib/types/monitor_target.ts
