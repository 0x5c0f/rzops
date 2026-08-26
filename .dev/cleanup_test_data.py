#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""清理测试数据：删除测试服务器、软删测试字典项，恢复干净基线"""
import json, urllib.request

BASE = "http://localhost:8000"

def req(method, path, token=None, body=None):
    r = urllib.request.Request(BASE + path, method=method)
    r.add_header("Content-Type", "application/json")
    if token:
        r.add_header("Authorization", "Bearer " + token)
    data = json.dumps(body).encode() if body is not None else None
    try:
        with urllib.request.urlopen(r, data=data) as resp:
            return resp.status, resp.read().decode()
    except urllib.error.HTTPError as e:
        return e.code, e.read().decode()

_, raw = req("POST", "/api/v1/auth/login", body={"email": "admin@rzops.local", "password": "admin123"})
token = json.loads(raw)["access_token"]

# 1. 删除测试服务器
_, raw = req("GET", "/api/v1/servers?limit=100", token)
servers = json.loads(raw)["data"]
del_sv = [s for s in servers if s["name"] in (
    "字典测试服务器API", "fetch测试服务器", "完整body测试5", "UI创建测试服务器",
    "UI测试服务器-176054", "测试服务器-1787732388-改", "测试服务器-1787732428-改",
    "web-01-renamed", "web-1787721998-renamed",
)]
print(f"删除服务器 {len(del_sv)} 台:")
for s in del_sv:
    code, _ = req("DELETE", f"/api/v1/servers/{s['id']}", token)
    print(f"  - {s['name']}: {code}")

# 2. 软删测试字典项
_, raw = req("GET", "/api/v1/dicts", token)
dicts = json.loads(raw)["data"]
del_d = [d for d in dicts if d["dict_code"] in ("edge_device", "alpha") or d["dict_type"] == "test_type"]
print(f"\n软删字典项 {len(del_d)} 条:")
for d in del_d:
    code, _ = req("DELETE", f"/api/v1/dicts/{d['id']}", token)
    print(f"  - {d['dict_type']}/{d['dict_code']} ({d['dict_label']}): {code}")

# 3. 验证
_, raw = req("GET", "/api/v1/dicts?enabled_only=true", token)
enabled = json.loads(raw)["data"]
print(f"\n启用字典项总数: {len(enabled)} (应为 153)")
_, raw = req("GET", "/api/v1/servers?limit=100", token)
print(f"服务器总数: {json.loads(raw)['count']}")
