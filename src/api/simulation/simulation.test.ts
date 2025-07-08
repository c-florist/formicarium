import { describe, expect, it, vi } from "vitest";
import type { Position } from "./domain";
import { ANT_STATES } from "./domain";
import { Simulation } from "./simulation";

// biome-ignore-start lint/style/noNonNullAssertion: No need to assert non-nullability in these tests
describe("Simulation", () => {
  it("should add a new ant to the world state when createAnt is called", () => {
    const simulation = new Simulation();
    const initialPosition: Position = { x: 10, y: 10 };

    simulation.createAnt(initialPosition);

    expect(simulation.world.ants.size).toBe(1);
    const ant = simulation.world.ants.values().next().value;
    expect(ant).toBeDefined();

    expect(ant!.position).toEqual(initialPosition);
    expect(ant!.state).toBe(ANT_STATES.FORAGING);
  });

  it("should update an ant's position in the world after a simulation tick", () => {
    vi.spyOn(Math, "random").mockReturnValue(0.8);

    const simulation = new Simulation();
    const initialPosition: Position = { x: 10, y: 10 };
    simulation.createAnt(initialPosition);
    const antId = simulation.world.ants.keys().next().value;
    expect(antId).toBeDefined();

    simulation.tick();

    const antAfterStep = simulation.world.ants.get(antId!);
    expect(antAfterStep).toBeDefined();
    expect(antAfterStep!.position).not.toEqual(initialPosition);

    vi.restoreAllMocks();
  });
});
// biome-ignore-end lint/style/noNonNullAssertion: No need to assert non-nullability in these tests
