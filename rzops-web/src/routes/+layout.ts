// SPA 模式：关闭服务端渲染与预渲染，所有路由由前端接管。
// 数据全部在客户端通过 API 拉取（JWT 存 localStorage），
// 静态导出（adapter-static + fallback: index.html）依赖此配置。
export const ssr = false;
export const prerender = false;
