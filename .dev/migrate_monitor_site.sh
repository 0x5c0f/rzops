#!/bin/bash
docker exec rzops-postgres psql -U rzops -d rzopsdb -c "ALTER TABLE cmdb_monitor_target ADD COLUMN IF NOT EXISTS site_id uuid REFERENCES cmdb_ops_site(id) ON DELETE SET NULL;"
docker exec rzops-postgres psql -U rzops -d rzopsdb -t -c "SELECT column_name, data_type FROM information_schema.columns WHERE table_name='cmdb_monitor_target' ORDER BY ordinal_position;"
