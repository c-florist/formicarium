<script lang="ts">
import InfoDashboard from "$lib/components/InfoDashboard.svelte";
import Navbar from "$lib/components/Navbar.svelte";
import { sineInOut } from "svelte/easing";
import { slide } from "svelte/transition";

let { children } = $props();

let isInfoDashboardOpen = $state(false);

function toggleInfoDashboard() {
  isInfoDashboardOpen = !isInfoDashboardOpen;
}
</script>

<div class="flex flex-col h-screen bg-stone-700">
  <Navbar>
    <button
      onclick={toggleInfoDashboard}
      class="bg-stone-600 hover:bg-stone-500 text-white font-bold py-2 px-4 border-b-4 border-stone-800 hover:border-stone-700 rounded"
    >
      Info
    </button>
  </Navbar>
  <main class="flex-1 flex">
    <div class="flex-1 relative">
      {@render children()}
    </div>

    {#if isInfoDashboardOpen}
      <div
        class="w-96 h-full"
        transition:slide={{ duration: 300, easing: sineInOut, axis: "x" }}
      >
        <InfoDashboard class="h-full" />
      </div>
    {/if}
  </main>
</div>
