import adapterStatic from '@sveltejs/adapter-static';
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig, loadEnv } from 'vite';

export default defineConfig(({ mode }) => {
	// 公网访问模式：设置环境变量 RZOPS_PUBLIC_HOST（如 cmdb.example.com）时，
	// HMR WebSocket 走公网 wss 域名，避免反代/穿透下反复刷新。
	const env = loadEnv(mode, process.cwd(), '');
	const publicHost = env.RZOPS_PUBLIC_HOST || '';

	// 允许的 Host 白名单：本地开发固定放行 localhost/127.0.0.1；
	// 公网穿透/反代域名由 RZOPS_PUBLIC_HOST 提供（含其子域通配），不再硬编码具体域名。
	const allowedHosts = publicHost
		? ['localhost', '127.0.0.1', publicHost, `.${publicHost}`]
		: ['localhost', '127.0.0.1'];

	return {
		plugins: [
			tailwindcss(),
			sveltekit({
				compilerOptions: {
					runes: ({ filename }) =>
						filename.split(/[/\\]/).includes('node_modules') ? undefined : true
				},
				// 静态导出（SPA 模式）：所有路由回退到 index.html，由前端路由接管。
			// 产物在 build/ 目录，可部署到任意静态服务器（nginx / OSS / CDN）。
			adapter: adapterStatic({ fallback: 'index.html' })
			})
		],
		server: {
			// 允许通过内网穿透 / 公网域名 / nginx 反代访问（开发环境）
			host: true,
			// 预热常用模块：登录、布局与主要列表页在服务就绪时即预转换，降低首个页面访问延迟
			warmup: {
				clientFiles: [
					'./src/routes/+layout.svelte',
					'./src/routes/login/+page.svelte',
					'./src/routes/servers/+page.svelte',
					'./src/routes/dicts/+page.svelte',
					'./src/lib/components/layout/Sidebar.svelte',
				]
			},
			allowedHosts,
			proxy: {
				'/api': {
					// 开发代理默认指向本机后端（compose 模式 API 端口 8000；如需改端口在 compose 环境变量调整）
					target: 'http://localhost:8000',
					changeOrigin: true,
				}
			},
			// 公网模式下：若 nginx 未转发 WebSocket，HMR 连接失败会导致页面反复刷新，
			// 故默认禁用 HMR；如需热更新，在 nginx 配置 WebSocket 升级后设为 wss 模式。
			...((publicHost
				? {
						hmr: false
					}
				: {}) as object)
		},
		// 生产构建预览（公网验证走 preview 最稳定：无动态编译 / 无 HMR）
		preview: {
			host: true,
			port: 5173,
			allowedHosts,
			proxy: {
				'/api': {
					target: 'http://localhost:8000',
					changeOrigin: true,
				}
			}
		}
	};
});
