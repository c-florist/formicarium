<script lang="ts">
import SecondaryButton from "$lib/components/ui/SecondaryButton.svelte";
import { toggleHelpPanel, toggleWorldStatsPanel } from "$lib/state/ui.svelte";

let isOpen = false;

const toggleMenu = () => {
  isOpen = !isOpen;
};

const closeMenu = () => {
  isOpen = false;
};
</script>

<div class="relative" role="button" tabindex="0" onfocusout={closeMenu}>
  <SecondaryButton onClick={toggleMenu}>
    Menu
  </SecondaryButton>

  {#if isOpen}
    <div class="absolute right-0 mt-2 w-48 bg-stone-800 rounded-md shadow-lg z-20 border border-stone-600">
      <button
        class="block w-full text-left px-4 py-2 text-sm text-white hover:bg-stone-700 cursor-pointer"
        onclick={() => {
          toggleWorldStatsPanel();
          closeMenu();
        }}
      >
        World Stats
      </button>
      <button
        class="block w-full text-left px-4 py-2 text-sm text-white hover:bg-stone-700 cursor-pointer"
        onclick={() => {
          toggleHelpPanel();
          closeMenu();
        }}
      >
        Help
      </button>
    </div>
  {/if}
</div>
