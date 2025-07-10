import type {
  AntState,
  FoodSource,
  LifecycleState,
  Nest,
  Pheromone,
  Position,
} from "../domain";

type AntOptions = {
  id: string;
  position: Position;
  state: AntState;
  lifecycle: LifecycleState;
};

/**
 * Represents the public state of an ant in the world.
 */
export class AntDto {
  readonly id: string;
  position: Position;
  state: AntState;
  lifecycle: LifecycleState;

  constructor(options: AntOptions) {
    this.id = options.id;
    this.position = options.position;
    this.state = options.state;
    this.lifecycle = options.lifecycle;
  }
}

type WorldOptions = {
  width: number;
  height: number;
  nestPosition: Position;
  foodSources?: FoodSource[];
};

/**
 * Represents the read model of the entire simulation world
 */
export class WorldDto {
  width: number;
  height: number;
  ants: Map<string, AntDto>;
  food: FoodSource[];
  pheromones: Map<string, Pheromone>;
  nest: Nest;

  constructor(options: WorldOptions) {
    this.width = options.width;
    this.height = options.height;
    this.ants = new Map();
    this.food = options.foodSources ?? [];
    this.pheromones = new Map();
    this.nest = { position: options.nestPosition };
  }

  toJSON() {
    return {
      width: this.width,
      height: this.height,
      ants: Object.fromEntries(this.ants),
      food: this.food,
      pheromones: Object.fromEntries(this.pheromones),
      nest: this.nest,
    };
  }
}
