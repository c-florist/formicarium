import { randomUUID } from "node:crypto";

/**
 * A type for 2D coordinates.
 */
export type Position = {
  x: number;
  y: number;
};

/**
 * The possible states an ant can be in.
 */
export type AntState = "FORAGING" | "RETURNING_TO_NEST";

export class Ant {
  public readonly id: string;
  public position: Position;
  public state: AntState;

  constructor(position: Position) {
    this.id = randomUUID();
    this.position = position;
    this.state = "FORAGING";
  }
}