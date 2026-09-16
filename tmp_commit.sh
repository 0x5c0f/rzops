#!/bin/bash
cd /mnt/c/workspace/RzOps || exit 1
git add -A
git status --short | head -6
git commit -m 'fix: 修复 Svelte a11y 警告（折叠态 hover 容器补 role=group），构建警告清零

- Sidebar.svelte:249 <div> 带 mouseenter/mouseleave 无 ARIA role 触发 a11y_no_static_element_interactions（SSR+client 各一次）
- 折叠态分组 hover 容器补 role="group"（菜单分组语义）
- 构建验证：vite-plugin-svelte / a11y / warn 零输出
- [PLUGIN_TIMINGS] 为 Rolldown 性能提示非警告，尝试 checks 配置关闭失败（类型 never）已回退' 2>&1 | tail -1
git push origin feature/frontend-ui-refactor 2>&1 | tail -1
