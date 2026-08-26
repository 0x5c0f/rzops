#!/bin/bash
cd /mnt/c/workspace/RzOps/rzops-web/src/routes
for d in servers datacenters providers domains certificates database-instances ops-sites credentials backup-plans monitor-targets server-ips server-ports attachments; do
  echo "=== $d ==="
  grep -n -A3 'function handleEdit' "$d/+page.svelte" | head -5
  echo "--- 首列 ---"
  grep -n -A3 'key: .name.\|key: .hostname.\|key: .ip_address.\|key: .domain_name.\|key: .filename.\|key: .service_name.' "$d/+page.svelte" | head -6
done
