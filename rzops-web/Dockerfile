# ============================================================
# RzOps CMDB Web — 静态构建（SPA）+ nginx 托管
# ============================================================

# ---- Builder ----
FROM node:22-alpine AS builder
WORKDIR /app

COPY rzops-web/package.json rzops-web/package-lock.json ./
RUN npm ci

COPY rzops-web/ .
RUN npm run build

# ---- Runtime ----
FROM nginx:alpine
COPY rzops-web/nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/build /usr/share/nginx/html
EXPOSE 80
