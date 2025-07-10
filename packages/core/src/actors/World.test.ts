import { describe, expect, it } from "vitest";
import { ANT_STATES, LIFECYCLE_STATES, type Position } from "../domain";
import { AntDto } from "../system/Dto";
import { WorldActor } from "./World";

describe("WorldActor", () => {
  it("should be created with a world DTO", () => {
    const nestPosition: Position = { x: 50, y: 50 };
    const actor = new WorldActor({
      width: 100,
      height: 100,
      nestPosition,
    });

    const worldDto = actor.getDto();
    expect(worldDto).toBeDefined();
    expect(worldDto.width).toBe(100);
    expect(worldDto.height).toBe(100);
    expect(worldDto.nest.position).toEqual(nestPosition);
  });

  it("should add an ant to the world", () => {
    const nestPosition: Position = { x: 50, y: 50 };
    const actor = new WorldActor({
      width: 100,
      height: 100,
      nestPosition,
    });

    const ant = new AntDto({
      id: "1",
      position: { x: 10, y: 10 },
      state: ANT_STATES.FORAGING,
      lifecycle: LIFECYCLE_STATES.ALIVE,
    });

    actor.addAnt(ant);

    const worldDto = actor.getDto();
    expect(worldDto.ants.size).toBe(1);
    expect(worldDto.ants.get("1")).toEqual(ant);
  });
});
