<script lang="ts">
  import { MATRIX_GROUPS, LEVEL_ACTIONS } from '$lib/utils/perm-matrix';
  import type { MatrixGroup } from '$lib/utils/perm-matrix';

  let {
    permissions,
    onChange,
    compact = false,
  }: {
    /** 当前权限点集合（受控） */
    permissions: string[];
    /** 权限点变化回调 */
    onChange: (perms: string[]) => void;
    /** 紧凑模式（用于弹窗等小空间） */
    compact?: boolean;
  } = $props();

  // ── 业务资源操作 ──
  function resourceLevel(resource: string): number {
    // 判断当前权限点集合中该资源的实际最高档位
    let level = 0;
    for (const { level: lv, key } of LEVEL_ACTIONS) {
      if (permissions.includes(`${resource}:${key}`)) level = Math.max(level, lv);
    }
    return level;
  }

  function setResource(resource: string, level: number) {
    const rest = permissions.filter((p) => {
      const i = p.lastIndexOf(':');
      if (i <= 0) return true;
      const r = p.slice(0, i);
      const a = p.slice(i + 1);
      if (r !== resource) return true;
      return !['read', 'create', 'update', 'delete'].includes(a);
    });
    if (level <= 0) {
      onChange(rest);
      return;
    }
    const perms: string[] = [];
    for (const { level: lv, key } of LEVEL_ACTIONS) {
      if (lv <= level) perms.push(`${resource}:${key}`);
    }
    onChange([...rest, ...perms]);
  }

  // ── 系统权限 ──
  function hasSystem(key: string): boolean {
    return permissions.includes(`system:${key}`);
  }

  function toggleSystem(key: string) {
    const perm = `system:${key}`;
    onChange(hasSystem(key) ? permissions.filter((p) => p !== perm) : [...permissions, perm]);
  }

  // 分组内是否还有业务资源 / 系统资源
  function hasBusiness(g: MatrixGroup): boolean {
    return g.resources.some((r) => !r.system);
  }
  function hasSystemGroup(g: MatrixGroup): boolean {
    return g.resources.some((r) => r.system);
  }
</script>

<div class="space-y-4">
  {#each MATRIX_GROUPS as group}
    <div class="rounded-md border">
      <div class="border-b bg-muted/40 px-3 py-2 text-sm font-medium">{group.title}</div>

      {#if hasBusiness(group)}
        <div class="overflow-x-auto">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b text-left text-muted-foreground">
                <th class="px-3 py-2 font-medium">资源</th>
                {#each LEVEL_ACTIONS as a}
                  <th class="px-3 py-2 text-center font-medium">{a.label}</th>
                {/each}
              </tr>
            </thead>
            <tbody>
              {#each group.resources.filter((r) => !r.system) as r}
                {@const level = resourceLevel(r.key)}
                <tr class="border-b last:border-0 hover:bg-muted/20">
                  <td class="px-3 py-1.5 font-medium">{r.label}</td>
                  {#each LEVEL_ACTIONS as a}
                    <td class="px-3 py-1.5 text-center">
                      <input
                        type="checkbox"
                        class="h-4 w-4"
                        checked={level >= a.level}
                        disabled={level > a.level}
                        onchange={(e) => setResource(r.key, (e.target as HTMLInputElement).checked ? a.level : a.level - 1)}
                      />
                    </td>
                  {/each}
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}

      {#if hasSystemGroup(group)}
        <div class="flex flex-wrap gap-3 p-3">
          {#each group.resources.filter((r) => r.system) as s}
            <label
              class="flex cursor-pointer items-center gap-1.5 rounded-full border px-3 py-1 text-sm transition-colors"
              class:border-primary={hasSystem(s.key)}
              style={hasSystem(s.key) ? 'background-color: hsl(var(--primary) / 0.1)' : ''}
            >
              <input
                type="checkbox"
                class="h-3.5 w-3.5"
                checked={hasSystem(s.key)}
                onchange={() => toggleSystem(s.key)}
              />
              {s.label}
            </label>
          {/each}
        </div>
      {/if}
    </div>
  {/each}
</div>
