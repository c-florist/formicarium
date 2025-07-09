<script lang="ts">
import { world } from "./lib/world-store";
</script>

<main>
  <h1>Formicarium</h1>

  {#if $world}
    <pre>{JSON.stringify($world, null, 2)}</pre>
  {:else}
    <p>Connecting to simulation...</p>
  {/if}
</main>

<style>
  main {
    font-family: sans-serif;
  }
  pre {
    color: #000;
    background-color: #f4f4f4;
    padding: 1em;
    border-radius: 5px;
  }
</style>
