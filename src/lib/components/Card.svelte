<script lang="ts">
  import type { Collection } from "$lib/api";
  import { ui } from "$lib/ui.svelte";
  import Cover from "./Cover.svelte";

  let { item, index = 0 }: { item: Collection; index?: number } = $props();
</script>

<button
  class="card"
  style:--d="{Math.min(index, 12) * 35}ms"
  onclick={() => ui.go({ name: "collection", kind: item.kind, id: item.id })}
>
  <div class="art">
    <Cover src={item.image} size="100%" round={item.kind === "artist"} radius={12} />
  </div>
  <span class="name">{item.name}</span>
  <span class="sub">{item.subtitle}</span>
</button>

<style>
  .card {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 12px;
    border-radius: 16px;
    text-align: left;
    min-width: 0;
    transition:
      background 0.2s,
      transform 0.25s cubic-bezier(0.2, 0.8, 0.2, 1);
    animation: rise 0.5s cubic-bezier(0.2, 0.8, 0.2, 1) both;
    animation-delay: var(--d);
  }
  .card:hover {
    background: rgb(255 255 255 / 0.06);
    transform: translateY(-3px);
  }
  .art {
    aspect-ratio: 1;
    margin-bottom: 4px;
    box-shadow: 0 10px 30px -12px rgb(0 0 0 / 0.7);
    border-radius: 12px;
  }
  .art :global(.cover[style*="50%"]) {
    border-radius: 50%;
  }
  .name {
    font-weight: 600;
    color: var(--text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .sub {
    font-size: 13px;
    color: var(--text-2);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
