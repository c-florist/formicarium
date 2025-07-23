import { simulationState } from "$lib/state/simulation.svelte";
import { redirect } from "@sveltejs/kit";

export const load = () => {
  if (!simulationState.isRunning) {
    redirect(307, "/");
  }
};
