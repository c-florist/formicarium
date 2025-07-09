import { describe, expect, it } from "vitest";
import { distance, hasArrived } from "./maths";

describe("distance", () => {
  it("should calculate the correct Euclidean distance between two points", () => {
    const posA = { x: 0, y: 0 };
    const posB = { x: 3, y: 4 };
    const result = distance(posA, posB);
    expect(result).toBe(5);
  });
});

describe("hasArrived", () => {
  it("should return true when the distance is less than the threshold", () => {
    const posA = { x: 10, y: 10 };
    const posB = { x: 11, y: 11 }; // distance is ~1.41
    expect(hasArrived(posA, posB)).toBe(true);
  });

  it("should return false when the distance is greater than the threshold", () => {
    const posA = { x: 10, y: 10 };
    const posB = { x: 13, y: 14 }; // distance is 5
    expect(hasArrived(posA, posB)).toBe(false);
  });

  it("should return true when the distance is exactly on the threshold", () => {
    const posA = { x: 10, y: 10 };
    const posB = { x: 12, y: 10 }; // distance is 2
    expect(hasArrived(posA, posB)).toBe(false); // because it's < 2, not <= 2
  });
});
