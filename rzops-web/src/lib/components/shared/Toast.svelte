<script lang="ts">
  import { getToasts, dismissToast, type ToastItem } from '$lib/stores/toast.svelte';

  let toasts = $derived(getToasts());

  const typeClass: Record<ToastItem['type'], { box: string; icon: string }> = {
    error: { box: 'border-red-200 bg-red-50', icon: 'text-red-600' },
    success: { box: 'border-green-200 bg-green-50', icon: 'text-green-600' },
    info: { box: 'border-slate-200 bg-white', icon: 'text-slate-500' },
  };

  const typeIcon: Record<ToastItem['type'], string> = {
    error: '✕',
    success: '✓',
    info: 'ℹ',
  };
</script>

<!-- 全局 Toast 通知层（固定右上角，不随页面滚动） -->
<div class="pointer-events-none fixed right-4 top-4 z-[999] flex w-80 max-w-[calc(100vw-2rem)] flex-col gap-2">
  {#each toasts as toast (toast.id)}
    {@const cls = typeClass[toast.type]}
    <div
      class="pointer-events-auto flex items-start gap-2 rounded-lg border px-3 py-2.5 shadow-lg shadow-black/5 {cls.box}"
      role="alert"
    >
      <span class={`mt-0.5 text-sm font-bold ${cls.icon}`}>{typeIcon[toast.type]}</span>
      <span class="flex-1 break-words text-sm leading-relaxed text-slate-800">{toast.message}</span>
      <button
        type="button"
        aria-label="关闭"
        class="mt-0.5 cursor-pointer text-slate-400 transition-colors hover:text-slate-600"
        onclick={() => dismissToast(toast.id)}
      >
        ✕
      </button>
    </div>
  {/each}
</div>
