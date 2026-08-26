#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""探查各 repo 文件中的枚举转换 helper 结构，为字典化批量改造做准备。"""
import os, re, sys

BASE = "/mnt/c/workspace/RzOps/rzops-api/infra/src/db/repositories"
files = [f for f in os.listdir(BASE) if f.endswith(".rs")]

for fn in sorted(files):
    path = os.path.join(BASE, fn)
    with open(path, encoding="utf-8") as fh:
        lines = fh.readlines()
    print("=" * 60)
    print(fn)
    for i, ln in enumerate(lines, 1):
        if re.search(r'use rzops_domain::enums|fn parse_|fn \w+_to_string|fn json_to_|fn \w+_to_json|Enum serialization|Row mapper|JSONB', ln):
            print(f"  {i}: {ln.rstrip()}")
