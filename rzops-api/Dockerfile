# ============================================================
# RzOps CMDB API — 多阶段构建（musl 静态链接）
# 产物为纯静态二进制（ldd: statically linked），无 glibc 依赖，
# 可在任意 Linux 发行版 / alpine / scratch 容器直接运行。
# 说明：项目使用运行时 sqlx::query()（非编译期宏），TLS 为 rustls，
#       构建无需数据库连接与 openssl。
# ============================================================

# ---- Builder（alpine 默认 musl 工具链）----
FROM rust:1-alpine AS builder
WORKDIR /src

# utoipa-swagger-ui 构建期需下载 swagger-ui 静态资源（curl + unzip）
RUN apk add --no-cache musl-dev curl unzip

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
    && cargo build --release --target x86_64-unknown-linux-musl --workspace 2>/dev/null || true

# 真实源码
COPY rzops-api/ .
RUN touch app/src/main.rs \
    && for d in server api domain infra config common; do touch $d/src/lib.rs; done \
    && cargo build --release --target x86_64-unknown-linux-musl --workspace

# ---- Runtime（alpine：仅 ca-certificates + tzdata）----
FROM alpine:3.20
RUN apk add --no-cache ca-certificates tzdata
WORKDIR /app

COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/rzops-app ./rzops-api
# 迁移 SQL 由 sqlx::migrate! 编译期嵌入二进制，此处保留目录便于排查
COPY rzops-api/server/migrations ./migrations
# 附件存储目录
RUN mkdir -p ./uploads

ENV RZOPS_SERVER__HOST=0.0.0.0 \
    RZOPS_SERVER__PORT=8000

EXPOSE 8000
CMD ["./rzops-api"]
