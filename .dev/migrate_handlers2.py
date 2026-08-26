#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""补充处理 handler 跨行 .as_deref()/.map(parse_x) 与短名 xxx_to_string。"""
import os, re

ROUTES = "/mnt/c/workspace/RzOps/rzops-api/api/src/routes"
SKIP = {"change_record_handlers.rs", "audit_log_handlers.rs", "mod.rs"}

for fn in os.listdir(ROUTES):
    if fn in SKIP or not fn.endswith(".rs"):
        continue
    p = os.path.join(ROUTES, fn)
    with open(p, encoding="utf-8") as fh:
        content = fh.read()
    if "parse_" not in content and "_to_string" not in content:
        continue
    # 跨行 .as_deref()\n.map(parse_x) 删除
    content = re.sub(r"\.as_deref\(\)\s*\n\s*\.map\(parse_\w+\)", "", content)
    # 同行 .as_deref().map(parse_x)
    content = re.sub(r"\.as_deref\(\)\.map\(parse_\w+\)", "", content)
    # 短名 .as_ref().map(xxx_to_string) → .clone()
    content = re.sub(r"\.as_ref\(\)\.map\((\w+_to_string)\)", ".clone()", content)
    content = re.sub(r"\.as_ref\(\)\.map\(to_\w+_string\)", ".clone()", content)
    # .map(parse_x) 残留（body.x.map(parse_status) → body.x）
    content = re.sub(r"\.map\(parse_\w+\)", "", content)
    # xxx_to_string(&x) / to_xxx_string(&x)
    content = re.sub(r"(\w+)_to_string\(&([\w.]+)\)", r"\2.clone()", content)
    content = re.sub(r"to_\w+_string\(&([\w.]+)\)", r"\1.clone()", content)
    with open(p, "w", encoding="utf-8") as fh:
        fh.write(content)
    print(f"patched {fn}")
print("done")
