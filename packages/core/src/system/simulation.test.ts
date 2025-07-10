import { describe, expect, it } from "vitest";
import { Simulation } from "./Simulation";

describe("Simulation", () => {
  it("should initialize with a world and a default number of ant actors", () => {
    const simulation = new Simulation();
    const world = simulation.getWorld();

    expect(world).toBeDefined();
    expect(world.width).toBe(1000);
    expect(world.height).toBe(600);
    expect(world.ants.size).toBe(100);
  });
});
