import { simulationState } from "$lib/stores/simulation-store";
import { redirect } from "@sveltejs/kit";
import { get } from "svelte/store";

export const load = () => {
  const isRunning = get(simulationState).isRunning;

  if (!isRunning) {
    redirect(307, "/");
  }
};
