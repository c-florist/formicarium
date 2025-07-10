import { randomUUID } from "node:crypto";
import type { FoodSource, Position } from "../domain";
import { type AntDto, WorldDto } from "../system/Dto";

type WorldActorOptions = {
  width: number;
  height: number;
  nestPosition: Position;
  foodSources?: FoodSource[];
};

export class WorldActor {
  readonly id: string;
  private world: WorldDto;

  constructor(options: WorldActorOptions) {
    this.id = randomUUID();
    this.world = new WorldDto({
      ...options,
      foodSources: options.foodSources ?? [],
    });
  }

  getDto() {
    return this.world;
  }

  addAnt(ant: AntDto) {
    this.world.ants.set(ant.id, ant);
  }
}
