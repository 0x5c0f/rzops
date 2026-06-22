<script lang="ts">
  import { Badge } from '$lib/ui/badge';
  import { cn } from '$lib/utils';

  let { status }: { status: string } = $props();

  const statusConfig: Record<string, { label: string; variant: 'default' | 'secondary' | 'destructive' | 'outline' }> = {
    active: { label: '活跃', variant: 'default' },
    inactive: { label: '停用', variant: 'secondary' },
    draft: { label: '草稿', variant: 'outline' },
    archived: { label: '已归档', variant: 'secondary' },
    retired: { label: '已退役', variant: 'destructive' },
    expired: { label: '已过期', variant: 'destructive' },
    expiring: { label: '即将过期', variant: 'outline' },
    enabled: { label: '已启用', variant: 'default' },
    disabled: { label: '已禁用', variant: 'destructive' },
    reserved: { label: '已预留', variant: 'secondary' },
    temporary_offline: { label: '临时下线', variant: 'outline' },
    permanent_offline: { label: '永久下线', variant: 'destructive' },
  };

  let config = $derived(statusConfig[status] || { label: status, variant: 'outline' });
</script>

<Badge variant={config.variant}>
  {config.label}
</Badge>
