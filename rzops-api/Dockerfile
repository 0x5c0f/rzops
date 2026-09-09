# ============================================================
# RzOps CMDB API — 多阶段构建
# 说明：项目使用运行时 sqlx::query()（非编译期宏），
#       且 TLS 为 rustls（纯 Rust），构建无需数据库连接与 openssl。
# ============================================================

# ---- Builder ----
FROM rust:1-slim-bookworm AS builder
WORKDIR /src

# utoipa-swagger-ui 构建期需下载 swagger-ui 静态资源（curl + unzip）
RUN apt-get update && apt-get install -y --no-install-recommends curl unzip \
    && rm -rf /var/lib/apt/lists/*

# 依赖层缓存（Cargo.lock 不变时复用）
COPY rzops-api/Cargo.toml rzops-api/Cargo.lock ./
COPY rzops-api/app/Cargo.toml app/
COPY rzops-api/server/Cargo.toml server/
COPY rzops-api/api/Cargo.toml api/
COPY rzops-api/domain/Cargo.toml domain/
COPY rzops-api/infra/Cargo.toml infra/
COPY rzops-api/config/Cargo.toml config/
COPY rzops-api/common/Cargo.toml common/
RUN mkdir -p app/src server/src api/src domain/src infra/src config/src common/src \
    && echo 'fn main() {}' > app/src/main.rs \
    && for d in server api domain infra config common; do echo '' > $d/src/lib.rs; done \
    && cargo build --release --workspace 2>/dev/null || true

# 真实源码
COPY rzops-api/ .
RUN touch app/src/main.rs \
    && for d in server api domain infra config common; do touch $d/src/lib.rs; done \
    && cargo build --release --workspace

# ---- Runtime ----
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /src/target/release/rzops-app ./rzops-api
# 迁移 SQL 由 sqlx::migrate! 编译期嵌入二进制，此处保留目录便于排查
COPY rzops-api/server/migrations ./migrations
# 附件存储目录
RUN mkdir -p ./uploads

ENV RZOPS_SERVER__HOST=0.0.0.0 \
    RZOPS_SERVER__PORT=8000

EXPOSE 8000
CMD ["./rzops-api"]
