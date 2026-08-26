#!/bin/bash
cd /mnt/c/workspace/RzOps/rzops-web/src/routes
for d in datacenters providers domains certificates database-instances ops-sites credentials backup-plans monitor-targets; do
  echo "=== $d ==="
  grep -n 'target = await\|site = await\|cert = await\|plan = await\|cred = await\|provider = await\|dc = await\|instance = await\|domain = await\|\.id }' "$d/[id]/+page.svelte" 2>/dev/null | head -3
  echo "--- tail ---"
  tail -4 "$d/[id]/+page.svelte"
done
