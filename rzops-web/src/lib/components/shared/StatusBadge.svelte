<script lang="ts">
  import { Badge } from '$lib/ui/badge';
  import { getStatusBadgeClass, getStatusLabel } from '$lib/utils/resource-status';

  let { status }: { status: string } = $props();

  let badgeClass = $derived(getStatusBadgeClass(status) || 'bg-muted text-muted-foreground border-transparent');
  let label = $derived(getStatusLabel(status) || status);
</script>

<Badge class={badgeClass}>
  {label}
</Badge>
