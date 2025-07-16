<script lang="ts">
import Navbar from "$lib/components/ui/Navbar.svelte";
import StatsDashboard from "$lib/components/ui/StatsDashboard.svelte";
import { sineInOut } from "svelte/easing";
import { slide } from "svelte/transition";

let { children } = $props();

let isStatsDashboardOpen = $state(false);

function toggleStatsDashboard() {
  isStatsDashboardOpen = !isStatsDashboardOpen;
}
</script>

<div class="relative flex flex-col h-screen bg-stone-700">
  <Navbar>
    <button
      onclick={toggleStatsDashboard}
      class="bg-stone-600 hover:bg-stone-500 text-white font-bold py-2 px-4 border-b-4 border-stone-800 hover:border-stone-700 rounded cursor-pointer"
    >
      Stats
    </button>
  </Navbar>
  <main class="flex-1">
    {@render children()}
  </main>

  {#if isStatsDashboardOpen}
    <div
      class="absolute top-0 right-0 w-96 bottom-0 z-10"
      transition:slide={{ duration: 300, easing: sineInOut, axis: "x" }}
    >
      <StatsDashboard class="h-full" onclose={toggleStatsDashboard} />
    </div>
  {/if}
</div>
