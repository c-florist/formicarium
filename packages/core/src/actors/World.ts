import { randomUUID } from "node:crypto";
import {
  type AntState,
  type FoodSource,
  LIFECYCLE_STATES,
  type LifecycleState,
  type Nest,
  type Pheromone,
  type Position,
} from "../domain";
import { AntDto, WorldDto } from "../system/Dto";
import { distance } from "../utils/maths";

type ManagedAnt = {
  id: string;
  position: Position;
  state: AntState;
  lifecycle: LifecycleState;
};

type WorldActorOptions = {
  width: number;
  height: number;
};

export class WorldActor {
  readonly id: string;
  private width: number;
  private height: number;
  private ants: Map<string, ManagedAnt> = new Map();
  private foodSources: FoodSource[];
  private pheromones: Map<string, Pheromone> = new Map();
  private nest: Nest;

  constructor(options: WorldActorOptions) {
    this.id = randomUUID();

    this.width = options.width;
    this.height = options.height;

    this.nest = {
      position: {
        x: Math.floor(Math.random() * options.width),
        y: Math.floor(Math.random() * options.height),
      },
    };

    this.foodSources = Array.from({ length: 5 }, () => ({
      id: randomUUID(),
      position: {
        x: Math.floor(Math.random() * options.width),
        y: Math.floor(Math.random() * options.height),
      },
      amount: 5,
    }));
  }

  getDto() {
    const antDtoMap = new Map<string, AntDto>();

    for (const managedAnt of this.ants.values()) {
      antDtoMap.set(managedAnt.id, new AntDto(managedAnt));
    }

    return new WorldDto({
      width: this.width,
      height: this.height,
      nest: this.nest,
      foodSources: this.foodSources,
      pheromones: this.pheromones,
      ants: antDtoMap,
    });
  }

  getNearestFoodSource(position: Position) {
    if (this.foodSources.length === 0) {
      return null;
    }

    let nearestFood = this.foodSources[0];
    if (!nearestFood) {
      return null;
    }

    let minDistance = distance(position, nearestFood.position);

    for (const food of this.foodSources) {
      const d = distance(position, food.position);
      if (d < minDistance) {
        minDistance = d;
        nearestFood = food;
      }
    }
    return nearestFood;
  }

  addAnt(antData: {
    id: string;
    position: Position;
    state: AntState;
    lifecycle: LifecycleState;
  }) {
    this.ants.set(antData.id, { ...antData });
  }

  removeAnt(antId: string) {
    this.ants.delete(antId);
  }

  updateAnt(antData: {
    id: string;
    position: Position;
    state: AntState;
    lifecycle: LifecycleState;
  }) {
    const antToUpdate = this.ants.get(antData.id);
    if (antToUpdate) {
      this.ants.set(antData.id, { ...antData });
    }
  }

  removeDeadAnts() {
    for (const ant of this.ants.values()) {
      if (ant.lifecycle === LIFECYCLE_STATES.DEAD) {
        this.ants.delete(ant.id);
      }
    }
  }

  depleteFoodSource(foodSourceId: string) {
    const foodSource = this.foodSources.find((f) => f.id === foodSourceId);
    if (foodSource) {
      foodSource.amount -= 1;
    }
  }

  removeDepletedFoodSources() {
    this.foodSources = this.foodSources.filter((f) => f.amount > 0);
  }

  endOfTickCleanup() {
    this.removeDepletedFoodSources();
    this.removeDeadAnts();
  }
}
