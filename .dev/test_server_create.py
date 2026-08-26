#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""通过 API 创建服务器，验证 server_type=edge_device 字典值落库"""
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

payload = {
    "name": "字典测试服务器API",
    "server_type": "edge_device",
    "status": "active",
}
s, d = req("POST", "/api/v1/servers", token, payload)
print("create server:", s, json.dumps(d, ensure_ascii=False)[:400])

if s in (200, 201):
    sid = d.get("id") or d.get("data", {}).get("id")
    if sid:
        _, srv = req("GET", f"/api/v1/servers/{sid}", token)
        print("server_type stored:", srv.get("server_type"), "status:", srv.get("status"))
