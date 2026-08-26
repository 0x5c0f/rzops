#!/bin/bash
cd /mnt/c/workspace/RzOps/rzops-web/src
grep -rln 'offset: 0' routes --include='+page.svelte' | sort
echo '=== types 里 List.*Query 用 limit/offset 的 ==='
grep -rln 'limit?: number' lib/types --include='*.ts' | sort
