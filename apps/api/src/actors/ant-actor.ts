import { randomUUID } from "node:crypto";
import { ANT_STATES, type AntState, type Position } from "../domain/types";
import {
  ANT_EVENT_TYPES,
  type AntMovedEvent,
} from "../events/types";

export class AntActor {
  readonly id: string;
  private position: Position;
  private state: AntState;

  constructor(position: Position) {
    this.id = randomUUID();
    this.position = position;
    this.state = ANT_STATES.FORAGING;
  }

  /**
   * Processes a single tick of the simulation for this ant.
   *
   * @returns A simulation event if the ant decided to do something, or null if not.
   */
  processTick() {
    if (this.state === ANT_STATES.FORAGING) {
      const newPosition = {
        x: this.position.x + Math.floor(Math.random() * 3) - 1,
        y: this.position.y + Math.floor(Math.random() * 3) - 1,
      };

      this.position = newPosition;

      const event: AntMovedEvent = {
        type: ANT_EVENT_TYPES.MOVED,
        payload: {
          id: this.id,
          position: newPosition,
        },
        timestamp: Date.now(),
      };

      return event;
    }

    return null;
  }
}
