import type { AntState, FoodSource, Pheromone, Position } from "./domain";

/**
 * Represents the public state of an ant in the world.
 */
export class Ant {
  readonly id: string;
  position: Position;
  state: AntState;

  constructor(id: string, position: Position, state: AntState) {
    this.id = id;
    this.position = position;
    this.state = state;
  }
}

/**
 * Represents the read model of the entire simulation world
 */
export class World {
  width: number;
  height: number;
  ants: Map<string, Ant>;
  food: FoodSource[];
  pheromones: Map<string, Pheromone>;

  constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
    this.ants = new Map();
    this.food = [];
    this.pheromones = new Map();
  }

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
