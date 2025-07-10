import { randomUUID } from "node:crypto";
import {
  ANT_ACTOR_ACTIONS,
  ANT_STATES,
  type AntState,
  LIFECYCLE_STATES,
  type LifecycleState,
  type Perception,
  type Position,
} from "../domain";
import { hasArrived } from "../utils/maths";

export class AntActor {
  readonly id: string;
  private position: Position;
  private state: AntState;
  private lifecycle: LifecycleState;
  // @ts-ignore: Whilst I determine how to model carrying capacity
  private hasFood: boolean;

  constructor(position: Position) {
    this.id = randomUUID();
    this.position = position;
    this.state = ANT_STATES.FORAGING;
    this.lifecycle = LIFECYCLE_STATES.ALIVE;
    this.hasFood = false;
  }

  getPosition() {
    return this.position;
  }

  getState() {
    return this.state;
  }

  getLifecycle() {
    return this.lifecycle;
  }

  kill() {
    this.lifecycle = LIFECYCLE_STATES.DEAD;
  }

  move(directionX: number, directionY: number) {
    this.position = {
      x: this.position.x + directionX,
      y: this.position.y + directionY,
    };
  }

  perceive(perception: Perception) {
    switch (this.state) {
      case ANT_STATES.FORAGING: {
        const { nearestFood } = perception;

        if (nearestFood && hasArrived(this.position, nearestFood.position)) {
          this.state = ANT_STATES.RETURNING_TO_NEST;
          this.hasFood = true;
          return {
            id: this.id,
            actionType: ANT_ACTOR_ACTIONS.TAKE_FOOD,
            payload: {
              foodId: nearestFood.id,
            },
          };
        }

        if (!nearestFood) {
          const directionX = Math.floor(Math.random() * 3) - 1;
          const directionY = Math.floor(Math.random() * 3) - 1;
          return {
            id: this.id,
            actionType: ANT_ACTOR_ACTIONS.MOVE,
            payload: {
              directionX,
              directionY,
            },
          };
        }

        const directionX = Math.sign(nearestFood.position.x - this.position.x);
        const directionY = Math.sign(nearestFood.position.y - this.position.y);
        return {
          id: this.id,
          actionType: ANT_ACTOR_ACTIONS.MOVE,
          payload: {
            directionX,
            directionY,
          },
        };
      }
      case ANT_STATES.RETURNING_TO_NEST: {
        const { nestPosition } = perception;
        if (hasArrived(this.position, nestPosition)) {
          this.state = ANT_STATES.FORAGING;
          this.hasFood = false;
          return {
            id: this.id,
            actionType: ANT_ACTOR_ACTIONS.IDLE,
          };
        }

        const directionX = Math.sign(nestPosition.x - this.position.x);
        const directionY = Math.sign(nestPosition.y - this.position.y);
        return {
          id: this.id,
          actionType: ANT_ACTOR_ACTIONS.MOVE,
          payload: {
            directionX,
            directionY,
          },
        };
      }
    }
  }
}
