<script lang="ts">
import { goto } from "$app/navigation";
import PrimaryButton from "$lib/components/ui/PrimaryButton.svelte";
import {
  simulationState,
  USER_OPTION_LIMITS,
  userOptions,
} from "$lib/state/simulation.svelte";

let form: HTMLFormElement;

const startSimulation = (event?: Event) => {
  event?.preventDefault();
  if (form.checkValidity()) {
    simulationState.isRunning = true;
    goto("/world");
  } else {
    form.reportValidity();
  }
};
</script>

<div class="flex min-h-screen items-center justify-center bg-amber-100">
  <div
    class="w-full space-y-8 max-w-4xl bg-stone-800 p-8 border-4 rounded-sm border-stone-900 shadow-[8px_8px_0_#00000020]"
  >
    <div class="text-center space-y-4">
      <h1 class="text-4xl font-bold text-amber-50">
        Welcome to the Formicarium
      </h1>
      <p class="mx-auto max-w-xl text-lg leading-relaxed text-stone-300">
        A real-time ant colony simulation.
      </p>
      <h3 class="text-2xl font-bold text-amber-50">
        Configure your simulation run
      </h3>
    </div>

    <form
      onsubmit={startSimulation}
      bind:this={form}
      novalidate
      class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8"
    >
      <div>
        <label for="startingAnts" class="block text-amber-50 mb-2">Starting Ants</label>
        <input
          type="number"
          required
          min={USER_OPTION_LIMITS.startingAnts.min}
          max={USER_OPTION_LIMITS.startingAnts.max}
          id="startingAnts"
          bind:value={userOptions.startingAnts}
          class="w-full bg-stone-700 text-white p-2 border-2 rounded-sm border-stone-900 focus:border-amber-400 focus:outline-none invalid:border-red-500"
        />
      </div>
      <div>
        <label for="startingFoodSources" class="block text-amber-50 mb-2">Starting Food Sources</label>
        <input
          type="number"
          required
          min={USER_OPTION_LIMITS.startingFoodSources.min}
          max={USER_OPTION_LIMITS.startingFoodSources.max}
          id="startingFoodSources"
          bind:value={userOptions.startingFoodSources}
          class="w-full bg-stone-700 text-white p-2 border-2 rounded-sm border-stone-900 focus:border-amber-400 focus:outline-none invalid:border-red-500"
        />
      </div>
      <div class="md:col-span-2">
        <label for="maxFoodSources" class="block text-amber-50 mb-2">Max Food Sources</label>
        <input
          type="number"
          required
          min={USER_OPTION_LIMITS.maxFoodSources.min}
          max={USER_OPTION_LIMITS.maxFoodSources.max}
          id="maxFoodSources"
          bind:value={userOptions.maxFoodSources}
          class="w-full bg-stone-700 text-white p-2 border-2 rounded-sm border-stone-900 focus:border-amber-400 focus:outline-none invalid:border-red-500"
        />
      </div>
    </form>

    <div class="text-center">
      <PrimaryButton onClick={startSimulation} text="Start simulation" />
    </div>
  </div>
</div>

<svelte:head>
  <title>Formicarium</title>
  <meta
    name="description"
    content="A real-time ant colony simulation showcasing complex emergent behaviors."
  />
</svelte:head>
