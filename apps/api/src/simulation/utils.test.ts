import { describe, expect, it } from "vitest";
import { distance } from "./utils";

describe("distance", () => {
  it("should calculate the correct Euclidean distance between two points", () => {
    const posA = { x: 0, y: 0 };
    const posB = { x: 3, y: 4 };
    const result = distance(posA, posB);
    expect(result).toBe(5);
  });
});
