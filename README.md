# RzOps CMDB 运维资产管理平台

面向中小型运维团队的资产管理（CMDB）系统，覆盖**基础设施、网络、应用、运维、权限与审计**全链路。后端 Rust 编写、前端 Svelte 编写，前后端完全分离，支持静态化部署。

- 后端：[Rust](https://www.rust-lang.org/)（axum + sqlx + Postgres 16）
- 前端：[Svelte 5](https://svelte.dev/) / [SvelteKit](https://kit.svelte.dev/)（Vite 8 + TypeScript 6 + Tailwind v4）
- 部署：`docker compose` 一键启动（推荐，开发/测试统一） / 纯静态文件（nginx / CDN）

---

## 功能一览

| 模块 | 菜单 | 能力 |
|---|---|---|
| 📊 Dashboard | 首页 | 资源统计、**到期提醒**（服务器/域名/证书续费，自动过滤已退役资源） |
| 基础设施 | 服务器 | 硬件配置、租赁信息、**IP/端口/数据库实例/关联站点内联录入**、环境标注（生产/测试/预发布） |
| | 数据中心 | 国家/地址、线路类型（字典多选） |
| | 供应商 | 供应商类型（含域名分类）、国家、地址 |
| 网络 | 域名 | 注册商、注册/到期日期、DNS 记录、绑定证书 |
| | 证书 | 证书信息、**绑定域名**（选择已录入域名）、到期提醒 |
| | 服务器IP | 多 IP 维护、网卡名称、ISP 供应商、状态联动服务器 |
| | 服务器端口 | 协议/端口/服务名、**每服务器独立端口 + 端口模板批量添加** |
| 应用 | 站点 | 站点信息、**关联服务器/数据库/域名**（一对多）、备份/监控联动 |
| | 数据库实例 | 类型/版本、关联服务器、站点关联 |
| 运维 | 备份计划 | 关联目标（数据库实例/站点）多对一 |
| | 监控目标 | 关联目标 + 站点、监控方式 |
| 管理 | 用户管理 | RBAC 用户、**一人多角色** |
| | 角色管理 | 业务资源权限 + 系统管理权限，**递进式权限矩阵**（查看→新建→编辑→删除） |
| | 字典管理 | **全系统枚举字典化**，前端下拉/徽章颜色由字典驱动，支持扩展属性 |
| | 回收站 | **软删除**体系，查看删除快照、恢复、永久删除 |
| | 附件 | 真实文件上传（关联任意目标类型）、上传者自动记录 |
| 审计 | 审计日志 / 变更记录 | 全操作留痕，资源名联动展示，区分临时删除/永久删除 |

**全局通用能力**：高级筛选（每菜单差异化字段）、列显示/隐藏、固定表头与序号列、分页大小可调、响应式布局（PC/平板/手机）、单元格截断 tooltip、下线/退役资源状态标注与排序、删除确认 Modal、前端表单校验、全局错误 Toast。

---

## 技术栈

```
后端    Rust 2021 edition · axum 0.8 · sqlx 0.8（runtime-tokio + tls-rustls）·
        tokio 1 · tower-http · Postgres 16 · JWT(HS256)
        7-crate workspace: app / server / api / domain / infra / config / common
前端    Svelte 5.56（runes）· SvelteKit 2.63 · Vite 8 · TypeScript 6（strict）·
        Tailwind v4 · bits-ui（shadcn-svelte 风格）· lucide 图标
部署    Docker（compose 三服务：postgres / api / web，**开发与测试统一走 compose**）·
        nginx 静态托管（SPA fallback + /api 反代）
```

---

## 快速启动（推荐：Docker Compose）

前置：安装 [Docker](https://www.docker.com/)（含 Compose）。

```bash
# 1. 克隆仓库并进入根目录
git clone <repo-url> && cd RzOps

# 2. （可选）按需修改环境变量
cp .env.example .env

# 3. 一键构建并启动（首次构建 Rust 约 5~10 分钟）
docker compose up -d --build
```

启动完成后：

| 服务 | 地址 | 说明 |
|---|---|---|
| Web 前端 | http://localhost:8080 | SPA，静态托管 + API 反代 |
| API | http://localhost:8000/api/v1 | 可直接调试 |
| Postgres | localhost:5432 | 库名 `rzopsdb` |

> **端口冲突**：若本机 8000/5432/8080 已被占用（如已有开发服务），可在 `.env` 中覆盖 `API_PORT` / `POSTGRES_PORT` / `WEB_PORT` 后重启：`docker compose up -d`（`.env` 不入库，`.env.example` 为模板）。

**默认账号**：`admin@rzops.local` / `admin123`（超级管理员，由后端 seeder 首次启动自动创建，可通过环境变量 `RZOPS_SEED_ADMIN_EMAIL` / `RZOPS_SEED_ADMIN_PASSWORD` 修改）。

数据库初始化由 `database/` 目录自动完成：`schema.sql`（标准 SQL 建表）+ `seed-data.sql`（仅基础数据：字典/角色/角色权限，INSERT 语法；账号由后端 seeder 创建）。不使用 sqlx 迁移机制；业务数据（服务器/域名等）首启为空，由用户在前端录入。

> 可选：`database/test-data.sql` 为模拟真实环境的测试数据（含测试用户、业务样例与审计/变更记录），**不会**随首次启动自动执行；如需人工复测/演示，手动执行：
> `docker exec -i rzops-db-1 psql -U rzops -d rzopsdb < database/test-data.sql`（可重复执行，自带清空业务数据）。

---

## 手动部署（不使用 Docker）

### 1. 数据库

```bash
docker run -d --name rzops-postgres -e POSTGRES_USER=rzops -e POSTGRES_PASSWORD=rzops \
  -e POSTGRES_DB=rzopsdb -p 5432:5432 postgres:16-alpine

# 初始化表结构与基础数据
docker exec -i rzops-postgres psql -U rzops -d rzopsdb < database/schema.sql
docker exec -i rzops-postgres psql -U rzops -d rzopsdb < database/seed-data.sql

# 可选：加载模拟真实环境的测试数据（不随首次启动自动执行）
docker exec -i rzops-postgres psql -U rzops -d rzopsdb < database/test-data.sql
```

### 2. 后端（Rust）

```bash
cd rzops-api
export RZOPS_DATABASE__HOST=localhost RZOPS_DATABASE__USER=rzops RZOPS_DATABASE__PASSWORD=rzops \
       RZOPS_DATABASE__NAME=rzopsdb RZOPS_JWT__SECRET=your-random-secret
cargo run --release -p rzops-app   # 或 cargo build --release --workspace 后运行 target/release/rzops-app
```

> 表结构由 `database/schema.sql` 标准 SQL 建好；API 启动仅做 seeder（创建 admin 账号、补齐字典），**不执行迁移**。

**跨平台静态构建**（可选）：产物为纯静态二进制（musl，无 glibc 依赖），可在任意 Linux 发行版直接运行：

```bash
rustup target add x86_64-unknown-linux-musl
sudo apt install musl-tools          # Debian/Ubuntu；其他系统装对应 musl 工具链
cargo build --release --target x86_64-unknown-linux-musl --workspace
# 产物：target/x86_64-unknown-linux-musl/release/rzops-app（ldd 显示 statically linked）
```

### 3. 前端（开发模式）

```bash
cd rzops-web
npm install
npm run dev        # http://localhost:5173，/api 自动代理到 8000
```

### 4. 前端（生产静态文件）

```bash
cd rzops-web
npm ci && npm run build    # 产物在 build/
```

将 `build/` 部署到任意静态服务器，需配置 **SPA fallback**（非文件请求返回 `index.html`）并将 `/api` 反代到后端。参考 `rzops-web/nginx.conf`（已含完整配置）。

---

## 环境变量

复制 `.env.example` 为 `.env` 后修改。关键项：

| 变量 | 默认值 | 说明 |
|---|---|---|
| `POSTGRES_USER/PASSWORD/DB` | `rzops/rzops/rzopsdb` | compose 数据库 |
| `RZOPS_JWT__SECRET` | `dev-only-secret-change-me` | **生产必改**，JWT HS256 签名密钥 |
| `RZOPS_SEED_ADMIN_EMAIL` | `admin@rzops.local` | 首次启动自动创建的超级管理员邮箱 |
| `RZOPS_SEED_ADMIN_PASSWORD` | `admin123` | 超级管理员密码 |
| `RZOPS_JWT__EXPIRATION_SECONDS` | `86400` | Token 有效期 |

后端完整配置项见 `rzops-api/.env.example`。

---

## 目录结构

```
RzOps/
├── rzops-api/            # Rust 后端（7-crate workspace）
│   ├── app/              #   入口（main.rs，仅装配）
│   ├── server/           #   路由、AppState、中间件、migrations、seeder
│   ├── api/              #   HTTP handlers + DTO
│   ├── domain/           #   领域模型与端口（纯业务，零 I/O 依赖）
│   ├── infra/            #   sqlx 仓储实现（唯一触碰 SQL 的层）
│   ├── config/           #   环境变量配置
│   ├── common/           #   共享错误与类型
│   ├── docs/             #   Rust 审计报告（AUDIT_REPORT.md / AUDIT_LOG.md）
│   └── uploads/          #   附件文件存储
├── rzops-web/            # Svelte 前端
│   ├── src/routes/       #   页面路由（68 页）
│   ├── src/lib/api/      #   API 客户端（统一 JWT/错误/字段清洗）
│   ├── src/lib/types/    #   类型镜像（与后端 DTO 对应）
│   ├── src/lib/components/ # 共享组件（DataTable / 表单 / 选择器…）
│   └── docs/             #   前端审计报告（FRONTEND_AUDIT_REPORT.md）
├── database/             # 数据库：schema.sql + seed-data.sql（compose 自动初始化）；test-data.sql（可选测试数据，手动执行）
├── seed/                 # 历史开发期种子脚本（SQL + 工具脚本，文档化用途见 HANDOVER）
├── docker-compose.yml    # 一键启动示例
└── .env.example          # 环境变量模板
```

---

## 文档索引

| 文档 | 用途 |
|---|---|
| **[docs/HANDOVER.md](docs/HANDOVER.md)** | ⭐ 交接文档（中文）：完整功能、架构、数据库、**踩坑记录**、权限/字典/日志体系、部署运维、遗留与规划 |
| **[docs/HANDOVER.en.md](docs/HANDOVER.en.md)** | Handover (English) |
| **[AGENTS.md](AGENTS.md)** | AI 协作速查：架构一句话、命令、约定、坑速查（AI 会话自动加载） |
| `rzops-api/docs/AUDIT_REPORT.md` | Rust 架构/规范审计报告 |
| `rzops-web/docs/FRONTEND_AUDIT_REPORT.md` | 前端规范审计报告 |

---

## 开发约定速览

- **后端**：分层依赖单向（app→server→api→domain，server/infra→domain，全部→common）；`domain` 禁用 sqlx/axum；SQL 只允许出现在 `infra`；禁止 `unwrap()` 于生产代码；错误用 `thiserror`。
- **前端**：Svelte 5 runes（`$state`/`$props`/`$derived`），禁用 Svelte 4 旧语法；数据请求一律走 `src/lib/api/`；组件 props 必须类型注解；禁 `any`。
- **数据库**：所有变更以新增 migration（`server/migrations/NNN_*.sql`）方式演进，禁止直接改表结构；业务删除一律**软删除**（`is_deleted` 标记）。

## License

MIT
