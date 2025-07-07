import { randomUUID } from "node:crypto";

export type Position = {
  x: number;
  y: number;
};

export const ANT_STATES = {
  FORAGING: "FORAGING",
  RETURNING_TO_NEST: "RETURNING_TO_NEST",
} as const;

export type AntState = (typeof ANT_STATES)[keyof typeof ANT_STATES];

export class Ant {
  readonly id: string;
  position: Position;
  state: AntState;

  constructor(position: Position) {
    this.id = randomUUID();
    this.position = position;
    this.state = ANT_STATES.FORAGING;
  }
}
