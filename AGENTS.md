# AGENTS.md — RzOps CMDB 协作速查（AI 会话自动加载）

> 面向后续开发者与 AI Agent 的精简速查。完整交接文档见 [docs/HANDOVER.md](docs/HANDOVER.md)。

## 项目一句话

RzOps 是运维 CMDB：Rust(axum) 后端 + Svelte5/SvelteKit 前端 + Postgres16，覆盖服务器/数据中心/供应商/域名/证书/IP/端口/端口模板/站点/数据库实例/备份计划/监控目标/用户/角色/字典/回收站/附件/审计/变更 等 19 个菜单，全量字典驱动、软删除、RBAC。

## 关键命令

```bash
# 后端（WSL 内，项目在 /mnt/c/workspace/RzOps）
cd rzops-api && cargo build --release --workspace    # 构建（glibc 动态）
cargo build --release --target x86_64-unknown-linux-musl --workspace  # 静态构建（无 glibc 依赖，Dockerfile 用此方案）
cargo run -p rzops-app                                # 运行（需 DATABASE_URL 相关 RZOPS_* 环境变量）
cargo clippy --workspace -- -D warnings               # lint

# 前端
cd rzops-web && npm ci && npm run dev                 # 开发 http://localhost:5173（/api 代理 8000）
npm run build                                          # 静态产物 build/（adapter-static SPA）
npm run check                                          # svelte-check

# 数据库（Docker 容器 rzops-postgres，库 rzopsdb，用户 rzops）
docker exec -it rzops-postgres psql -U rzops -d rzopsdb

# 一键启动（推荐）
docker compose up -d --build                           # web:8080 api:8000 db:5432
```

## 运行环境（非显然，必读）

- **开发环境在 WSL**（Ubuntu + systemd）：`rzops-api.service`、`rzops-web.service`。
- **跨盘（/mnt/c）文件变更 Vite watcher 不感知**：改前端代码后必须 `sudo systemctl restart rzops-web.service`（sudo 密码 `1`）。
- Vite 缓存目录已指到原生盘（`vite.config.ts` cacheDir），勿改回。
- 附件上传存储：`rzops-api/uploads/`（compose 中为 uploads 卷）。
- 数据库密码/JWT 密钥在 `rzops-api/.env` 与根 `.env.example`（占位）。

## 架构与数据流（读代码前先看）

- **后端 7-crate workspace**（`rzops-api/`）：`app`(入口，仅装配) → `server`(路由/State/中间件/migrations/seeder) → `api`(handlers+DTO)；`infra`(sqlx 仓储，**全项目唯一允许写 SQL 的层**) → `domain`(模型+port trait，**禁 sqlx/axum/HTTP 类型**)；`config`(环境变量)；`common`(共享错误)。依赖方向单向，违反即架构错误。
- **前端**：`src/lib/api/`（统一 client.ts：JWT 注入、401 跳转、`sanitizeEmptyRefs` 剔除空 `_id/_date/_time` 字段——**后端 Option 字段拒绝空串，提交前必须清洗**）；`src/lib/types/`（与后端 DTO 一一对应）；`src/lib/components/`（DataTable、表单、TableSelectModal 等共享组件）；页面在 `src/routes/`。
- **认证**：JWT(HS256)，登录返回 `access_token`；401 仅非 `/login` 路径跳转（防死循环）。
- **迁移**：`server/migrations/NNN_*.sql`，`sqlx::migrate!` 启动自动应用。**禁止直接改表**，只加新 migration。

## 核心约定（改代码前必读）

1. **字典驱动**：所有枚举展示（状态/类型/颜色）走 `cmdb_dict`，前端用字典 code 翻译；状态颜色存字典 `extra_data` JSON 的 `color` 键。**禁止新增硬编码枚举**。
2. **软删除**：业务表 `is_deleted` 标记 + `deleted_at`/`deleted_by`；删除=UPDATE，恢复/永久删除在回收站；审计/变更日志记 `delete_type`（soft/permanent）。
3. **关联关系用中间表**（M2M）：`cmdb_server_port_server`、`cmdb_ops_site_server/database/domain` 等；列表/详情展示关联对象一律**显示名称（名称+状态后缀），禁止显示 uuid**。
4. **大数据量选择器**：单选/多选关联对象一律用**弹出表格选择器**（TableSelectModal，远程搜索+分页+表头固定），禁止全量下拉。
5. **列表页规范**：序号列固定、表头固定、分页大小可选、列显隐、下线/退役行置底+状态色（字体颜色）；列表/筛选排序按 `created_at` 倒序、非活跃置底。
6. **表单规范**：右上角操作栏 + 底部保存统一；"编辑直达编辑页、首列/名称进详情"；时间字段校验（开始≤结束）；前端校验 + 后端校验双保险。
7. **错误处理**：前端 `request()` 统一抛 ApiError + 全局 toast；后端 `common` 定义 AppError（thiserror）→ api 层 `IntoResponse`。
8. **seed/ 目录**：历史开发脚本（`_` 前缀临时脚本用完即删，不提交）；`database/schema.sql + seed-data.sql` 是当前库的权威快照（compose 自动初始化）。

## 已知坑速查（详细见 HANDOVER §6）

| 坑 | 表现 | 解法 |
|---|---|---|
| Vite 跨盘 watcher | 改代码不生效 | 重启 rzops-web.service |
| 公网反代 | Blocked request / 无限刷新 | Vite `allowedHosts` 加域名；公网生产用**静态构建**（hmr 不参与） |
| 云端 WAF(ModSecurity) | 保存 403（PUT/含"ssh"字段被拦） | WAF 规则问题非应用缺陷，放行 PUT/调整规则 |
| 后端 Option 空串 | 保存报错 | 前端 `sanitizeEmptyRefs`（已统一） |
| 登录无限刷新 | /login 死循环 | 已修：登录页不预载需认证接口 + 401 防重入（勿回退） |

## 文档

- 交接文档（中文）：`docs/HANDOVER.md`；英文：`docs/HANDOVER.en.md`
- Rust 审计：`rzops-api/docs/AUDIT_REPORT.md`；前端审计：`rzops-web/docs/FRONTEND_AUDIT_REPORT.md`
