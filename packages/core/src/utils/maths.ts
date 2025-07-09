import type { Position } from "../domain";

const ARRIVAL_THRESHOLD = 2;

export const distance = (a: Position, b: Position) => {
  return Math.sqrt((a.x - b.x) ** 2 + (a.y - b.y) ** 2);
};

export const hasArrived = (a: Position, b: Position) => {
  return distance(a, b) < ARRIVAL_THRESHOLD;
};
