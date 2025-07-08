import type { Position } from "../domain";

export const distance = (a: Position, b: Position) => {
  return Math.sqrt((a.x - b.x) ** 2 + (a.y - b.y) ** 2);
};
