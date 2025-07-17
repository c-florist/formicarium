import { writable } from "svelte/store";

type SimulationState = {
  isRunning: boolean;
};

/**
 * Tracks the global state of the simulation.
 */
export const simulationState = writable<SimulationState>({
  isRunning: false,
});
