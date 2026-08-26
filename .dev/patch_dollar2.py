#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""补充：给 store 方法调用加 $（xxxOptions.map/filter/find/length 等）"""
import os, re

ROOT = "/mnt/c/workspace/RzOps/rzops-web/src"
STORE_NAMES = [
    "serverStatusOptions","hostingTypeOptions","serverTypeOptions","serverRoleOptions",
    "architectureOptions","raidLevelOptions","webServerSoftwareOptions","ipStatusOptions",
    "ipTypeOptions","protocolOptions","providerTypeOptions","commonStatusOptions",
    "reservedStatusOptions","certificateStatusOptions","certificateTypeOptions",
    "databaseTypeOptions","databaseStatusOptions","importanceOptions","siteStatusOptions",
    "serviceTargetOptions","codeRepoTypeOptions","webFrameworkOptions","credentialTypeOptions",
    "monitorTypeOptions","contractStatusOptions","lineTypeOptions","domainPrivacyStatusOptions",
    "siteServerRoleOptions","siteDatabaseUsageOptions","assetTargetTypeOptions",
]

changed = 0
for dirpath, _, files in os.walk(ROOT):
    for fn in files:
        if not fn.endswith(".svelte"):
            continue
        p = os.path.join(dirpath, fn)
        with open(p, encoding="utf-8") as fh:
            c = fh.read()
        orig = c
        for name in STORE_NAMES:
            # 避免已经带 $ 的（$xxxOptions.map）被重复加
            # 匹配非 $ 前缀的 xxxOptions.method(
            c = re.sub(r"(?<![$\w])" + re.escape(name) + r"\.(map|filter|find|forEach|some|every|reduce|slice|length|indexOf|includes)\b",
                       lambda m: "$" + name + "." + m.group(1), c)
        if c != orig:
            with open(p, "w", encoding="utf-8") as fh:
                fh.write(c)
            changed += 1
            print("patched", p.replace(ROOT, ""))
print("done, changed:", changed)
