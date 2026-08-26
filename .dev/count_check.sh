#!/bin/bash
echo "servers: $(docker exec rzops-postgres psql -U rzops -d rzopsdb -t -c 'SELECT count(*) FROM cmdb_server;')"
echo "ips: $(docker exec rzops-postgres psql -U rzops -d rzopsdb -t -c 'SELECT count(*) FROM cmdb_server_ip;')"
echo "ports: $(docker exec rzops-postgres psql -U rzops -d rzopsdb -t -c 'SELECT count(*) FROM cmdb_server_port;')"
