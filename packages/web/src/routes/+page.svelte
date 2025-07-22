<script lang="ts">
import { goto } from "$app/navigation";
import { simulationState } from "$lib/stores/simulation-store";

const startSimulation = () => {
  simulationState.set({ isRunning: true });
  goto("/world");
};
</script>

<div class="flex min-h-screen items-center justify-center bg-amber-100 p-4">
  <div class="w-full max-w-4xl bg-stone-800 p-8 border-4 rounded-sm border-stone-900 shadow-[8px_8px_0_#00000020]">
    <div class="text-center">
      <h1 class="text-4xl font-bold text-amber-50 mb-4">
        Welcome to the Formicarium
      </h1>
      <p class="mx-auto max-w-xl text-lg leading-relaxed text-stone-300 mb-8">
        A real-time ant colony simulation showcasing complex emergent behaviors.
      </p>
      <button
        on:click={startSimulation}
        class="bg-green-700 px-6 py-3 font-bold text-white border-2 rounded-sm border-green-900 shadow-[4px_4px_0_theme(colors.green.900)] hover:bg-green-800 active:translate-y-0.5 active:shadow-[2px_2px_0_theme(colors.green.900)] transition-all cursor-pointer"
      >
        Start Simulation
      </button>
    </div>
  </div>
</div>
