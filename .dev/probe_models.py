#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import subprocess, re

files = ["attachment","backup_plan","credential","data_center","database_instance","domain_asset",
         "monitor_target","ops_site","ops_site_relation","provider","server","server_ip","server_port",
         "certificate","contract"]
pat = re.compile(r"pub (status|monitor_type|target_type|credential_type|certificate_type|line_type|privacy_status|importance|hosting_type|server_type|web_server_type|role_tags|service_target|code_repo_type|web_framework|deploy_role|usage_type|protocol|ip_type|db_type)\b[^;]*")
for f in files:
    out = subprocess.run(["git","show","HEAD:rzops-api/domain/src/models/%s.rs"%f],
                         capture_output=True, text=True, cwd="/mnt/c/workspace/RzOps").stdout
    print("=== %s ===" % f)
    for m in pat.finditer(out):
        print(m.group(0)[:110])
