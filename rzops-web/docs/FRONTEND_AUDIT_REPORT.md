# 前端架构与规范审计报告

| Field | Value |
|---|---|
| **项目** | RzOps CMDB Web（rzops-web） |
| **审计日期** | 2026-09-09 |
| **标准** | Svelte 5 runes 官方规范 + SvelteKit 2 官方文档 + ai-dev-audit §E |
| **技术栈** | Svelte 5.56 / SvelteKit 2.63 / Vite 8 / TypeScript 6（strict）/ Tailwind v4 / bits-ui 2.18 |
| **模式** | SPA（API 后端分离，JWT + localStorage） |
| **结构** | 68 页面 / 24 API 模块 / 22 类型模块 / 34 组件 / 17.4k 行 |

---

## Executive Summary

**Overall Health: 🟢 Healthy**

前端代码整体非常规范：runes 语法完全一致、fetch 全部收敛于 API 层、类型注解 100%、零 `any`、无残留调试代码、`svelte-check` 零错误。仅有 1 项 Medium（错误反馈缺失）与 1 项 Low（`$app/stores` 旧 API）需要跟进，另有 2 个观察项。

| Category | Status |
|---|---|
| E1 runes 一致性（Svelte 5 语法） | 🟢 通过 |
| E2 fetch 收敛（API 包装层） | 🟢 通过 |
| E3 API 包装层存在性 | 🟢 通过 |
| E4 类型镜像层 | 🟢 通过 |
| E5 TypeScript `any` | 🟢 通过（0 处） |
| E6 环境变量作用域 | 🟢 通过（未使用 `$env`） |
| 官方规范符合度（props/load/state） | 🟢 基本通过，2 项跟进 |

---

## Issues

### 🟡 Medium

#### F1: 错误反馈缺失——145 处 catch 全部静默

- **Location:** `src/routes/**`、`src/lib/components/**`（145 处 `catch`）
- **官方规范依据:** SvelteKit 交互应以用户可感知的方式呈现结果/错误；`client.ts` 已抛出结构化的 `ApiError`，但调用方仅 `console.error` 后静默。
- **现状:**
  ```ts
  } catch (err) {
    console.error('Failed to save server:', err);  // 用户无任何提示
  }
  ```
- **Why it matters:** 用户操作（保存/删除/加载）失败时页面无反馈，只能开控制台才看到错误。此问题用户此前实际遇到（"保存失败 ApiError: 未知错误"）。
- **Recommended response:** 在 `client.ts` 或全局布局引入统一错误提示（toast/alert），catch 处调用展示 `err.message`；删除/保存等高频操作至少展示错误文案。
- **Effort:** S（约 1 个共享组件 + 批量替换）

### 🔵 Low

#### F2: `$app/stores` 旧 API 未迁移

- **Location:** 10+ 文件（Sidebar、+layout、+error、各详情页等）
- **官方规范依据:** SvelteKit 2.12 起推荐 `$app/state`（`page` 为响应式 proxy，无需 `$` 前缀访问）；`$app/stores` 仍受支持但属于旧 API。
- **Why it matters:** 功能无影响；新项目规范角度建议渐进迁移，未来若移除旧 API 时避免集中改造。
- **Recommended response:** 低优先级渐进迁移：`import { page } from '$app/state'`，访问 `page.params` / `page.data`（去掉 `$` 前缀）。
- **Effort:** M（10+ 文件，机械替换）

---

## Passed Checks

- ✅ **E1 runes 一致性**：0 处 `export let` / `$:` / `on:` 旧式事件 / `createEventDispatcher` / `$$props` / 旧 `<slot>`——全仓 100% Svelte 5 runes 写法
- ✅ **E2 fetch 收敛**：所有 `fetch(` 均位于 `src/lib/api/`，组件/路由零直接 fetch
- ✅ **E3 API 包装层**：`client.ts` 统一处理 JWT 注入、401 自动登出、错误解析（ApiError）、空引用字段清洗（sanitizeEmptyRefs）、204 响应
- ✅ **E4 类型镜像层**：`src/lib/types/` 22 个模块与后端 DTO 一一对应
- ✅ **E5 零 `any`**：全仓无 `: any` / `as any`
- ✅ **E6 环境变量**：无 `$env` 使用，无 VITE_ 泄露面
- ✅ **props 类型注解**：34/34 组件 100% 带类型注解（含内联 `}: {...} = $props()` 形式）
- ✅ **props 变异**：无 `ownership_invalid_mutation` 违规模式
- ✅ **调试残留**：0 console.log / debugger / TODO / FIXME
- ✅ **依赖方向**：`src/lib` 不反向依赖 `src/routes`
- ✅ **静态检查**：`svelte-check` 0 error（TS strict）
- ✅ **目录组织**：`api/ components/ stores/ types/ ui/ utils/` 分层清晰，符合 SvelteKit `$lib` 约定

---

## Observations（观察项，不在检查清单内）

1. **SPA 取数模式**：107 处 `onMount` 取数、0 个 `load` 函数。对「JWT + 内部工具」场景是合理设计（无需 SSR/SEO）；代价是每次进入页面先闪 loading。若未来追求首屏体验或 SSR，可逐步将列表页迁到 `+page.js` 的 `load`（官方推荐数据获取位置）。
2. **核心组件职责**：`DataTable.svelte`（419 行）承载列配置/排序/分页/列显隐/tooltip 等多职责，功能正常但可考虑拆分；`+page.svelte`（dashboard，425 行）为最大页面，可拆子组件。
3. **form actions 未使用**：项目用 API 客户端（`api.post/put`）而非 SvelteKit form actions。与「前后端分离 API 应用」架构一致，官方文档亦认可该模式（"Alternatives" 章节），非违规。

---

## 结论

前端代码质量优秀，接近官方规范基线。建议优先处理 F1（错误反馈，影响实际用户体验），F2 可随日常改动渐进迁移。无需结构性改造。

*Report generated per Svelte 5 / SvelteKit 2 official docs (svelte.dev) + ai-dev-audit §E checklist.*
