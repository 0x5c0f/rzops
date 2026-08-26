#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""批量将 infra repo 中的枚举转换 helper 移除，枚举字段已改为 String。"""
import os, re, sys

BASE = "/mnt/c/workspace/RzOps/rzops-api/infra/src/db/repositories"

# 需处理的文件（排除已完成的 server_repo、保留 ChangeType 的 change_record_repo、
# 无 helper 的 audit_log_repo、手动处理的 provider_repo、mod.rs/user_repo）
TARGETS = [
    "attachment_repo.rs", "backup_plan_repo.rs", "certificate_repo.rs",
    "contract_repo.rs", "credential_repo.rs", "database_instance_repo.rs",
    "datacenter_repo.rs", "domain_repo.rs", "monitor_target_repo.rs",
    "ops_site_repo.rs", "server_ip_repo.rs", "server_port_repo.rs",
    "site_relation_repo.rs",
]

HELPERS = '''// ── JSONB ↔ Vec<String> helpers ──

fn json_to_strings(val: serde_json::Value) -> Vec<String> {
    match val {
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        _ => Vec::new(),
    }
}

fn strings_to_json(vals: &[String]) -> serde_json::Value {
    serde_json::Value::Array(
        vals.iter()
            .map(|s| serde_json::Value::String(s.clone()))
            .collect(),
    )
}

'''

def is_helper_start(line):
    s = line.strip()
    return s.startswith("fn parse_") or re.match(r"fn \w+_to_string\(", s) or re.match(r"fn json_to_\w+\(", s) or re.match(r"fn \w+_to_json\(", s)

def collect_fn_end(lines, i):
    """从函数签名行 i 开始，返回函数体结束的行号（花括号平衡）。"""
    j = i
    while j < len(lines):
        opens = lines[j].count("{")
        closes = lines[j].count("}")
        if opens > closes:
            # 函数体从本行开始且未闭合 → 累计到闭合
            depth = opens - closes
            while depth > 0:
                j += 1
                if j >= len(lines):
                    return len(lines) - 1
                depth += lines[j].count("{") - lines[j].count("}")
            return j
        if opens == closes and opens > 0:
            # 行内已平衡（单行函数）→ 本行即函数体
            return j
        # 纯签名行（无 '{'），继续下一行
        j += 1
    return len(lines) - 1

def process(path):
    with open(path, encoding="utf-8") as fh:
        lines = fh.readlines()

    out = []
    i = 0
    needs_helpers = False
    while i < len(lines):
        ln = lines[i]
        # 删除 use rzops_domain::enums 行
        if "use rzops_domain::enums" in ln:
            i += 1
            continue
        # 删除 helper 函数块
        if is_helper_start(ln):
            end = collect_fn_end(lines, i)
            i = end + 1
            continue
        # 行内替换
        new = ln
        new = re.sub(r"\.map\(\|s\| parse_\w+\(&s\)\)", "", new)
        new = re.sub(r"parse_\w+\(&row\.get::<String, _>\(([^)]*)\)\)", r"row.get::<String, _>(\1)", new)
        new = re.sub(r"parse_\w+\(&(\w+)\)", r"\1", new)
        new = re.sub(r"parse_\w+\((\w+)\)", r"\1", new)
        new = re.sub(r"(\w+)_to_string\(&([\w.]+)\)", r"\2.clone()", new)
        new = re.sub(r"json_to_\w+\((\w+)\)", r"json_to_strings(\1)", new)
        new = re.sub(r"\w+_to_json\(&([\w.]+)\)", r"strings_to_json(&\1)", new)
        if "json_to_strings(" in new or "strings_to_json(" in new:
            needs_helpers = True
        out.append(new)
        i += 1

    # 若用到 JSONB helper，在文件头部插入 helper 定义
    if needs_helpers:
        # 找到第一个 struct 定义行之前插入
        idx = 0
        for k, l in enumerate(out):
            if l.strip().startswith("pub struct") or l.strip().startswith("#[async_trait]"):
                idx = k
                break
        out.insert(idx, HELPERS)

    with open(path, "w", encoding="utf-8") as fh:
        fh.writelines(out)
    print(f"processed {os.path.basename(path)} (helpers={needs_helpers})")

for t in TARGETS:
    p = os.path.join(BASE, t)
    process(p)
print("done")
