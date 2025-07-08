import { randomUUID } from "node:crypto";
import { ANT_STATES, type AntState, type Position } from "./domain";
import { distance } from "./utils";
import type { World } from "./world";

export class AntActor {
  readonly id: string;
  private position: Position;
  private state: AntState;

  constructor(position: Position) {
    this.id = randomUUID();
    this.position = position;
    this.state = ANT_STATES.FORAGING;
  }

  getPosition() {
    return this.position;
  }

  getState() {
    return this.state;
  }

  private findNearestFood(world: World) {
    if (world.food.length === 0) {
      return null;
    }

    let nearestFood = world.food[0];
    if (!nearestFood) {
      return null;
    }

    let minDistance = distance(this.position, nearestFood.position);

    for (const food of world.food) {
      const d = distance(this.position, food.position);
      if (d < minDistance) {
        minDistance = d;
        nearestFood = food;
      }
    }
    return nearestFood;
  }

  update(world: World) {
    if (this.state === ANT_STATES.FORAGING) {
      const nearestFood = this.findNearestFood(world);

      if (!nearestFood) {
        // If there's no food, wander randomly
        this.position = {
          x: this.position.x + (Math.floor(Math.random() * 3) - 1),
          y: this.position.y + (Math.floor(Math.random() * 3) - 1),
        };
        return;
      }

      // Move towards the nearest food source
      const directionX = nearestFood.position.x - this.position.x;
      const directionY = nearestFood.position.y - this.position.y;

      this.position = {
        x: this.position.x + Math.sign(directionX),
        y: this.position.y + Math.sign(directionY),
      };
    }
  }
}
