
<script lang="ts">
import { uiState } from "$lib/state/ui.svelte";
</script>

<div class="absolute top-24 right-4 bg-stone-800/80 text-white p-4 rounded-lg shadow-lg border border-stone-600 w-64">
  <div class="flex flex-row justify-between mb-2 pb-2 border-b border-stone-600">
    <h3 class="text-lg font-bold">Help</h3>
    <button class="text-sm text-stone-400 hover:text-white cursor-pointer" on:click={() => { uiState.showHelpPanel = false }}>
      Close
    </button>
  </div>
  <div class="space-y-4">
    <div>
      <h4 class="font-semibold mb-1">Ant Stats</h4>
      <p class="text-sm">Click on any ant to see its current health and state in the bottom-left panel.</p>
    </div>
    <div>
      <h4 class="font-semibold mb-1">Food Stats</h4>
      <p class="text-sm">Toggle the visibility of food source amounts using the <span class="font-bold">eye icon</span> in the top bar.</p>
    </div>
    <div>
      <h4 class="font-semibold mb-1">World Stats</h4>
      <p class="text-sm">View overall simulation statistics by clicking the <span class="font-bold">"World Stats"</span> button.</p>
    </div>
  </div>
</div>
