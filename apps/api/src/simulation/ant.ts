const ANT_STATES = {
  FORAGING: "FORAGING",
  CARRYING: "CARRYING",
} as const;

export type AntState = (typeof ANT_STATES)[keyof typeof ANT_STATES];

export interface Position {
  x: number;
  y: number;
}

/**
 * Represents the state of an ant in the world projection.
 */
export class Ant {
  readonly id: string;
  position: Position;
  state: AntState;

  constructor(id: string, position: Position) {
    this.id = id;
    this.position = position;
    this.state = ANT_STATES.FORAGING;
  }
}
