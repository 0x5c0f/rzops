# RzOps CMDB — Asset Management Platform

A Configuration Management Database (CMDB) for small-to-medium ops teams, covering the full lifecycle of **infrastructure, network, application, operations, RBAC and audit**. Rust backend, Svelte frontend, fully decoupled, and static-deployable.

- Backend: [Rust](https://www.rust-lang.org/) (axum + sqlx + Postgres 16)
- Frontend: [Svelte 5](https://svelte.dev/) / [SvelteKit](https://kit.svelte.dev/) (Vite 8 + TypeScript 6 + Tailwind v4)
- Deployment: `docker compose` one-command startup (recommended, dev & test unified) / pure static files (nginx / CDN)

---

## Feature Overview

| Module | Menu | Capabilities |
|---|---|---|
| 📊 Dashboard | Home | Resource stats, **expiry reminders** (server/domain/certificate renewals, retired resources auto-filtered) |
| Infrastructure | Servers | Hardware config, lease info, **inline IP/ports/database instances/site relations**, environment tagging (prod/staging/test) |
| | Data Centers | Country/address, line types (dict-driven multi-select) |
| | Providers | Provider types (incl. domain category), country, address |
| Network | Domains | Registrar, registration/expiry dates, DNS records, bound certificates |
| | Certificates | Certificate info, **domain binding** (pick from existing domains), expiry reminders |
| | Server IPs | Multiple IPs, NIC name, ISP provider, status linked to server |
| | Server Ports | Protocol/port/service, **per-server ports + batch add from port templates** |
| Applications | Sites | Site info, **server/database/domain relations** (1-to-many), backup & monitoring linkage |
| | Database Instances | Type/version, linked server, site relations |
| Operations | Backup Plans | Associated targets (database instances/sites), many-to-one |
| | Monitor Targets | Associated targets + site, monitoring method |
| Admin | Users | RBAC users, **one user with multiple roles** |
| | Roles | Business resource perms + system admin perms, **progressive permission matrix** (view→create→edit→delete) |
| | Dicts | **Everything enum is dictionary-driven**; dropdowns & badge colors served by dict entries with extensible attributes |
| | Recycle Bin | **Soft-delete** system: view deleted snapshot, restore, permanent delete |
| | Attachments | Real file upload (link to any target type), uploader auto-recorded |
| Audit | Audit Logs / Change Records | Full operation trail, resource-name linkage, distinguishes soft vs permanent delete |

**Global list capabilities**: advanced filters (per-menu fields), column show/hide, sticky header & index column, configurable page size, responsive layout (PC/tablet/phone), cell truncation tooltip, offline/retired status styling & sorting, modal delete confirmation, frontend form validation, global error toasts.

---

## Tech Stack

```
Backend  Rust 2021 edition · axum 0.8 · sqlx 0.8 (runtime-tokio + tls-rustls) ·
         tokio 1 · tower-http · Postgres 16 · JWT (HS256)
         7-crate workspace: app / server / api / domain / infra / config / common
Frontend Svelte 5.56 (runes) · SvelteKit 2.63 · Vite 8 · TypeScript 6 (strict) ·
         Tailwind v4 · bits-ui (shadcn-svelte style) · lucide icons
Deploy   Docker (compose: postgres / api / web, **dev & test unified**) ·
         nginx static hosting (SPA fallback + /api proxy)
```

---

## Quick Start (Recommended: Docker Compose)

Prerequisite: [Docker](https://www.docker.com/) with Compose.

```bash
# 1. Clone and enter the repo
git clone <repo-url> && cd RzOps

# 2. (Optional) Adjust environment variables
cp .env.example .env

# 3. Build and start (first Rust build takes ~5–10 min)
docker compose up -d --build
```

| Service | URL | Notes |
|---|---|---|
| Web | http://localhost:8080 | SPA, static hosting + API proxy |
| API | http://localhost:8000/api/v1 | directly testable |
| Postgres | localhost:5432 | database `rzopsdb` |

> **Port conflicts**: if 8000/5432/8080 are already taken on your machine, override `API_PORT` / `POSTGRES_PORT` / `WEB_PORT` in `.env` and restart with `docker compose up -d` (`.env` is not committed; `.env.example` is the template).

**Default account**: `admin@rzops.local` / `admin123` (superuser, auto-created by the backend seeder on first boot; override via `RZOPS_SEED_ADMIN_EMAIL` / `RZOPS_SEED_ADMIN_PASSWORD`).

Database init is automated from the `database/` directory: `schema.sql` (standard SQL DDL) + `seed-data.sql` (only base data — dicts / roles / role permissions, INSERT syntax; accounts are created by the seeder). No sqlx migration mechanism is used; business data (servers, domains, etc.) starts empty and is entered through the UI.

> Optional: `database/test-data.sql` holds realistic sample data (test users, business records, audit/change logs) and is **not** executed on first boot. For manual testing/demo:
> `docker exec -i rzops-db-1 psql -U rzops -d rzopsdb < database/test-data.sql` (idempotent — truncates business tables first).

---

## Manual Deployment (without Docker)

### 1. Database

```bash
docker run -d --name rzops-postgres -e POSTGRES_USER=rzops -e POSTGRES_PASSWORD=rzops \
  -e POSTGRES_DB=rzopsdb -p 5432:5432 postgres:16-alpine

docker exec -i rzops-postgres psql -U rzops -d rzopsdb < database/schema.sql
docker exec -i rzops-postgres psql -U rzops -d rzopsdb < database/seed-data.sql

# Optional: load realistic test data (not executed automatically on first boot)
docker exec -i rzops-postgres psql -U rzops -d rzopsdb < database/test-data.sql
```

### 2. Backend (Rust)

```bash
cd rzops-api
export RZOPS_DATABASE__HOST=localhost RZOPS_DATABASE__USER=rzops RZOPS_DATABASE__PASSWORD=rzops \
       RZOPS_DATABASE__NAME=rzopsdb RZOPS_JWT__SECRET=your-random-secret
cargo run --release -p rzops-app
```

> The schema comes from `database/schema.sql` (standard SQL). The API only runs seeders on startup (admin account, base dicts) — **no migrations are executed**.

**Cross-platform static build** (optional): produces a pure static binary (musl, no glibc dependency) runnable on any Linux distribution:

```bash
rustup target add x86_64-unknown-linux-musl
sudo apt install musl-tools          # Debian/Ubuntu; install the equivalent musl toolchain elsewhere
cargo build --release --target x86_64-unknown-linux-musl --workspace
# artifact: target/x86_64-unknown-linux-musl/release/rzops-app (ldd: statically linked)
```

### 3. Frontend (dev mode)

```bash
cd rzops-web
npm install
npm run dev        # http://localhost:5173, /api proxied to 8000
```

> When accessing the dev server through a public-domain reverse proxy / tunnel, set `RZOPS_PUBLIC_HOST` (e.g. `cmdb.example.com`) in `rzops-web/.env` (template `rzops-web/.env.example`): vite automatically allows that host (including subdomains) and disables HMR to avoid infinite page reloads when the proxy does not forward WebSocket.

### 4. Frontend (production static)

```bash
cd rzops-web
npm ci && npm run build    # output in build/
```

Serve `build/` on any static server with **SPA fallback** (non-file requests → `index.html`) and `/api` proxied to the backend. See `rzops-web/nginx.conf` for a complete reference.

---

## Environment Variables

Copy `.env.example` to `.env`. Key items:

| Variable | Default | Notes |
|---|---|---|
| `POSTGRES_USER/PASSWORD/DB` | `rzops/rzops/rzopsdb` | compose database |
| `RZOPS_JWT__SECRET` | `dev-only-secret-change-me` | **must change in production**, HS256 signing key |
| `RZOPS_SEED_ADMIN_EMAIL` | `admin@rzops.local` | superuser email auto-created on first boot |
| `RZOPS_SEED_ADMIN_PASSWORD` | `admin123` | superuser password |
| `RZOPS_JWT__EXPIRATION_SECONDS` | `86400` | token lifetime |
| `RZOPS_PUBLIC_HOST` | (empty) | dev-only, for public reverse-proxy dev access (see §3); compose web is a static build and unaffected |

Full backend vars: `rzops-api/.env.example`.

---

## Directory Layout

```
RzOps/
├── rzops-api/            # Rust backend (7-crate workspace)
│   ├── app/              #   entry (main.rs, wiring only)
│   ├── server/           #   router, AppState, middleware, seeder
│   ├── api/              #   HTTP handlers + DTOs
│   ├── domain/           #   domain models & ports (pure, zero I/O deps)
│   ├── infra/            #   sqlx repository impls (the only layer touching SQL)
│   ├── config/           #   env config
│   ├── common/           #   shared errors & types
│   ├── docs/             #   Rust audit reports (AUDIT_REPORT.md / AUDIT_LOG.md)
│   └── uploads/          #   attachment file storage
├── rzops-web/            # Svelte frontend
│   ├── src/routes/       #   page routes (68 pages)
│   ├── src/lib/api/      #   API client (unified JWT / errors / field sanitize)
│   ├── src/lib/types/    #   type mirror (mapped to backend DTOs)
│   ├── src/lib/components/ # shared components (DataTable / forms / pickers…)
│   └── docs/             #   frontend audit (FRONTEND_AUDIT_REPORT.md)
├── database/             # Schema (schema.sql) + base data (seed-data.sql), auto-init for compose; test-data.sql (optional sample data, manual)
├── seed/                 # historical dev seed scripts (usage documented in HANDOVER)
├── docker-compose.yml    # one-command startup
└── .env.example          # env template
```

---

## Documentation Index

| Doc | Purpose |
|---|---|
| **[docs/HANDOVER.md](docs/HANDOVER.md)** | ⭐ Handover (Chinese): full features, architecture, database, **pitfall log**, RBAC/dict/log systems, ops, leftovers & roadmap |
| **[docs/HANDOVER.en.md](docs/HANDOVER.en.md)** | Handover (English) |
| **[AGENTS.md](AGENTS.md)** | AI-collaboration cheat sheet: architecture one-liner, commands, conventions, pitfall quick ref (auto-loaded by AI sessions) |
| `rzops-api/docs/AUDIT_REPORT.md` | Rust architecture/code-quality audit |
| `rzops-web/docs/FRONTEND_AUDIT_REPORT.md` | Frontend standards audit |

---

## Development Conventions (quick)

- **Backend**: layered dependency direction (app→server→api→domain; server/infra→domain; all→common); `domain` must not use sqlx/axum; SQL only in `infra`; no `unwrap()` in production; typed errors via `thiserror`.
- **Frontend**: Svelte 5 runes (`$state`/`$props`/`$derived`) only, no legacy Svelte 4 syntax; all requests through `src/lib/api/`; typed props required; no `any`.
- **Database**: every change as a new migration (`server/migrations/NNN_*.sql`); business deletes are always **soft deletes** (`is_deleted` flag).

## License

MIT
