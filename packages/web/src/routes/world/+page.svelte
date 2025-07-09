<script lang="ts">
  import WorldCanvas from "$lib/WorldCanvas.svelte";
  import { worldStore } from "$lib/world-store";
</script>

<main class="flex items-center justify-center w-full h-screen p-4 bg-amber-50">
  {#if $worldStore}
    <WorldCanvas />
  {:else}
    <div class="text-center">
      <h2 class="text-2xl font-semibold text-stone-700 mb-2">Connecting to Simulation...</h2>
      <p class="text-stone-500">
        The ants might be sleeping.
      </p>
    </div>
  {/if}
</main>
