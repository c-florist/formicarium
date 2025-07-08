import { randomUUID } from "node:crypto";
import { ANT_STATES, type AntState, type Position } from "./types";
import type { World } from "./world";

export class AntActor {
  readonly id: string;
  position: Position;
  state: AntState;

  constructor(position: Position) {
    this.id = randomUUID();
    this.position = position;
    this.state = ANT_STATES.FORAGING;
  }

  /**
   * Processes a single tick of the simulation for this ant.
   * The ant makes a decision based on the world state and updates its own state.
   */
  update(_world: World) {
    if (this.state === ANT_STATES.FORAGING) {
      const newPosition = {
        x: this.position.x + Math.floor(Math.random() * 3) - 1,
        y: this.position.y + Math.floor(Math.random() * 3) - 1,
      };

      // Directly update private state
      this.position = newPosition;
    }
  }
}
