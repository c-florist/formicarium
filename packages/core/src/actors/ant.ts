import { randomUUID } from "node:crypto";
import {
  ANT_STATES,
  type AntState,
  type Position,
  ACTOR_ACTIONS,
  type Action,
} from "../domain";
import type { World } from "../system/world";
import { distance, hasArrived } from "../utils/maths";

export class AntActor {
  readonly id: string;
  private position: Position;
  private state: AntState;
  // @ts-ignore: Whilst I determine how to model carrying capacity
  private hasFood: boolean;

  constructor(position: Position) {
    this.id = randomUUID();
    this.position = position;
    this.state = ANT_STATES.FORAGING;
    this.hasFood = false;
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

  perceive(world: World): Action {
    switch (this.state) {
      case ANT_STATES.FORAGING: {
        const nearestFood = this.findNearestFood(world);

        if (nearestFood && hasArrived(this.position, nearestFood.position)) {
          this.state = ANT_STATES.RETURNING_TO_NEST;
          this.hasFood = true;
          nearestFood.amount -= 1;
          // This will be changed to a TAKE_FOOD action
          return {
            type: ACTOR_ACTIONS.IDLE,
          };
        }

        if (!nearestFood) {
          return {
            type: ACTOR_ACTIONS.MOVE,
            payload: {
              directionX: Math.floor(Math.random() * 3) - 1,
              directionY: Math.floor(Math.random() * 3) - 1,
            },
          };
        }

        const directionX = nearestFood.position.x - this.position.x;
        const directionY = nearestFood.position.y - this.position.y;

        return {
          type: ACTOR_ACTIONS.MOVE,
          payload: {
            directionX: Math.sign(directionX),
            directionY: Math.sign(directionY),
          },
        };
      }
      case ANT_STATES.RETURNING_TO_NEST: {
        if (hasArrived(this.position, world.nest.position)) {
          this.state = ANT_STATES.FORAGING;
          this.hasFood = false;
          return {
            type: ACTOR_ACTIONS.IDLE,
          };
        }

        const directionX = world.nest.position.x - this.position.x;
        const directionY = world.nest.position.y - this.position.y;

        return {
          type: ACTOR_ACTIONS.MOVE,
          payload: {
            directionX: Math.sign(directionX),
            directionY: Math.sign(directionY),
          },
        };
      }
    }
  }

  update(world: World) {
    const action = this.perceive(world);

    switch (action.type) {
      case ACTOR_ACTIONS.MOVE:
        this.position = {
          x: this.position.x + action.payload.directionX,
          y: this.position.y + action.payload.directionY,
        };
        break;
      case ACTOR_ACTIONS.TAKE_FOOD:
        // This will be implemented later
        break;
      case ACTOR_ACTIONS.IDLE:
        // Do nothing
        break;
    }
  }
}
