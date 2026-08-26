#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""添加 server_type 新字典项并验证"""
import json, urllib.request

BASE = "http://localhost:8000"

def req(method, path, token=None, body=None):
    r = urllib.request.Request(BASE + path, method=method)
    r.add_header("Content-Type", "application/json")
    if token:
        r.add_header("Authorization", "Bearer " + token)
    data = json.dumps(body).encode() if body is not None else None
    with urllib.request.urlopen(r, data=data) as resp:
        return resp.status, json.loads(resp.read().decode() or "null")

_, login = req("POST", "/api/v1/auth/login", body={"email": "admin@rzops.local", "password": "admin123"})
token = login["access_token"]

# 幂等：先查是否存在
s, lst = req("GET", "/api/v1/dicts?dict_type=server_type", token)
if any(i["dict_code"] == "edge_device" for i in lst["data"]):
    print("edge_device already exists")
else:
    s, d = req("POST", "/api/v1/dicts", token, {"dict_type": "server_type", "dict_code": "edge_device", "dict_label": "边缘设备", "sort_order": 98})
    print("created edge_device:", s)

# 验证列表
s, lst = req("GET", "/api/v1/dicts?dict_type=server_type&enabled_only=true", token)
print("server_type enabled:", [(i["dict_code"], i["dict_label"]) for i in lst["data"]])
