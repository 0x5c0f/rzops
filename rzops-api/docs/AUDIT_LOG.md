# Audit Log — rzops-api

This file records audit history for organizational memory. Each entry enables cross-audit comparison (recurring/new/resolved issues).

---

## 2026-06-17 — Initial Audit

| Field | Value |
|---|---|
| **Project** | rzops-api |
| **Mode** | Multi-Crate Workspace |
| **Health** | 🔴 Significant Issues |
| **Critical** | 1 (S3 — SQL injection, 13 files) |
| **High** | 3 (B1×2, S4) |
| **Medium** | 1 (C8 — no tests) |
| **Low** | 0 |
| **Notes** | First audit. Project in active development. |

### Findings

| ID | Severity | Summary | File(s) |
|---|---|---|---|
| S3 | 🔴 Critical | SQL injection via `format!("...='{}'", user_input)` in 13 repo files | `infra/src/db/repositories/*_repo.rs` |
| B1 | 🟠 High | `app → infra` forbidden dependency | `app/Cargo.toml` |
| B1 | 🟠 High | `api → infra` forbidden dependency | `api/Cargo.toml` |
| S4 | 🟠 High | Only `auth_handlers.rs` uses `AuthUser`; all other endpoints unprotected | `api/src/routes/` |
| C8 | 🟡 Medium | No tests found anywhere in codebase | Project-wide |

### Notes

- **`Claims` struct has `Serialize`/`Deserialize` in domain**: `domain/src/models/claims.rs` derives `Serialize` and `Deserialize`. This is acceptable because `Claims` is a JWT token structure (not a domain entity), and the derives are required by `jsonwebtoken` for encoding/decoding. The alternative (moving `Claims` to `infra`) would prevent the `TokenService` trait in `domain/ports/` from referencing it.

### Gotcha Validation

- **S3 false positive pattern discovered:** `format!("SELECT {} FROM ...", COLS)` where `COLS` is a constant — NOT a SQL injection risk. The script flagged these alongside real injection patterns. Future audits should distinguish between:
  - Column list interpolation with constants → safe, ignore
  - User input interpolation (`status`, `q`, `resource_type`, etc.) → real injection
- **C4 false positive avoided:** Domain enums derive `Deserialize` but are used for DB storage (sqlx), not HTTP request parsing. API has separate DTOs. Correctly identified as acceptable per gotchas.

---

## 2026-06-17 — S3 Residual Fix Verified

| ID | Status | Note |
|---|---|---|
| S3 | ✅ Resolved | `domain_repo.rs` ILIKE injection fixed — zero remaining SQL injection patterns |
| C8 | ⚠️ Recurring | No tests found yet |

---

## 2026-06-17 — Second Audit (Post-fix)

| Field | Value |
|---|---|
| **Project** | rzops-api |
| **Mode** | Multi-Crate Workspace |
| **Health** | 🟡 Needs Attention |
| **Critical** | 0 |
| **High** | 1 (S3 — domain_repo.rs residual) |
| **Medium** | 1 (C8 — no tests) |
| **Low** | 0 |
| **Notes** | Most previous findings resolved. New capabilities added (OpenAPI, TokenService trait). |

### Findings

| ID | Severity | Summary | File(s) | Status |
|---|---|---|---|---|
| S3 | 🟠 High | 2 remaining ILIKE string interpolation in domain_repo.rs | `infra/src/db/repositories/domain_repo.rs:80,95` | Recurring (mostly fixed) |
| C8 | 🟡 Medium | No tests found | Project-wide | Recurring |

### Resolved from Previous Audit

| ID | Previous Severity | Resolution |
|---|---|---|
| S3 | 🔴 Critical (13 files) | Fixed in 12/13 files; 1 file remains |
| B1 (app→infra) | 🟠 High | Removed from app/Cargo.toml |
| B1 (api→infra) | 🟠 High | Removed from api/Cargo.toml |
| S4 | 🟠 High | AuthUser added to all 18 route handlers |
| .gitignore | Observation | Created with proper exclusions |

### New Capabilities Detected

- OpenAPI/Swagger UI integration (`api/src/openapi.rs`, utoipa)
- `TokenService` trait in domain ports (port/adapter pattern)
- Database connection pool configuration (`PgPoolOptions`)
- Unified error response (`api/src/error_response.rs`)

---

## 2026-06-17 — Remediation of Initial Audit Findings

| Field | Value |
|---|---|
| **Scope** | S3, S4, B1 (×2) |
| **Health** | 🟡 Improved (was 🔴) |
| **Resolved** | 4 of 5 issues |

### Changes Made

| ID | Resolution |
|---|---|
| S3 | Replaced `format!("...='{}'", user_input)` with `$N` + `.bind()` in all 13 repo files. Also fixed bind-order bug in `server_repo.rs::find_all_with_uuid_filter`. Fixed missed `domain_repo.rs` ILIKE injection (2 occurrences). |
| S4 | Added `_auth: AuthUser` extractor to all 83 handler functions across 17 handler files (auth_handlers excluded). |
| B1 (app→infra) | Removed `rzops-infra` from `app/Cargo.toml`. Inlined `create_pool` in `app/src/main.rs`. |
| B1 (api→infra) | Created `TokenService` trait in `domain/ports/`, implemented for `JwtService` in `infra`. Replaced `Arc<JwtService>` with `Arc<dyn TokenService>` in `api` crate. Removed `rzops-infra` from `api/Cargo.toml`. |
| .gitignore | Created `.gitignore` with `.env`, `target/`, IDE files. Updated `.env.example` with JWT and seed admin vars. |

### Remaining

| ID | Issue | Notes |
|---|---|---|
| C8 | No tests | Deferred to next sprint |

---

## 2026-09-09 — Third Audit

| Field | Value |
|---|---|
| **Project** | rzops-api |
| **Mode** | Multi-Crate Workspace |
| **Health** | 🔴 Significant Issues |
| **Critical** | 1（B1 domain→sqlx） |
| **High** | 4（S3×2、B1 api→sqlx、C8 升级） |
| **Medium** | 2（C5、B1 app→sqlx） |
| **Low** | 1（C2 expect） |
| **Notes** | 用户要求按 ai-dev-discipline + ai-dev-audit 复核。历史 S3 清零后被新增代码回退。 |

### Findings

| ID | Severity | Summary | File(s) | Status |
|---|---|---|---|---|
| B1 | 🔴 Critical | domain 端口 trait 直接使用 sqlx::Error（10+ 端口） | domain/src/ports/*.rs | New（历史漏检） |
| S3 | 🟠 High | recycle 搜索词手工转义拼接 ILIKE | pi/src/routes/recycle_handlers.rs:104 | Recurring（新文件引入） |
| S3 | 🟠 High | user_repo 搜索词手工转义拼接 ILIKE | infra/src/db/repositories/user_repo.rs:84 | Recurring（新文件引入） |
| B1 | 🟠 High | api 层 10 文件直连 sqlx 内联 SQL，绕过 domain 端口 | pi/src/routes/*_handlers.rs | New |
| C8 | 🟠 High | 全仓无测试（3 连复发，按 gotchas 从 Medium 升级） | Project-wide | Recurring（升级） |
| C5 | 🟡 Medium | main.rs 35 行 + seeder.rs 业务逻辑驻留 app | pp/src/main.rs, pp/src/seeder.rs | New |
| B1 | 🟡 Medium | app 依赖 sqlx（pool/migrate/seeder 在 app） | pp/Cargo.toml | New |
| C2 | 🔵 Low | 常量表模式匹配 expect | pi/src/resource_names.rs:55 | New（Low） |

### Resolved / Passed from Previous Audit

| ID | Previous Severity | Status |
|---|---|---|
| S3（12/13 repo 旧点） | 🔴 Critical | ✅ 仍修复（本次为新增文件复发，非旧点回退） |
| B1（app→infra、api→infra 旧点） | 🟠 High | ✅ 未回退（本次新发现为 domain→sqlx 与 api→sqlx 直连，属不同违反面） |
| S4（auth 覆盖） | 🟠 High | ✅ 保持（authz 中间件存在） |
| C8 | 🟡 Medium | ⚠️ Recurring 且升级 High |

### Gotcha Validation

- **S3 模式区分再次验证**：quality-scan 报 40 处 S3，仅 2 处为真实用户输入插值；其余 38 处为 COLS/SELECT_COLS/RESOURCE_TABLE 常量白名单 +  绑定 —— 按 gotchas 判为可接受（false positive）。
- **C4 判误报**：claims.rs（JWT 载荷）与 change_type.rs（存储枚举）的 Deserialize 非 HTTP 请求体解析，符合 gotchas