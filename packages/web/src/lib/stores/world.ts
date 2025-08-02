import SimulationService from "$lib/services/simulation";
import type { StatsDto, WorldDto } from "@formicarium/domain";
import { writable } from "svelte/store";

export const worldStore = writable<WorldDto | null>(null);
export const statsStore = writable<StatsDto | null>(null);

const SIMULATION_TICK_RATE = 100; // ms per tick
let animationFrameId: number | null = null;

export const startWorldUpdates = () => {
  if (animationFrameId) {
    throw new Error("World updates are already running");
  }

  let lastUpdateTime = performance.now();
  let accumulator = 0;

  const gameLoop = (currentTime: number) => {
    const deltaTime = currentTime - lastUpdateTime;
    lastUpdateTime = currentTime;
    accumulator += deltaTime;

    // Run the simulation logic if enough time has passed
    while (accumulator >= SIMULATION_TICK_RATE) {
      const worldData = SimulationService.advance();
      worldStore.set(worldData);

      const statsData = SimulationService.getWorldStatistics();
      statsStore.set(statsData);

      accumulator -= SIMULATION_TICK_RATE;
    }

    animationFrameId = requestAnimationFrame(gameLoop);
  };

  // Start the loop
  animationFrameId = requestAnimationFrame(gameLoop);
};

export const stopWorldUpdates = () => {
  if (animationFrameId) {
    cancelAnimationFrame(animationFrameId);
    animationFrameId = null;
    worldStore.set(null);
    statsStore.set(null);
  }
};
