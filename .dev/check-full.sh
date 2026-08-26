#!/bin/sh
cd /mnt/c/workspace/RzOps/rzops-web
export PATH="/opt/node/bin:$(echo $PATH | tr ':' '\n' | grep -v '^/mnt/c' | paste -sd:)"
npx svelte-check --output human > /tmp/check-full.txt 2>&1
echo "ERROR_LINES=$(grep -c 'Error:' /tmp/check-full.txt)"
echo "=== all error lines ==="
grep -n -B1 'Error:' /tmp/check-full.txt
echo "=== summary ==="
tail -3 /tmp/check-full.txt
