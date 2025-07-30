<script lang="ts">
import { confirmAndExit } from "$lib/state/ui.svelte";
import type { Snippet } from "svelte";

let { children }: { children: Snippet } = $props();
</script>

<nav class="w-full bg-stone-800 text-white p-4 border-b-4 border-stone-900">
  <div class="flex space-x-4 justify-between items-center">
    <button onclick={confirmAndExit} class="cursor-pointer flex items-center space-x-2">
      <img src="/icon/favicon-96x96.png" alt="Formicarium logo" class="h-8 w-8" />
      <h1 class="text-2xl font-bold">Formicarium</h1>
    </button>
    <div class="flex items-center space-x-4">
      {#if children}
        {@render children()}
      {/if}
    </div>
  </div>
</nav>
