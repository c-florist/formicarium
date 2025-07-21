<script lang="ts">
import Button from "$lib/components/ui/Button.svelte";
import Icon from "$lib/components/ui/Icon.svelte";
import Navbar from "$lib/components/ui/Navbar.svelte";
import Popover from "$lib/components/ui/Popover.svelte";
import StatsDashboard from "$lib/components/ui/StatsDashboard.svelte";
import { uiStateStore } from "$lib/stores/ui-state-store";
import { sineInOut } from "svelte/easing";
import { slide } from "svelte/transition";

let { children } = $props();

let isStatsDashboardOpen = $state(false);

const toggleStatsDashboard = () => {
  isStatsDashboardOpen = !isStatsDashboardOpen;
};

const toggleStatsOverlay = () => {
  uiStateStore.update((state) => ({
    ...state,
    showStatsOverlay: !state.showStatsOverlay,
  }));
};
</script>

<div class="relative flex flex-col h-screen bg-stone-700">
  <Navbar>
    <Popover text="Toggle stats" position="bottom">
      <Button
        onClick={toggleStatsOverlay}
      >
        <Icon name="eye" class="w-6 h-6" />
      </Button>
    </Popover>
    <Button
      onClick={toggleStatsDashboard}
    >
      Menu
    </Button>
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
