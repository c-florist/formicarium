<script lang="ts">
import PixelWorldCanvas from "$lib/components/PixelWorldCanvas.svelte";
import { worldStore } from "$lib/world/world-store";
</script>

<main class="relative w-screen h-screen">
  {#if $worldStore}
    <PixelWorldCanvas />
  {:else}
    <div class="flex items-center justify-center h-full">
      <div class="text-center">
        <h2 class="text-2xl font-semibold text-stone-700 mb-2">
          Connecting<span class="animated-ellipsis"></span>
        </h2>
        <p class="text-stone-500">
          Do simulated ants dream of electric sheep?
        </p>
      </div>
    </div>
  {/if}
</main>

<style>
  .animated-ellipsis::after {
    content: ".";
    animation: ellipsis 1.5s infinite;
  }

  @keyframes ellipsis {
    0% {
      content: ".";
    }
    33% {
      content: "..";
    }
    66% {
      content: "...";
    }
  }
</style>
