import adapter from '@sveltejs/adapter-auto';
import { sveltekit } from '@sveltejs/kit/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig, loadEnv } from 'vite';

export default defineConfig(({ mode }) => {
	// 公网访问模式：设置环境变量 RZOPS_PUBLIC_HOST（如 cmdb.0x5c0f.cc）时，
	// HMR WebSocket 走公网 wss 域名，避免反代/穿透下反复刷新。
	const env = loadEnv(mode, process.cwd(), '');
	const publicHost = env.RZOPS_PUBLIC_HOST || '';

	return {
		// 依赖预构建缓存放到 WSL 原生盘（ext4）——项目位于 /mnt/c（9P 跨盘 IO 慢），
		// 将 node_modules/.vite 的依赖扫描/预构建产物移出慢盘，可显著缩短冷启动时间。
		cacheDir: '/home/chenxiaodong/.cache/rzops-vite',
		plugins: [
			tailwindcss(),
			sveltekit({
				compilerOptions: {
					runes: ({ filename }) =>
						filename.split(/[/\\]/).includes('node_modules') ? undefined : true
				},
				adapter: adapter()
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
			allowedHosts: [
				'localhost',
				'127.0.0.1',
				'cmdb.0x5c0f.cc',
				'.0x5c0f.cc',
			],
			proxy: {
				'/api': {
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
			allowedHosts: [
				'localhost',
				'127.0.0.1',
				'cmdb.0x5c0f.cc',
				'.0x5c0f.cc',
			],
			proxy: {
				'/api': {
					target: 'http://localhost:8000',
					changeOrigin: true,
				}
			}
		}
	};
});
