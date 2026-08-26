#!/bin/bash
export PATH="/home/chenxiaodong/node/bin:$PATH"
cd /mnt/c/workspace/RzOps/rzops-web
echo "--- which node/npm ---"
which node npm
echo "--- svelte-kit sync ---"
timeout 120 npx svelte-kit sync 2>&1 | tail -8
echo "--- svelte-check ---"
timeout 280 npx svelte-check --tsconfig ./tsconfig.json 2>&1 | tail -120
