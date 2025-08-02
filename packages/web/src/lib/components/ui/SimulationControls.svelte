<script lang="ts">
import {
  decrementSpeed,
  incrementSpeed,
  resetSpeed,
} from "$lib/state/simulation.svelte";
</script>

<div class="absolute bottom-4 right-4 flex items-center gap-2 z-10">
  <button
    class="flex h-10 w-10 items-center justify-center rounded-sm bg-stone-800 text-white hover:bg-stone-700 cursor-pointer"
    onclick={decrementSpeed}
  >
    &lt;&lt;
  </button>
  <button
    class="flex h-10 w-20 items-center justify-center rounded-sm bg-stone-800 text-white hover:bg-stone-700 cursor-pointer"
    onclick={resetSpeed}
  >
    Reset
  </button>
  <button
    class="flex h-10 w-10 items-center justify-center rounded-sm bg-stone-800 text-white hover:bg-stone-700 cursor-pointer"
    onclick={incrementSpeed}
  >
    &gt;&gt;
  </button>
</div>
