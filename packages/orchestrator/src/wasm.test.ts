import { describe, expect, it } from "vitest";
import { Simulation } from "../../core-rs/pkg/core_rs";

describe("Wasm Simulation", () => {
  it("should update an entity's position after a tick", () => {
    // 1. Setup
    const simulation = new Simulation();
    const entityId = simulation.add_ant(10.0, 10.0, 5.0, -5.0);

    // 2. Action
    simulation.tick();

    // 3. Assertion
    const newPositionX = simulation.get_ant_position_x(entityId);
    expect(newPositionX).toBe(15.0);
  });
});
