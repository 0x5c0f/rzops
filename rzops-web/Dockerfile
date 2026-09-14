# ============================================================
# RzOps CMDB Web — 静态构建（SPA）+ nginx 托管
# ============================================================

# ---- Builder ----
FROM node:22-alpine AS builder
WORKDIR /app

# 禁用 npm 版本更新提示（打包输出保持干净，避免无关 notice 干扰告警检查）
ENV npm_config_update_notifier=false

COPY rzops-web/package.json rzops-web/package-lock.json ./
RUN npm ci

COPY rzops-web/ .
RUN npm run build

# ---- Runtime ----
FROM nginx:alpine
COPY rzops-web/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/build /usr/share/nginx/html
EXPOSE 80
