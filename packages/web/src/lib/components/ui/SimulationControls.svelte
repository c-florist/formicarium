<script lang="ts">
import {
  decrementSpeed,
  incrementSpeed,
  resetSimulationSpeed,
  simulationState,
  togglePause,
} from "$lib/state/simulation.svelte";
</script>

<div
  class="absolute bottom-4 left-1/2 z-10 -translate-x-1/2 sm:left-auto sm:right-4 sm:translate-x-0"
>
  <div
    class="flex flex-col items-center gap-2 rounded-lg border border-stone-600 bg-stone-800/80 p-2 shadow-lg sm:flex-row sm:gap-4"
  >
    <div class="flex items-center gap-2">
      <span class="text-sm font-bold text-white">Speed</span>
      <button
        class="flex h-10 w-10 items-center justify-center rounded-full bg-stone-600 text-white transition-colors hover:bg-stone-500 cursor-pointer"
        onclick={decrementSpeed}
      >
        &lt;&lt;
      </button>
      <button
        class="flex h-10 w-20 items-center justify-center rounded-sm bg-stone-600 text-white transition-colors hover:bg-stone-500 cursor-pointer"
        onclick={resetSimulationSpeed}
      >
        Reset
      </button>
      <button
        class="flex h-10 w-10 items-center justify-center rounded-full bg-stone-600 text-white transition-colors hover:bg-stone-500 cursor-pointer"
        onclick={incrementSpeed}
      >
        &gt;&gt;
      </button>
    </div>
    <div class="h-px w-full border-b border-stone-600 sm:h-10 sm:w-px sm:border-l"></div>
    <div class="flex items-center gap-2">
      <span class="text-sm font-bold text-white">Simulation</span>
      <button
        class="flex h-10 w-20 items-center justify-center rounded-sm bg-stone-600 text-white transition-colors hover:bg-stone-500 cursor-pointer"
        onclick={togglePause}
      >
        {#if simulationState.isPaused}
          Resume
        {:else}
          Pause
        {/if}
      </button>
    </div>
  </div>
</div>
