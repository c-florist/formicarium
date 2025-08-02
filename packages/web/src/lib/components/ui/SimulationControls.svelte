<script lang="ts">
import {
  decrementSpeed,
  incrementSpeed,
  resetSpeed,
} from "$lib/state/simulation.svelte";
</script>

<div class="absolute bottom-4 right-4 z-10">
  <div
    class="flex items-center gap-2 rounded-lg border border-stone-600 bg-stone-800/80 p-2 shadow-lg"
  >
    <button
      class="flex h-10 w-10 items-center justify-center rounded-full bg-stone-600 text-white transition-colors hover:bg-stone-500 cursor-pointer"
      onclick={decrementSpeed}
    >
      &lt;&lt;
    </button>
    <button
      class="flex h-10 w-20 items-center justify-center rounded-sm bg-stone-600 text-white transition-colors hover:bg-stone-500 cursor-pointer"
      onclick={resetSpeed}
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
</div>
