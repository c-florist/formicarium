import { randomUUID } from "node:crypto";
import { describe, expect, it } from "vitest";
import type { FoodSource, Nest, Pheromone } from "../domain";
import { type AntDto, WorldDto } from "./Dto";

describe("WorldDto", () => {
  const nest: Nest = { position: { x: 50, y: 50 } };
  const pheromones = new Map<string, Pheromone>();
  const ants = new Map<string, AntDto>();

  it("should initialize correctly", () => {
    const world = new WorldDto({
      width: 100,
      height: 100,
      nest,
      foodSources: [],
      pheromones,
      ants,
    });

    expect(world.width).toBe(100);
    expect(world.height).toBe(100);
    expect(world.nest).toEqual(nest);
  });

  it("should initialize food sources when provided", () => {
    const foodSources: FoodSource[] = [
      { id: randomUUID(), position: { x: 10, y: 10 }, amount: 100 },
      { id: randomUUID(), position: { x: 80, y: 20 }, amount: 50 },
    ];

    const world = new WorldDto({
      width: 100,
      height: 100,
      nest,
      foodSources,
      pheromones,
      ants,
    });

    expect(world.foodSources).toEqual(foodSources);
    expect(world.foodSources.length).toBe(2);
  });
});
