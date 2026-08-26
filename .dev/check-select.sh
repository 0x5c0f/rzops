#!/bin/sh
cd /mnt/c/workspace/RzOps/rzops-web || exit 1
echo '=== package.json bits-ui ==='
grep -n 'bits-ui\|@bit/' package.json
echo '=== ui/select dir ==='
ls src/lib/ui/select/ 2>/dev/null
echo '=== ui/select/index.ts head ==='
head -40 src/lib/ui/select/index.ts 2>/dev/null
