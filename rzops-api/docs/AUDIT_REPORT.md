# Architecture & Quality Audit Report

| Field | Value |
|---|---|
| **Project** | rzops-api |
| **Audit Date** | 2026-09-09 |
| **Auditor** | ai-dev-audit |
| **Standard** | ai-dev-discipline v1 |
| **Rust Edition** | 2021（workspace.package edition = "2021"，rust-toolchain.toml 存在） |
| **Mode** | Multi-Crate Workspace（7 crate：app/server/api/domain/infra/config/common） |
| **Tools Run** | rg 14.1.0, cargo check（1.98.0）, cargo clippy 0.1.98 |

---

## Executive Summary

**Overall Health:** 🔴 Significant Issues

| Category | Status | Issues |
|---|---|---|
| A. Workspace Structure | 🟢 | 0 |
| B. Dependency Direction | 🔴 | 1（Critical） |
| S. Security | 🟡 | 2（High） |
| C. Code Quality | 🟡 | 3（1 High + 2 Medium） |
| D. Module Organization | 🟢 | 0 |
| E. Frontend (SvelteKit) | N/A | N/A |

**Issue Count**

| Severity | Count |
|---|---|
| 🔴 Critical | 1 |
| 🟠 High | 4 |
| 🟡 Medium | 2 |
| 🔵 Low | 1 |
| **Total** | **8** |

---

## Adversarial Audit Pass

**Inputs reviewed:** quality-scan.sh JSONL（41 行）、security-scan.sh 输出、detect-structure.sh 输出、各 crate Cargo.toml 依赖、domain/src 与 api/src 手工抽查、历史 AUDIT_REPORT/AUDIT_LOG（2026-06-17）、audit-checklist.md、gotchas.md
**Result:** Material gaps found（历史审计漏检 domain→sqlx 与 api→sqlx 分层问题）

### Added or Reclassified Findings

| ID | Action | Severity | Location | Evidence |
|---|---|---|---|---|
| B1(domain→sqlx) | Added | Critical | `domain/src/ports/*.rs`（10+ 文件） | 端口 trait 方法签名返回 `Result<..., sqlx::Error>` |
| S3 (api 层) | Added | High | `api/src/routes/recycle_handlers.rs:104` | 搜索词 q 手工 `''` 转义后直接拼 ILIKE |
| S3 (infra 层) | Added | High | `infra/src/db/repositories/user_repo.rs:84` | 搜索词 q 手工 `''` 转义后直接拼 ILIKE |
| B1(api→sqlx) | Added | High | `api/src/routes/*.rs`（10 文件）+ `api/Cargo.toml` | api 直接依赖 sqlx 并内联 SQL，绕过 domain port |
| C5 (app/seeder) | Added | Medium | `app/src/main.rs`（35 行）+ `app/src/seeder.rs` | main.rs 直连 pool/migrate；seeder 业务逻辑驻留 app |
| C2 (expect) | Reclassified | Low | `api/src/resource_names.rs:55` | `.expect("known resource type")` 为常量表模式匹配，无运行时风险；原判 High 过高 |
| C4 (Deserialize) | False positive | — | `domain/src/models/claims.rs`、`enums/change_type.rs` | JWT 载荷/存储反序列化，非 HTTP 请求体；按 gotchas 不构成违规 |
| S3 (其余 38 处 format!) | False positive | — | 各 repo COLS/SELECT_COLS、RESOURCE_TABLE 白名单 | 插值均为代码常量，值走 $1 绑定；按 gotchas 可接受 |

### Pass Coverage

- A1/A2/A3/A4/A5：workspace 清单、7 crate、workspace.dependencies、无多余 crate
- B1（其余边）：api→domain/common；infra→domain；server→api/infra/config/domain；config→common；common 零依赖 —— 均合规
- C1：无 unwrap（非测试）
- C3：domain 无 anyhow（用 thiserror）
- C6：AppState 全 `Arc<dyn Trait>`（18 个 repo）
- C9：无 println/dbg/eprintln
- D1–D5：domain(models/ports/enums)、api(routes/dto)、server(middleware)、infra(db/repositories)
- S1：无 unsafe；S2：无硬编码密钥；S4：authz 中间件 + AuthUser 提取器；S6：clippy 干净；S7：cargo check 通过

### Assumptions

- 本次审计范围限于 Rust 后端（用户明确指定）；前端 rzops-web 不在 scope（E 系列 N/A）
- `rust_edition=2021` 取自 workspace.package（技能 config.json 的 project 字段为 null，未臆造）
- S3 判定的"可接受"仅针对代码常量插值 + 参数绑定的模式；凡用户输入进入 SQL 字符串的均视为真实发现

---

## Issues

### 🔴 Critical

#### B1: domain 端口 trait 泄漏 sqlx（domain → sqlx 禁止依赖）

- **Location:** `domain/src/ports/user_repository.rs:8` 起，`domain/src/ports/*.rs`（10+ 个 Repository trait）
- **Rule:** §B Forbidden Matrix — `domain` 禁止依赖外部 I/O crate（axum/sqlx/tower/hyper）；rust-arch 明确"domain errors must NOT depend on infra crates like sqlx. Use a plain String or a domain-level wrapper"
- **Finding:**
  ```rust
  // domain/src/ports/user_repository.rs
  async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error>;
  ```
  所有端口 trait 的错误类型直接使用 `sqlx::Error`，使 domain 在编译期耦合 sqlx crate。
- **Why it matters:** 领域层无法独立编译/测试；api 或上层无法为端口提供非 sqlx 实现（mock 困难）；违反端口-适配器模式的根本目的（领域不感知基础设施）。这正是 C8"无测试"难以落地的结构性原因之一。
- **Recommended response:**
  1. 在 `domain` 定义 thiserror 领域错误（如 `RepositoryError::NotFound/Database(String)`）
  2. 端口 trait 返回 `Result<_, RepositoryError>`；`infra` 层 `impl From<sqlx::Error> for RepositoryError` 映射
  3. 移除 `domain/Cargo.toml` 的 `sqlx` 依赖
- **Effort:** M（2–4h，涉及 10+ 端口签名 + infra 映射）

### 🟠 High

#### S3: 用户搜索词手工转义后拼接 SQL（recycle）

- **Location:** `api/src/routes/recycle_handlers.rs:104`
- **Rule:** S3 — 所有 DB 查询使用 sqlx 宏或 $1 参数
- **Finding:**
  ```rust
  let esc = q.replace('\'', "''");
  conds.push(format!("name ILIKE '%{}%'", esc));
  ```
  `q` 为请求参数（用户输入），经 `''` 转义后直接嵌入 SQL 字符串。
- **Why it matters:** 非参数化拼接是注入反模式。`''` 转义在 PG 默认配置下可阻止经典单引号闭合，**实际可利用性较低**，但 LIKE 模式中 `%`/`_`/`\` 通配语义未转义（功能缺陷），且手工转义易被后续修改破坏。安全底线：任何用户输入不得进入 SQL 字符串。
- **Recommended response:** 改为 `sqlx::query("... name ILIKE $1 ...").bind(format!("%{}%", q))`；LIKE 通配符如需字面匹配，用 `ESCAPE` 子句。
- **Effort:** S（30min–2h）

#### S3: 用户搜索词手工转义后拼接 SQL（user_repo）

- **Location:** `infra/src/db/repositories/user_repo.rs:84`
- **Rule:** S3
- **Finding:**
  ```rust
  where_clause.push_str(&format!(
      " AND (email ILIKE '%{}%' OR full_name ILIKE '%{}%')",
      keyword.replace('\'', "''"),
      keyword.replace('\'', "''")
  ));
  ```
- **Why it matters:** 同上。用户列表搜索词进入 SQL 字符串。
- **Recommended response:** 参数化 `bind(format!("%{}%", keyword))` ×2。
- **Effort:** S

#### B1(分层): api 层直连 sqlx 内联 SQL，绕过 domain 端口

- **Location:** `api/src/routes/{recycle,backup_plan,monitor_target,attachment,audit_log,change_record,server_ip,database_instance}_handlers.rs` + `resource_names.rs`（10 文件）+ `api/Cargo.toml`
- **Rule:** rust-arch — "api receives domain services through Axum State — never imports from infra"；"All SQL lives in infra — never leaks into domain or api"
- **Finding:** api 层直接 `use sqlx::PgPool`，通过 `Extension(pool): Extension<PgPool>` 在 handler 内联 SQL（如 `resolve_target_name` 直查 cmdb_server/cmdb_database_instance/cmdb_ops_site）。SQL 职责泄漏出 infra。
- **Why it matters:** 与 S3 注入点直接相关（recycle 的注入在 api 层）；SQL 分散导致审计面扩大；跳过 domain 端口使领域约束（如软删除过滤）难以统一强制。
- **Recommended response:** 将 api 内联查询收敛到对应 infra repository 方法（或 domain 端口扩展方法），handler 只经 `State<Arc<dyn Repo>>` 取数；移除 api 对 sqlx 的依赖。
- **Effort:** M（2–4h）

#### C8: 全仓无测试（3 连审计复发，升级）

- **Location:** Project-wide
- **Rule:** C8 — Tests exist（≥1 测试块或 tests/ 目录）
- **Finding:** 无 `#[cfg(test)]`、无 `tests/` 目录。2026-06-17 两次审计均为 Medium 且未解决，本次为第 3 次复现，按 gotchas"3+ 连续审计同严重度升级一级"规则升级为 High。
- **Why it matters:** 系统已进入真实数据运维阶段（用户人工复测依赖浏览器手工验证），无自动化回归护栏；domain 耦合 sqlx 进一步抬高测试成本。
- **Recommended response:** 优先为 domain 端口/错误映射与字典缓存等纯逻辑补单测；关键 API（auth、server CRUD）补集成测试。
- **Effort:** L（4–8h）

### 🟡 Medium

#### C5: app crate 混入初始化与种子业务逻辑

- **Location:** `app/src/main.rs`（35 行）+ `app/src/seeder.rs`（新增）
- **Rule:** C5 — main.rs ≤ 30 行、仅装配；app 零业务逻辑
- **Finding:** main.rs 直接创建 `PgPool`、执行 `sqlx::migrate!`、调用 `seeder::seed_admin_user/seed_dicts`；seeder.rs 含 admin 用户与字典种子数据逻辑。规范做法是 `server::AppState::build(&config)` 封装 pool/migrate，种子逻辑归 infra 或独立初始化工具。
- **Why it matters:** app 职责漂移；seeder 逻辑不可测试；main.rs 超过 30 行指导线（35 行，虽仍属装配性质，gotchas 认可 35–40 行纯装配为可接受范围——按规范计 Medium）。
- **Recommended response:** 将 pool/migrate 移入 `AppState::build`；seeder 移到 infra 或 `--seed` feature 开关。
- **Effort:** S

#### B1(边界): app 依赖 sqlx

- **Location:** `app/Cargo.toml`
- **Rule:** rust-arch — app 仅依赖 config/server/common
- **Finding:** `app/Cargo.toml` 含 `sqlx` 依赖（main.rs/seeder.rs 使用）。非 §B 矩阵明确禁止项，但违背 app 装配层职责。
- **Why it matters:** 与 C5 同根因（pool/migrate/seeder 在 app）。修复 C5 后该依赖自然移除。
- **Recommended response:** 随 C5 一并收敛。
- **Effort:** S

### 🔵 Low

#### C2: 常量表模式匹配 expect

- **Location:** `api/src/resource_names.rs:55`
- **Rule:** C2 — expect 仅限 startup
- **Finding:** `RESOURCE_TYPES.iter().find(...).expect("known resource type")` —— 在编译期常量表中查找，命中失败在逻辑上不可能。
- **Why it matters:** 无运行时风险；仅为规则严格性记录。可改为 `if let Some(...)` 消除 expect。
- **Recommended response:** 改为模式匹配解构，避免 expect。
- **Effort:** XS

---

## Risk Priority Plan

Ordered by: severity first, then confidence and blast radius.

| Priority | ID | Title | Recommended response | Notes |
|---|---|---|---|---|
| 1 | B1(domain→sqlx) | domain 端口泄漏 sqlx | 领域错误化 + 移除 sqlx 依赖 | 架构根因，阻塞测试与 mock |
| 2 | S3 (recycle) | 搜索词拼接 SQL | 参数化 bind | api 层真实注入点 |
| 3 | S3 (user_repo) | 搜索词拼接 SQL | 参数化 bind | infra 层真实注入点 |
| 4 | B1(api→sqlx) | api 层内联 SQL | 收敛到 infra repo | 与 S3 同源 |
| 5 | C8 | 无测试 | 先补 domain 单测 | 3 连复发，已升级 |
| 6 | C5/B1(app→sqlx) | app 初始化漂移 | AppState::build + seeder 迁移 | 职责收敛 |
| 7 | C2 | expect | 模式匹配消除 | 清理 |

### Suggested Triage Allocation

**Immediate owner review:**
- B1(domain→sqlx) — 架构不变量，影响后续所有测试与扩展
- S3 ×2 — 安全底线，虽转义兜底但须参数化

**Planned remediation:**
- B1(api→sqlx)、C8

**Track and revisit:**
- C5/B1(app→sqlx)、C2

---

## Passed Checks

✅ **A1**: `[workspace]` + resolver 2 + 7 members
✅ **A2**: 各 crate 位于 workspace 根（rust-arch 简化布局认可）
✅ **A3**: app/server/api/domain/infra/config/common 7 crate 齐全
✅ **A4**: `[workspace.dependencies]` 集中锁版本
✅ **A5**: 无无法解释的 crate
✅ **B1(其余边)**: infra→domain、server→api/infra/config/domain、api→domain/common、config→common、common 零内部依赖
✅ **C1**: 非测试代码无 unwrap
✅ **C3**: domain 使用 thiserror，无 anyhow
✅ **C6**: AppState 18 个 repo 全为 `Arc<dyn Trait>`
✅ **C9**: 无 println/dbg/eprintln（tracing 正确使用）
✅ **C10**: 无 TODO/FIXME/HACK
✅ **D1**: domain 按 models/enums/ports 组织（rust-arch 模板）
✅ **D2**: api 有 routes/ + dto/
✅ **D3**: domain 有 ports/ 目录存放 trait
✅ **D4**: server 有 middleware/（authz.rs 权限中间件）
✅ **D5**: infra 有 db/repositories/ 分组
✅ **S1**: 零 unsafe
✅ **S2**: 源码无硬编码密钥（.env 已被 .gitignore 排除）
✅ **S4**: JWT auth + authz 中间件存在，handler 使用 AuthUser 提取器
✅ **S6**: cargo clippy 干净（0 warnings/errors）
✅ **S7**: cargo check 编译通过（workspace）

---

## Skipped / Not Applicable

- **S5**: `cargo-audit` 未安装。建议 `cargo install cargo-audit` 后部署前复查依赖 CVE（历史未装，维持 skip）。
- **E1–E6**: 前端在 `rzops-web/`（workspace 外），本次审计范围为用户指定的 Rust 后端。

---

## Observations

1. **历史 S3 修复被新代码回退**：2026-06-17 曾将 13 个 repo 的注入清零并验证，但后续迭代新增的 recycle_handlers（回收站）与 user_repo（用户搜索）重新引入了"用户输入拼 SQL + 手工转义"模式。**结论：需要把"S3 参数化"作为代码评审硬门槛，而非一次性修复。**

2. **api 层 `Extension(pool)` 模式**：多个 handler 通过 axum Extension 注入 `PgPool` 直查——这是 SQL 泄漏到 api 的载体。若短期无法全量收敛，至少将 `resolve_target_name` 这类名称解析统一到 infra（该函数是 3 个 handler 复制的模式）。

3. **domain 仍保留 `enums/change_type.rs`**（ChangeType 枚举 + Deserialize）：与既定"枚举→字典"演进方向相关的历史遗留。ChangeType 为变更日志内部类型，保留代码枚举可接受，但需与字典策略对齐后决定去留。

4. **app/seeder.rs 硬编码种子**（admin 用户、字典）：生产环境安全性存疑（默认管理员凭据来源），建议改为 env 注入或一次性迁移脚本，避免随二进制分发。

5. **无 CI/CD**：S6/S7 仅本地验证。建议加 GitHub Actions（cargo check + clippy + test + audit），把审计脚本的 C1/C2/S1/S2/S3 模式检查纳入 pre-merge。

6. **rust-toolchain.toml 存在**：锁定了工具链版本，利于复现构建 —— 良好实践。

---

*Report generated by ai-dev-audit. Standards: ai-dev-discipline v1.*
*File: docs/AUDIT_REPORT.md*
