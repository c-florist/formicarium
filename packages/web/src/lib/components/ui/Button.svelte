<script lang="ts">
import type { Snippet } from "svelte";

let {
  class: className = "",
  onClick,
  children,
}: { class?: string; onClick: () => void; children?: Snippet } = $props();
</script>

<button onclick={onClick} class={`bg-stone-600 hover:bg-stone-500 text-white font-bold py-2 px-4 border-b-4 border-stone-800 hover:border-stone-700 rounded-sm transition-all duration-150 ease-in-out active:translate-y-0.5 cursor-pointer ${className}`} aria-label="button">
  {#if children}
    {@render children()}
  {/if}
</button>
