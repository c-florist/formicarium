import { describe, expect, it } from "vitest";
import { randomUUID } from "node:crypto";
import { AntActor } from "./ant";
import type { FoodSource, Perception, Position } from "../domain";
import { ACTOR_ACTIONS } from "../domain";

describe("AntActor", () => {
  it("should update its position when a MOVE action is applied", () => {
    const initialPosition: Position = { x: 10, y: 10 };
    const actor = new AntActor(initialPosition);

    actor.move(1, -1);

    const newPosition = actor.getPosition();
    expect(newPosition.x).toBe(11);
    expect(newPosition.y).toBe(9);
  });

  it("should return a TAKE_FOOD action when it finds food", () => {
    const initialPosition: Position = { x: 19, y: 19 };
    const actor = new AntActor(initialPosition);
    const foodId = randomUUID();

    const food: FoodSource = {
      id: foodId,
      position: { x: 20, y: 20 },
      amount: 100,
    };
    const nestPosition: Position = { x: 50, y: 50 };

    const perception: Perception = {
      nearestFood: food,
      nestPosition,
    };

    const action = actor.perceive(perception);

    expect(action.type).toBe(ACTOR_ACTIONS.TAKE_FOOD);
    if (action.type === ACTOR_ACTIONS.TAKE_FOOD) {
      expect(action.payload.foodId).toBe(foodId);
    }
  });

  it("should return a MOVE action towards the nearest food source when perceiving the world", () => {
    const initialPosition: Position = { x: 10, y: 10 };
    const actor = new AntActor(initialPosition);

    const nearestFood: FoodSource = {
      id: randomUUID(),
      position: { x: 20, y: 20 },
      amount: 100,
    };
    const nestPosition: Position = { x: 50, y: 50 };

    const perception: Perception = {
      nearestFood,
      nestPosition,
    };

    const action = actor.perceive(perception);

    expect(action.type).toBe(ACTOR_ACTIONS.MOVE);
    if (action.type === ACTOR_ACTIONS.MOVE) {
      expect(action.payload.directionX).toBe(1);
      expect(action.payload.directionY).toBe(1);
    }
  });
});
