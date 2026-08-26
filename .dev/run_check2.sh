#!/bin/bash
export PATH="/home/chenxiaodong/node/bin:$PATH"
cd /mnt/c/workspace/RzOps/rzops-web
npx svelte-check --tsconfig ./tsconfig.json 2>&1 | grep -E 'Error|error|found' | head -20
