import SimulationService from "$lib/services/simulation";
import type { StatsDto, WorldDto } from "@formicarium/domain";
import { writable } from "svelte/store";

export const worldStore = writable<WorldDto | null>(null);
export const statsStore = writable<StatsDto | null>(null);

let intervalId: NodeJS.Timeout | null = null;

export const startWorldUpdates = () => {
  if (intervalId) {
    throw new Error("World updates are already running");
  }

  intervalId = setInterval(() => {
    const worldData = SimulationService.advance();
    worldStore.set(worldData);

    const statsData = SimulationService.getWorldStatistics();
    statsStore.set(statsData);
  }, 100);
};

export const stopWorldUpdates = () => {
  if (intervalId) {
    clearInterval(intervalId);
    intervalId = null;
  }
};
