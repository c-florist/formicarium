<script lang="ts">
import { deselectAnt, uiStateStore } from "$lib/stores/ui-state-store";
import { worldStore } from "$lib/stores/world-store";

let selectedAnt:
  | { id: number; health: number; state: { type: string } }
  | undefined;

$: {
  if ($uiStateStore.selectedAntId !== null && $worldStore) {
    selectedAnt = $worldStore.ants.find(
      (ant) => ant.id === $uiStateStore.selectedAntId,
    );
  } else {
    selectedAnt = undefined;
  }
}
</script>

{#if selectedAnt}
  <div class="absolute bottom-4 left-4 bg-stone-800/80 text-white p-4 rounded-lg shadow-lg border border-stone-600">
    <h3 class="text-lg font-bold mb-2 border-b border-stone-600 pb-1">Ant #{selectedAnt.id}</h3>
    <div class="grid grid-cols-2 gap-x-4 gap-y-1">
      <span class="font-semibold">Health:</span>
      <span>{selectedAnt.health}</span>
      <span class="font-semibold">State:</span>
      <span class="capitalize">{selectedAnt.state.type}</span>
    </div>
    <button class="mt-4 text-sm text-stone-400 hover:text-white" on:click={() => deselectAnt()}>
      Clear selection
    </button>
  </div>
{:else}
  <div class="absolute bottom-4 left-4 bg-stone-800/80 text-white p-4 rounded-lg shadow-lg border border-stone-600">
    <p class="text-stone-400">Click an ant to see its stats.</p>
  </div>
{/if}
