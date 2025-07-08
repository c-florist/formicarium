import { describe, expect, it } from "vitest";
import type { FoodSource, Position } from "../domain";
import { World } from "../system/world";
import { distance } from "../utils/maths";
import { AntActor } from "./ant";

describe("AntActor", () => {
  it("should change position when in FORAGING state", () => {
    const initialPosition: Position = { x: 10, y: 10 };
    const actor = new AntActor(initialPosition);
    const world = new World({
      width: 100,
      height: 100,
      nestPosition: { x: 50, y: 50 },
    });

    actor.update(world);

    expect(actor.getPosition()).not.toEqual(initialPosition);
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
});
