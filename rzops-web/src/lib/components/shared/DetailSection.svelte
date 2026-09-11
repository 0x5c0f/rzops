<script lang="ts">
  import * as Card from '$lib/ui/card';
  import type { Snippet } from 'svelte';

  let {
    title,
    children,
  }: {
    title: string;
    children: Snippet;
  } = $props();
</script>

<Card.Root>
  <Card.Header>
    <Card.Title>{title}</Card.Title>
  </Card.Header>
  <Card.Content>
    <dl class="divide-y">
      {@render children()}
    </dl>
  </Card.Content>
</Card.Root>
