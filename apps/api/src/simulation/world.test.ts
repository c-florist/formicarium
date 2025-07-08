import { describe, it, expect } from "vitest";
import { World } from "./world";
import type { FoodSource, Position } from "./domain";

describe("World", () => {
  const nestPosition: Position = { x: 50, y: 50 };

  it("should initialize the nest at the correct position", () => {
    const world = new World({
      width: 100,
      height: 100,
      nestPosition,
    });

    expect(world.nest.position).toEqual(nestPosition);
  });

  it("should initialize food sources when provided", () => {
    const foodSources: FoodSource[] = [
      { position: { x: 10, y: 10 }, amount: 100 },
      { position: { x: 80, y: 20 }, amount: 50 },
    ];

    const world = new World({
      width: 100,
      height: 100,
      nestPosition,
      foodSources,
    });

    expect(world.food).toEqual(foodSources);
    expect(world.food.length).toBe(2);
  });

  it("should default to an empty array if food sources are not provided", () => {
    const world = new World({
      width: 100,
      height: 100,
      nestPosition,
    });

    expect(world.food).toEqual([]);
    expect(world.food.length).toBe(0);
  });
});
