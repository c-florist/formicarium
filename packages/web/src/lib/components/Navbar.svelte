<script lang="ts">
import type { Snippet } from "svelte";

let { children }: { children: Snippet } = $props();
</script>

<nav class="w-full bg-stone-800 text-white p-4 border-b-4 border-stone-900">
  <div class="container mx-auto flex justify-between items-center">
    <h1 class="text-2xl font-bold">Formicarium</h1>
    {#if children}
      {@render children()}
    {/if}
  </div>
</nav>
