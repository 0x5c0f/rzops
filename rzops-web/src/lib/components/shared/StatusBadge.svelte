<script lang="ts">
  import { Badge } from '$lib/ui/badge';
  import { getStatusBadgeClass, getColorBadgeClass, getStatusLabel } from '$lib/utils/resource-status';

  let {
    status,
    /** 可选：显式指定徽章颜色值（如来自字典 extra_data.color，如 'green'），优先于静态映射 */
    color,
    /** 可选：显式指定显示标签（如来自字典 dict_label），优先于静态映射 */
    label,
  }: {
    status: string;
    color?: string | null;
    label?: string;
  } = $props();

  let badgeClass = $derived(getColorBadgeClass(color) || getStatusBadgeClass(status) || 'bg-muted text-muted-foreground border-transparent');
  let displayLabel = $derived(label || getStatusLabel(status) || status);
</script>

<Badge class={badgeClass}>
  {displayLabel}
</Badge>
