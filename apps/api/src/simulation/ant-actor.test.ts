import { describe, it, expect } from "vitest";
import { AntActor } from "./ant-actor";
import { World } from "./world";
import type { Position } from "./domain";

describe("AntActor", () => {
  it("should change position when in FORAGING state", () => {
    // Arrange
    const initialPosition: Position = { x: 10, y: 10 };
    const actor = new AntActor(initialPosition);
    const world = new World(100, 100, { x: 50, y: 50 });

    // Act
    actor.update(world);

    // Assert
    expect(actor.position).not.toEqual(initialPosition);
  });
});
