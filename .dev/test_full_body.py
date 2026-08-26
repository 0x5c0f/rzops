#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""用 UI 完整 body 复现 create 500"""
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

_, login = req("POST", "/api/v1/auth/login", body={"email": "admin@rzops.local", "password": "admin123"})
token = login["access_token"]

# UI form 提交的完整结构（模拟 ServerForm form 对象）
payload = {
    "name": "完整body测试",
    "asset_code": "",
    "primary_ip": "",
    "server_type": "edge_device",
    "status": "active",
    "architecture": "",
    "location": "",
    "brand": "",
    "operating_system": "",
    "data_center_id": "",
    "hosting_type": "",
    "isp_provider_id": "",
    "server_provider_id": "",
    "software_provider_id": "",
    "is_dual_line": False,
    "cpu": "",
    "memory_gb": "",
    "disk_layout": "",
    "hardware_config": "",
    "is_raid": False,
    "raid_level": "",
    "is_database_server": False,
    "role_tags": [],
    "web_server_type": [],
    "lease_start_date": "",
    "lease_end_date": "",
    "price": "",
    "price_currency": "CNY",
    "warranty_info": "",
    "remarks": "",
}
s, d = req("POST", "/api/v1/servers", token, payload)
print("create full body:", s, json.dumps(d, ensure_ascii=False)[:300])
