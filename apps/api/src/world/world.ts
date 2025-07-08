import type { FoodSource, Pheromone, Position } from "../domain/types";

/**
 * Represents an ant in the world projection
 */
export class Ant {
  readonly id: string;
  position: Position;

  constructor(id: string, position: Position) {
    this.id = id;
    this.position = position;
  }
}

/**
 * Represents the read model of the entire simulation world
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
