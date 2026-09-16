#!/usr/bin/env python3
zh = """### 2026-09-16 Round 11-补: 构建警告清零（a11y）+ PLUGIN_TIMINGS 说明
- **用户发现**：新环境 `docker compose build` 输出中 npm run build 出现代码帧提示（Sidebar.svelte:249 `<div>`），怀疑有警告被遗漏。原因：此前验证用 `grep 'built in|error|Error'` 过滤，漏掉了 warning 类输出。
- **a11y 警告修复**：`[vite-plugin-svelte] Sidebar.svelte:249:8 <div> with a mouseenter or mouseleave handler must have an ARIA role`（SSR + client 各报一次）——折叠态 hover 容器 `<div class="group relative" onmouseenter onmouseleave>` 缺 role。**修复**：加 `role="group"`（菜单分组语义，合理）。构建验证：vite-plugin-svelte / a11y / warn 均零输出。
- **[PLUGIN_TIMINGS] 无法关闭（事实）**：该黄色提示是 Rolldown 引擎的插件耗时性能提示，**非代码警告**。尝试 `build.rolldownOptions.checks: { plugintimings: false }` 关闭失败——Rolldown 该版本 ChecksOptions 中 plugintimings 类型为 `never`（不支持配置，提示硬编码），反而触发 `Invalid input options` 真警告，已回退。**结论**：保留该提示（官方无关闭开关，不影响功能）；a11y 类代码警告必须清零。
- **教训**：构建验证必须看完整输出（不过滤），重点搜 `warn|vite-plugin-svelte|a11y|error`；docker build 日志中的 `=> => # 行号` 代码帧 = 编译警告，需定位修复。
"""
en = """### 2026-09-16 Round 11-add: Build warnings zeroed (a11y) + PLUGIN_TIMINGS note
- **User found**: in a fresh env `docker compose build` showed a code frame during npm run build (Sidebar.svelte:249 `<div>`), suspecting a missed warning. Cause: earlier verification grepped `built in|error|Error` and missed warning-class output.
- **a11y warning fix**: `[vite-plugin-svelte] Sidebar.svelte:249:8 <div> with a mouseenter or mouseleave handler must have an ARIA role` (once each for SSR + client) — the collapsed-mode hover container `<div class="group relative" onmouseenter onmouseleave>` lacked a role. **Fix**: added `role="group"` (menu-group semantics, appropriate). Build verified: zero vite-plugin-svelte / a11y / warn output.
- **[PLUGIN_TIMINGS] cannot be disabled (fact)**: this yellow notice is Rolldown's plugin-timing performance info, **not a code warning**. Tried `build.rolldownOptions.checks: { plugintimings: false }` — failed: this Rolldown version's ChecksOptions types plugintimings as `never` (not configurable, hardcoded), and it produced a real `Invalid input options` warning, so reverted. **Conclusion**: keep the notice (no official off-switch, no functional impact); code warnings of the a11y class must be zero.
- **Lesson**: build verification must read full unfiltered output, grepping `warn|vite-plugin-svelte|a11y|error`; in docker build logs, `=> => # line` code frames = compile warnings that need fixing.
"""

p = 'docs/HANDOVER.md'
s = open(p, encoding='utf-8').read()
if 'Round 11-补' not in s:
    s = s.rstrip() + '\n\n' + zh
    open(p, 'w', encoding='utf-8').write(s)
print('ZH ok')
p = 'docs/HANDOVER.en.md'
s = open(p, encoding='utf-8').read()
if 'Round 11-add' not in s:
    s = s.rstrip() + '\n\n' + en
    open(p, 'w', encoding='utf-8').write(s)
print('EN ok')
