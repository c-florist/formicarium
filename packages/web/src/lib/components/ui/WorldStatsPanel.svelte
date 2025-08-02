<script lang="ts">
import { uiState } from "$lib/state/ui.svelte";
import { worldStore } from "$lib/stores/world";
</script>

<div class="absolute top-24 right-4 bg-stone-800/80 text-white p-4 rounded-lg shadow-lg border border-stone-600 w-64">
  <div class="flex flex-row justify-between mb-2 pb-2 border-b border-stone-600">
    <h3 class="text-lg font-bold">World Stats</h3>
    <button class="text-sm text-stone-400 hover:text-white cursor-pointer" onclick={() => { uiState.showWorldStatsPanel = false }}>
      Close
    </button>
  </div>
  {#if $worldStore}
    <div class="space-y-2 mb-4">
      <h4 class="text-md font-bold mr-4">
        Alive ants: <span class="font-normal">{$worldStore.stats.aliveAnts}</span>
      </h4>
      <h4 class="text-md font-bold mr-4">
        Dead ants: <span class="font-normal">{$worldStore.stats.deadAnts}</span>
      </h4>
    </div>
    <div class="space-y-2">
      <h4 class="text-md font-bold mr-4">
        Food sources: <span class="font-normal">{$worldStore.stats.foodSourceCount}</span>
      </h4>
      <h4 class="text-md font-bold mr-4">
        Food in nest: <span class="font-normal">{$worldStore.stats.foodInNest}</span>
      </h4>
    </div>
  {:else}
    <p class="text-stone-400">Waiting for simulation data...</p>
  {/if}
</div>
