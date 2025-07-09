import { describe, expect, it, vi } from "vitest";
import { AntActor } from "./ant";
import type { FoodSource, Position } from "../domain";
import { ANT_STATES } from "../domain";
import { distance } from "../utils/maths";
import { World } from "../system/world";

describe("AntActor", () => {
  it("should change position when in FORAGING state", () => {
    vi.spyOn(Math, "random").mockReturnValue(0.9);

    const initialPosition: Position = { x: 10, y: 10 };
    const actor = new AntActor(initialPosition);
    const world = new World({
      width: 100,
      height: 100,
      nestPosition: { x: 50, y: 50 },
    });

    actor.update(world);

    expect(actor.getPosition()).not.toEqual(initialPosition);

    vi.restoreAllMocks();
  });

  it("should move towards the nearest food source when foraging", () => {
    const initialPosition: Position = { x: 10, y: 10 };
    const actor = new AntActor(initialPosition);

    const nearestFood: FoodSource = { position: { x: 20, y: 20 }, amount: 100 };
    const farthestFood: FoodSource = {
      position: { x: 80, y: 80 },
      amount: 100,
    };

    const world = new World({
      width: 100,
      height: 100,
      nestPosition: { x: 50, y: 50 },
      foodSources: [farthestFood, nearestFood],
    });

    const initialDistance = distance(initialPosition, nearestFood.position);

    actor.update(world);

    const newDistance = distance(actor.getPosition(), nearestFood.position);

    expect(newDistance).toBeLessThan(initialDistance);
  });

  it("should switch to RETURNING_TO_NEST state when it finds food", () => {
    const initialPosition: Position = { x: 18, y: 18 };
    const actor = new AntActor(initialPosition);

    const food: FoodSource = { position: { x: 20, y: 20 }, amount: 100 };

    const world = new World({
      width: 100,
      height: 100,
      nestPosition: { x: 50, y: 50 },
      foodSources: [food],
    });

    // Move the ant close to the food
    actor.update(world);
    actor.update(world);

    expect(actor.getState()).toBe(ANT_STATES.RETURNING_TO_NEST);
  });
});
