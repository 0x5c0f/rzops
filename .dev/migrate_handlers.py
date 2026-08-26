#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""处理 api handler：逐个删除枚举转换 helper 函数 + 行内转换替换。"""
import os, re

ROUTES = "/mnt/c/workspace/RzOps/rzops-api/api/src/routes"
SKIP = {"change_record_handlers.rs", "audit_log_handlers.rs", "mod.rs"}

DEFAULT_MAP = {
    "ReservedStatus::Draft": "draft",
    "ServerStatus::Active": "active",
    "CertificateStatus::Active": "active",
    "ContractStatus::Draft": "draft",
    "CommonStatus::Active": "active",
    "IpStatus::Enabled": "enabled",
    "SiteStatus::Active": "active",
    "DatabaseStatus::Active": "active",
    "IpStatus::Disabled": "disabled",
}

def is_helper_start(s):
    st = s.strip()
    return st.startswith("fn parse_") or re.match(r"fn to_\w+_string\(", st) or re.match(r"fn \w+_to_string\(", st)

def collect_fn_end(lines, i):
    j = i
    while j < len(lines):
        opens = lines[j].count("{")
        closes = lines[j].count("}")
        if opens > closes:
            depth = opens - closes
            while depth > 0:
                j += 1
                if j >= len(lines):
                    return len(lines) - 1
                depth += lines[j].count("{") - lines[j].count("}")
            return j
        if opens == closes and opens > 0:
            return j
        j += 1
    return len(lines) - 1

def process(path):
    with open(path, encoding="utf-8") as fh:
        lines = fh.readlines()

    # 1) 删除 use rzops_domain::enums 行（ChangeType 文件已跳过）
    lines = [ln for ln in lines if "use rzops_domain::enums" not in ln]

    # 2) 逐个删除 helper 函数
    out = []
    i = 0
    while i < len(lines):
        if is_helper_start(lines[i]):
            end = collect_fn_end(lines, i)
            i = end + 1
            continue
        out.append(lines[i])
        i += 1
    lines = out

    # 3) 行内替换
    out = []
    for ln in lines:
        new = ln
        new = re.sub(r"\.as_deref\(\)\.map\(parse_\w+\)", "", new)
        new = re.sub(r"\.as_ref\(\)\.map\(to_\w+_string\)", ".clone()", new)
        new = re.sub(r"\.iter\(\)\.map\(to_\w+_string\)\.collect\(\)", "", new)
        new = re.sub(r"\.iter\(\)\.map\(\|s\| parse_\w+\(s\)\)\.collect\(\)", "", new)
        new = re.sub(r"\.map\(to_\w+_string\)", ".clone()", new)
        new = re.sub(r"to_\w+_string\(&([\w.]+)\)", r"\1.clone()", new)
        new = re.sub(r"(\w+)_to_string\(&([\w.]+)\)", r"\2.clone()", new)
        new = re.sub(r"parse_\w+\(&([\w.]+)\)", r"\1", new)
        new = re.sub(r"parse_\w+\(([\w.]+)\)", r"\1", new)
        for enum_ref, val in DEFAULT_MAP.items():
            new = new.replace(f".unwrap_or({enum_ref})", f'.unwrap_or_else(|| "{val}".to_string())')
        out.append(new)
    lines = out

    with open(path, "w", encoding="utf-8") as fh:
        fh.writelines(lines)
    print(f"processed {os.path.basename(path)}")

for fn in os.listdir(ROUTES):
    if fn in SKIP or not fn.endswith(".rs"):
        continue
    p = os.path.join(ROUTES, fn)
    with open(p, encoding="utf-8") as fh:
        content = fh.read()
    if "rzops_domain::enums" in content:
        process(p)
print("done")
