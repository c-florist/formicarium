<script lang="ts">
import SecondaryButton from "$lib/components/ui/SecondaryButton.svelte";
import {
  openHelpPanel,
  openWorldStatsPanel,
  toggleSimulationControls,
} from "$lib/state/ui.svelte";

let isOpen = $state(false);
let container: HTMLDivElement;

const toggleMenu = () => {
  isOpen = !isOpen;
};

const closeMenu = () => {
  isOpen = false;
};

const handleFocusOut = (event: FocusEvent) => {
  // If the focus is moving to an element that is still inside our component,
  // then we don't want to close the menu.
  if (
    event.relatedTarget instanceof Node &&
    container.contains(event.relatedTarget)
  ) {
    return;
  }
  closeMenu();
};
</script>

<div
  class="relative"
  role="button"
  tabindex="-1"
  bind:this={container}
  onfocusout={handleFocusOut}
>
  <SecondaryButton onClick={toggleMenu}>
    Menu
  </SecondaryButton>

  {#if isOpen}
    <div
      class="absolute right-0 mt-2 w-48 bg-stone-800 rounded-md shadow-lg z-20 border border-stone-600"
    >
      <button
        class="block w-full text-left px-4 py-2 text-sm text-white hover:bg-stone-700 cursor-pointer"
        onclick={() => {
          toggleSimulationControls();
          closeMenu();
        }}
      >
        Speed controls
      </button>
      <button
        class="block w-full text-left px-4 py-2 text-sm text-white hover:bg-stone-700 cursor-pointer"
        onclick={() => {
          openWorldStatsPanel();
          closeMenu();
        }}
      >
        World Stats
      </button>
      <button
        class="block w-full text-left px-4 py-2 text-sm text-white hover:bg-stone-700 cursor-pointer"
        onclick={() => {
          openHelpPanel();
          closeMenu();
        }}
      >
        Help
      </button>
    </div>
  {/if}
</div>
