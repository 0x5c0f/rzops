#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""批量给前端页面加 $ 解包（store）。排除 changeTypeOptions（静态数组）。"""
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
            # options={xxxOptions} → options={$xxxOptions}
            c = c.replace(f"options={{{name}}}", f"options={{{chr(36)}{name}}}")
            # getOptionLabel(xxxOptions, → getOptionLabel($xxxOptions,
            c = c.replace(f"getOptionLabel({name}", f"getOptionLabel({chr(36)}{name}")
            c = c.replace(f"getOptionLabels({name}", f"getOptionLabels({chr(36)}{name}")
            # {#each xxxOptions as → {#each $xxxOptions as
            c = re.sub(r"\{#each\s+" + re.escape(name) + r"\s+as", "{#each $" + name + " as", c)
            # ensureOption/ensureOptions 数组参数
            c = c.replace(f"ensureOption({name}", f"ensureOption({chr(36)}{name}")
            c = c.replace(f"ensureOptions({name}", f"ensureOptions({chr(36)}{name}")
        if c != orig:
            with open(p, "w", encoding="utf-8") as fh:
                fh.write(c)
            changed += 1
            print("patched", p.replace(ROOT, ""))
print("done, changed files:", changed)
