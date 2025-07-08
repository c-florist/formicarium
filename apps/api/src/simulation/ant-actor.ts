import { randomUUID } from "node:crypto";

import type { SimulationEvent } from "./events";

export type Position = {
  x: number;
  y: number;
};

export const ANT_STATES = {
  FORAGING: "FORAGING",
  RETURNING_TO_NEST: "RETURNING_TO_NEST",
} as const;

export type AntState = (typeof ANT_STATES)[keyof typeof ANT_STATES];

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
  processTick(): SimulationEvent | null {
    // For now, the ant does nothing.
    return null;
  }
}
