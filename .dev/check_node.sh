#!/bin/bash
if [ -e /mnt/c/workspace/RzOps/rzops-web/node_modules/.bin/svelte-kit ]; then
  echo "WSL_has_svelte_kit"
else
  echo "WSL_no_svelte_kit"
fi
echo "node_modules count: $(ls /mnt/c/workspace/RzOps/rzops-web/node_modules 2>/dev/null | wc -l)"
echo "--- windows node/npm ---"
cmd.exe /c "where node npm" 2>/dev/null | head -5
