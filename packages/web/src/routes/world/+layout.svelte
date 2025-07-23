<script lang="ts">
import Button from "$lib/components/ui/Button.svelte";
import ConfirmModal from "$lib/components/ui/ConfirmModal.svelte";
import HelpPanel from "$lib/components/ui/HelpPanel.svelte";
import Icon from "$lib/components/ui/Icon.svelte";
import Navbar from "$lib/components/ui/Navbar.svelte";
import Popover from "$lib/components/ui/Popover.svelte";
import WorldStatsPanel from "$lib/components/ui/WorldStatsPanel.svelte";
import {
  hideConfirmation,
  toggleHelpPanel,
  toggleStatsOverlay,
  toggleWorldStatsPanel,
  uiState,
} from "$lib/state/ui.svelte";

let { children } = $props();
</script>

<div class="relative flex flex-col h-screen bg-stone-700">
  <Navbar>
    <div class="flex items-center space-x-4">
      <Popover text="Reveal food stats" position="bottom">
        <Button
          onClick={toggleStatsOverlay}
        >
          <Icon name="eye" class="w-6 h-6" />
        </Button>
      </Popover>
      <Popover text="Toggle stats panel" position="bottom">
        <Button
          onClick={toggleWorldStatsPanel}
        >
          Stats
        </Button>
      </Popover>
      <Popover text="Help" position="bottom">
        <Button
          onClick={toggleHelpPanel}
        >
          <Icon name="info" class="w-6 h-6" />
        </Button>
      </Popover>
    </div>
  </Navbar>
  <main class="flex-1">
    {@render children()}
  </main>

  {#if uiState.showWorldStatsPanel}
    <WorldStatsPanel />
  {/if}

  {#if uiState.showHelpPanel}
    <HelpPanel />
  {/if}

  {#if uiState.confirmation.isOpen}
    <ConfirmModal
      title={uiState.confirmation.title}
      message={uiState.confirmation.message}
      onConfirm={uiState.confirmation.onConfirm}
      onCancel={hideConfirmation}
    />
  {/if}
</div>
