<script lang="ts">
import Button from "$lib/components/ui/Button.svelte";
import Icon from "$lib/components/ui/Icon.svelte";
import Menu from "$lib/components/ui/Menu.svelte";
import Navbar from "$lib/components/ui/Navbar.svelte";
import Popover from "$lib/components/ui/Popover.svelte";
import { toggleMenu, toggleStatsOverlay, uiState } from "$lib/state/ui.svelte";
import { sineInOut } from "svelte/easing";
import { slide } from "svelte/transition";

let { children } = $props();
</script>

<div class="relative flex flex-col h-screen bg-stone-700">
  <Navbar>
    <Popover text="Show food stats" position="bottom">
      <Button
        onClick={toggleStatsOverlay}
      >
        <Icon name="eye" class="w-6 h-6" />
      </Button>
    </Popover>
    <Button
      onClick={toggleMenu}
    >
      Menu
    </Button>
  </Navbar>
  <main class="flex-1">
    {@render children()}
  </main>

  {#if uiState.menuIsOpen}
    <div
      class="absolute top-0 right-0 w-96 bottom-0 z-10"
      transition:slide={{ duration: 300, easing: sineInOut, axis: "x" }}
    >
      <Menu class="h-full" onclose={toggleMenu} />
    </div>
  {/if}
</div>
