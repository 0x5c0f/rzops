#!/bin/bash
cd /mnt/c/workspace/RzOps/rzops-web/src/routes
for d in servers datacenters providers domains certificates database-instances ops-sites credentials backup-plans monitor-targets server-ips server-ports attachments; do
  echo -n "$d: "
  ls "$d/" 2>/dev/null | tr '\n' ' '
  echo
done
