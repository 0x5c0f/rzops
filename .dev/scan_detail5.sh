#!/bin/bash
cd /mnt/c/workspace/RzOps/rzops-web/src/routes
for f in 'certificates/[id]/+page.svelte' 'database-instances/[id]/+page.svelte' 'datacenters/[id]/+page.svelte' 'domains/[id]/+page.svelte' 'providers/[id]/+page.svelte'; do
  echo "=== $f ==="
  grep -n 'let .* = \$state\|const \[' "$f" | head -10
done
