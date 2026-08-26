#!/bin/bash
cd /mnt/c/workspace/RzOps/rzops-web/src/routes
for d in servers datacenters providers domains certificates database-instances ops-sites credentials backup-plans monitor-targets server-ips server-ports attachments; do
  echo "=== $d ==="
  grep -n 'function handleEdit' "$d/+page.svelte" 2>/dev/null | head -2
  grep -n -A2 'onEdit=' "$d/+page.svelte" 2>/dev/null | head -3
done
