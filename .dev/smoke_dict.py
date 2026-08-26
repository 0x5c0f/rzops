#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""字典 API 冒烟测试"""
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
            return resp.status, json.loads(resp.read().decode() or "null")
    except urllib.error.HTTPError as e:
        return e.code, json.loads(e.read().decode() or "null")

# 登录
_, login = req("POST", "/api/v1/auth/login", body={"email": "admin@rzops.local", "password": "admin123"})
token = login["access_token"]
print("login ok")

# 按类型查询
s, d = req("GET", "/api/v1/dicts?dict_type=server_type&enabled_only=true", token)
print("server_type:", [(i["dict_code"], i["dict_label"]) for i in d["data"]])

# 全量
s, d = req("GET", "/api/v1/dicts", token)
print("total dicts:", d["count"])

# 创建
s, d = req("POST", "/api/v1/dicts", token, {"dict_type": "server_type", "dict_code": "bare_metal", "dict_label": "裸金属", "sort_order": 99})
print("create:", s, d.get("dict_code") if isinstance(d, dict) else d)

# 更新
if s == 201 and isinstance(d, dict):
    did = d["id"]
    s2, d2 = req("PUT", f"/api/v1/dicts/{did}", token, {"dict_label": "裸金属服务器", "enabled": True})
    print("update:", s2, d2.get("dict_label") if isinstance(d2, dict) else d2)

# 删除（软删）
s3, d3 = req("DELETE", f"/api/v1/dicts/{did}", token)
print("delete:", s3)

# 验证软删后 list 不含
s4, d4 = req("GET", "/api/v1/dicts?dict_type=server_type", token)
print("after delete, bare_metal present in full list:", any(i["dict_code"] == "bare_metal" for i in d4["data"]))
s5, d5 = req("GET", "/api/v1/dicts?dict_type=server_type&enabled_only=true", token)
print("after delete, bare_metal present in enabled list:", any(i["dict_code"] == "bare_metal" for i in d5["data"]))
