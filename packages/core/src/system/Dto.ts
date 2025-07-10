import type {
  AntState,
  FoodSource,
  LifecycleState,
  Nest,
  Pheromone,
  Position,
} from "../domain";

/**
 * Represents the public state of an ant in the world.
 */
export class AntDto {
  readonly id: string;
  readonly position: Position;
  readonly state: AntState;
  readonly lifecycle: LifecycleState;

  constructor({
    id,
    position,
    state,
    lifecycle,
  }: {
    id: string;
    position: Position;
    state: AntState;
    lifecycle: LifecycleState;
  }) {
    this.id = id;
    this.position = position;
    this.state = state;
    this.lifecycle = lifecycle;
  }
}

/**
 * Represents the read model of the entire simulation world
 */
export class WorldDto {
  readonly width: number;
  readonly height: number;
  readonly ants: ReadonlyMap<string, AntDto>;
  readonly foodSources: readonly FoodSource[];
  readonly pheromones: ReadonlyMap<string, Pheromone>;
  readonly nest: Nest;

  constructor({
    width,
    height,
    ants,
    foodSources,
    pheromones,
    nest,
  }: {
    width: number;
    height: number;
    ants: Map<string, AntDto>;
    foodSources: FoodSource[];
    pheromones: Map<string, Pheromone>;
    nest: Nest;
  }) {
    this.width = width;
    this.height = height;
    this.ants = ants;
    this.foodSources = foodSources;
    this.pheromones = pheromones;
    this.nest = nest;
  }

  toJSON() {
    return {
      width: this.width,
      height: this.height,
      ants: Object.fromEntries(this.ants),
      food: this.foodSources,
      pheromones: Object.fromEntries(this.pheromones),
      nest: this.nest,
    };
  }
}
