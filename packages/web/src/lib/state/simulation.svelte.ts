import type { SimulationOptions } from "@formicarium/domain";

type SimulationState = {
  isRunning: boolean;
};

export const simulationState = $state<SimulationState>({
  isRunning: false,
});

type UserSimulationOptions = Omit<SimulationOptions, "width" | "height">;

export const userOptions = $state<UserSimulationOptions>({
  startingAnts: 50,
  startingFoodSources: 50,
  maxFoodSources: 100,
});
