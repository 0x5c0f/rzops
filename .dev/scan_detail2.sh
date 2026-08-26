#!/bin/bash
cd /mnt/c/workspace/RzOps/rzops-web/src/routes
for d in datacenters providers domains certificates database-instances ops-sites credentials backup-plans monitor-targets; do
  echo "=== $d ==="
  grep -n '= await .*Api.getById\|= await .*Api\.get' "$d/[id]/+page.svelte" | head -2
done
