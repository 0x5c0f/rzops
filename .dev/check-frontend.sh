#!/bin/sh
# RzOps 前端类型检查脚本（WSL 内运行）
export PATH="/opt/node/bin:$(echo $PATH | tr ':' '\n' | grep -v '^/mnt/c' | paste -sd:)"
cd /mnt/c/workspace/RzOps/rzops-web
npx svelte-check --output human 2>&1 | tail -40
