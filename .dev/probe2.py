#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import subprocess
for f in ["attachment","backup_plan","credential","ops_site_relation"]:
    out = subprocess.run(["git","show","HEAD:rzops-api/domain/src/models/%s.rs"%f],
                         capture_output=True, text=True, cwd="/mnt/c/workspace/RzOps").stdout
    print("=== %s ===" % f)
    for ln in out.splitlines():
        if "pub status" in ln or "usage_type" in ln or "deploy_role" in ln:
            print(ln)
