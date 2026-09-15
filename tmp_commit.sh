#!/bin/bash
cd /mnt/c/workspace/RzOps
rm -f tmp_build.sh tmp_sf.sh
git add rzops-web/src/lib/components/forms/ServerIpForm.svelte
git commit -m "fix: 服务器IP重复提醒检查请求路径修复为 /api/v1 并加缓存规避参数

- 检查请求路径由 /server-ips 修正为 /api/v1/server-ips，避免 nginx SPA fallback 返回 index.html 导致 JSON 解析失败
- 检查请求增加 _t 时间戳参数，防止浏览器内存缓存复用旧检查结果
- 增加 [dup-check] 观测日志便于定位"
git add rzops-web/src/routes/backup-plans/+page.svelte \
        rzops-web/src/routes/backup-plans/[id]/+page.svelte \
        rzops-web/src/routes/database-instances/+page.svelte \
        rzops-web/src/routes/database-instances/[id]/+page.svelte \
        rzops-web/src/routes/monitor-targets/+page.svelte \
        rzops-web/src/routes/monitor-targets/[id]/+page.svelte \
        rzops-web/src/routes/server-ips/+page.svelte \
        rzops-web/src/routes/server-ips/[id]/+page.svelte \
        rzops-web/src/routes/server-ports/[id]/+page.svelte
git commit -m "fix: 统一关联目标列未关联与已删除的显示文案

- 列表/详情页未关联目标由 '-' 改为 '未关联目标'，未绑定服务器改为 '未关联服务器'
- 服务器列 '已删除' 统一为 '服务器已删除'（server-ips/database-instances/server-ports 详情）
- 保留原有行级颜色逻辑（未关联/已删除/已退役的颜色区分不变）"
echo '=== push ==='
git push origin feature/frontend-ui-refactor 2>&1 | tail -3
echo '=== log ==='
git log --oneline -3
