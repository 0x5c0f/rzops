<script lang="ts">
  import { MATRIX_GROUPS, LEVEL_ACTIONS } from '$lib/utils/perm-matrix';
  import type { MatrixGroup } from '$lib/utils/perm-matrix';
  import * as Card from '$lib/ui/card';

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

  // ── 系统权限（单权限点，显示在"查看"列） ──
  function hasSystem(key: string): boolean {
    return permissions.includes(`system:${key}`);
  }

  function toggleSystem(key: string) {
    const perm = `system:${key}`;
    onChange(hasSystem(key) ? permissions.filter((p) => p !== perm) : [...permissions, perm]);
  }
</script>

<div class="space-y-4">
  {#each MATRIX_GROUPS as group}
    <Card.Root>
      <Card.Header>
        <Card.Title class="text-sm">{group.title}</Card.Title>
      </Card.Header>
      <Card.Content class="p-0">
        <div class="overflow-x-auto">
          <table class="w-full text-sm">
            <thead>
              <tr class="border-b bg-muted/40 text-left text-muted-foreground">
                <th class="w-32 px-3 py-2 font-medium">资源</th>
                {#each LEVEL_ACTIONS as a}
                  <th class="w-16 px-3 py-2 text-center font-medium">{a.label}</th>
                {/each}
              </tr>
            </thead>
            <tbody>
              {#each group.resources as r}
                {#if !r.system}
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
                {:else}
                  {@const sysChecked = hasSystem(r.key)}
                  <tr class="border-b last:border-0 hover:bg-muted/20" class:opacity-75={!sysChecked}>
                    <td class="px-3 py-1.5 font-medium">{r.label}</td>
                    <td class="px-3 py-1.5 text-center">
                      <input
                        type="checkbox"
                        class="h-4 w-4"
                        checked={sysChecked}
                        onchange={() => toggleSystem(r.key)}
                      />
                    </td>
                    <td class="px-3 py-1.5 text-center text-muted-foreground/60">—</td>
                    <td class="px-3 py-1.5 text-center text-muted-foreground/60">—</td>
                  </tr>
                {/if}
              {/each}
            </tbody>
          </table>
        </div>
      </Card.Content>
    </Card.Root>
  {/each}
</div>
