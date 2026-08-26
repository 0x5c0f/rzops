#!/bin/bash
cd /mnt/c/workspace/RzOps/rzops-web/src/routes
for d in datacenters providers domains certificates database-instances; do
  echo "=== $d ==="
  grep -n '\.getById\|= await .*Api' "$d/[id]/+page.svelte" | head -4
done
