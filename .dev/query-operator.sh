#!/bin/bash
docker exec rzops-postgres psql -U rzops -d rzopsdb -c 'SELECT email, is_superuser FROM "user" WHERE is_superuser = false ORDER BY created_at DESC LIMIT 3;' 2>&1
