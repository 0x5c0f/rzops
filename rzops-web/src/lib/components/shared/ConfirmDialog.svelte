<script lang="ts">
  import * as Dialog from '$lib/ui/dialog';
  import { Button } from '$lib/ui/button';

  let {
    open = $bindable(false),
    title = '确认操作',
    description = '确定要执行此操作吗？',
    confirmLabel = '确认',
    onConfirm,
  }: {
    open?: boolean;
    title?: string;
    description?: string;
    confirmLabel?: string;
    onConfirm: () => void;
  } = $props();

  function handleConfirm() {
    onConfirm();
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>{title}</Dialog.Title>
      <Dialog.Description>{description}</Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer>
      <Button variant="outline" onclick={() => (open = false)}>取消</Button>
      <Button variant="destructive" onclick={handleConfirm}>{confirmLabel}</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
