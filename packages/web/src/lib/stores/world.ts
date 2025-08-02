import SimulationService from "$lib/services/simulation";
import { simulationState } from "$lib/state/simulation.svelte";
import type { StatsDto, WorldDto } from "@formicarium/domain";
import { writable } from "svelte/store";

export const worldStore = writable<{ world: WorldDto; stats: StatsDto } | null>(
  null,
);

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
    while (accumulator >= simulationState.speed) {
      SimulationService.tick();
      const worldData = SimulationService.getWorldData();
      worldStore.set(worldData);

      accumulator -= simulationState.speed;
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
  }
};
