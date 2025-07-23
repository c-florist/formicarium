<script lang="ts">
import Button from "$lib/components/ui/Button.svelte";
import Icon from "$lib/components/ui/Icon.svelte";
import Navbar from "$lib/components/ui/Navbar.svelte";
import Popover from "$lib/components/ui/Popover.svelte";
import WorldStatsPanel from "$lib/components/ui/WorldStatsPanel.svelte";
import {
  toggleStatsOverlay,
  toggleWorldStatsPanel,
  uiState,
} from "$lib/state/ui.svelte";

let { children } = $props();
</script>

<div class="relative flex flex-col h-screen bg-stone-700">
  <Navbar>
    <Popover text="Reveal food stats" position="bottom">
      <Button
        onClick={toggleStatsOverlay}
      >
        Reveal food
      </Button>
    </Popover>
    <Popover text="Toggle stats panel" position="left">
      <Button
        onClick={toggleWorldStatsPanel}
      >
        Stats
      </Button>
    </Popover>
  </Navbar>
  <main class="flex-1">
    {@render children()}
  </main>

  {#if uiState.showWorldStatsPanel}
    <WorldStatsPanel />
  {/if}
</div>
