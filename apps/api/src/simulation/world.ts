import type { Ant } from "./ant";
import type { Position } from "./ant-actor";

export type FoodSource = {
  position: Position;
  amount: number;
};

export const PHEROMONE_TYPES = {
  TO_FOOD: "to_food",
  TO_HOME: "to_home",
} as const;

export type PheromoneType =
  (typeof PHEROMONE_TYPES)[keyof typeof PHEROMONE_TYPES];

export type Pheromone = {
  position: Position;
  intensity: number;
  type: PheromoneType;
};

/**
 * Represents the read model of the entire simulation world.
 */
export class World {
  ants: Map<string, Ant> = new Map();
  food: FoodSource[] = [];
  pheromones: Map<string, Pheromone> = new Map();

  constructor(
    public width: number,
    public height: number,
  ) {}

  toJSON() {
    return {
      width: this.width,
      height: this.height,
      ants: Object.fromEntries(this.ants),
      food: this.food,
      pheromones: Object.fromEntries(this.pheromones),
    };
  }
}
