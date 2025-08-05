import type { SimulationOptions } from "@formicarium/domain";

type UserSimulationOptions = Omit<SimulationOptions, "width" | "height">;

export const userOptions = $state<UserSimulationOptions>({
  startingAnts: 50,
  startingFoodSources: 50,
  maxFoodSources: 100,
});

export const USER_OPTION_LIMITS = {
  startingAnts: { min: 10, max: 2500 },
  startingFoodSources: { min: 10, max: 2500 },
  maxFoodSources: { min: 1, max: 2500 },
};
