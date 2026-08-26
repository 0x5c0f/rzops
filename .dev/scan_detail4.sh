#!/bin/bash
cd /mnt/c/workspace/RzOps/rzops-web/src/routes
for f in 'datacenters/[id]/+page.svelte' 'providers/[id]/+page.svelte' 'domains/[id]/+page.svelte' 'certificates/[id]/+page.svelte' 'database-instances/[id]/+page.svelte'; do
  echo "=== $f ==="
  grep -n -B2 -A2 'getById(id)' "$f" | head -10
done
