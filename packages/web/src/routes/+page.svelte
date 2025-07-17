<script lang="ts">
import { goto } from "$app/navigation";
import { simulationState } from "$lib/stores/simulation-store";

const startSimulation = () => {
  simulationState.set({ isRunning: true });
  goto("/world");
};
</script>

<main class="flex flex-col items-center justify-center min-h-screen bg-amber-50 text-center p-4">
  <h1 class="text-5xl font-bold text-stone-800 mb-4">Welcome to Formicarium</h1>
  <p class="text-xl text-stone-600 mb-8">An ant colony simulator</p>
  <button
    on:click={startSimulation}
    class="px-6 py-3 bg-green-700 text-white font-semibold rounded-lg shadow-md hover:bg-green-800 transition-colors"
  >
    Start simulation
  </button>
</main>
