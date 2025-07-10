import { randomUUID } from "node:crypto";
import { describe, expect, it, vi } from "vitest";
import type { Position } from "../domain";
import { ANT_STATES, LIFECYCLE_STATES } from "../domain";
import { Simulation } from "./simulation";

describe("Simulation", () => {
  it("should add a new ant to the world state when createAnt is called", () => {
    const simulation = new Simulation();
    const initialPosition: Position = { x: 10, y: 10 };

    simulation.createAnt(initialPosition);

    expect(simulation.world.ants.size).toBe(1);
    const ant = simulation.world.ants.values().next().value;
    expect(ant).toBeDefined();

    expect(ant?.position).toEqual(initialPosition);
    expect(ant?.state).toBe(ANT_STATES.FORAGING);
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
    expect(antAfterStep?.position).not.toEqual(initialPosition);

    vi.restoreAllMocks();
  });

  it("should remove a food source when its amount reaches zero", () => {
    const simulation = new Simulation();
    const foodId = randomUUID();
    simulation.world.food = [
      { id: foodId, position: { x: 20, y: 20 }, amount: 1 },
    ];
    simulation.createAnt({ x: 19, y: 19 });

    expect(simulation.world.food.length).toBe(1);

    // The ant will find the food and decrement its amount to 0
    simulation.tick();

    // The simulation should now remove the empty food source
    expect(simulation.world.food.length).toBe(0);
  });

  it("should add an ant with an ALIVE lifecycle state to the world", () => {
    const simulation = new Simulation();
    const initialPosition: Position = { x: 10, y: 10 };

    simulation.createAnt(initialPosition);

    const ant = simulation.world.ants.values().next().value;
    expect(ant).toBeDefined();
    expect(ant?.lifecycle).toBe(LIFECYCLE_STATES.ALIVE);
  });

  it("should remove dead ants from the simulation", () => {
    const simulation = new Simulation();
    const initialPosition: Position = { x: 10, y: 10 };
    simulation.createAnt(initialPosition);

    const antId = simulation.world.ants.keys().next().value;
    simulation.killAnt(antId!);

    simulation.tick();

    expect(simulation.world.ants.size).toBe(0);
  });
});
