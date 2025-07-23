<script lang="ts">
import { uiState } from "$lib/state/ui.svelte";
import { worldStore } from "$lib/stores/world-store";

let selectedAnt:
  | { id: number; health: number; state: { type: string } }
  | undefined;

$: {
  if (uiState.selectedAntId !== null && $worldStore) {
    selectedAnt = $worldStore.ants.find(
      (ant) => ant.id === uiState.selectedAntId,
    );
  } else {
    selectedAnt = undefined;
  }
}
</script>

{#if selectedAnt}
  <div class="absolute bottom-4 left-4 bg-stone-800/80 text-white p-4 rounded-lg shadow-lg border border-stone-600">
    <div class="flex flex-row justify-between mb-2 pb-2 border-b border-stone-600">
      <h3 class="text-lg font-bold">Ant #{selectedAnt.id}</h3>
      <button class="text-sm text-stone-400 hover:text-white cursor-pointer" onclick={() => { uiState.selectedAntId = null }}>
        Close
      </button>
    </div>
    <div class="flex flex-col space-x-1 space-y-1">
      <div class="flex flex-row">
        <p class="text-md font-semibold">Health: <span>{selectedAnt.health}</span></p>

      </div>
      <div class="flex flex-row">
        <p class="text-md font-semibold">State: <span class="capitalize">{selectedAnt.state.type}</span></p>
      </div>
    </div>
  </div>
{/if}
