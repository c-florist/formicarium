import { describe, expect, it } from "vitest";
import { ANT_STATES, LIFECYCLE_STATES } from "../domain";
import { AntDto } from "../system/Dto";
import { WorldActor } from "./World";

describe("WorldActor", () => {
  it("should be created with a world DTO", () => {
    const actor = new WorldActor({
      width: 100,
      height: 100,
    });

    const worldDto = actor.getDto();
    expect(worldDto).toBeDefined();
    expect(worldDto.width).toBe(100);
    expect(worldDto.height).toBe(100);
    expect(worldDto.ants.size).toBe(0);
    expect(worldDto.foodSources.length).toBe(5); // Default food sources
  });

  it("should add an ant", () => {
    const actor = new WorldActor({ width: 100, height: 100 });
    const antData = {
      id: "1",
      position: { x: 10, y: 10 },
      state: ANT_STATES.FORAGING,
      lifecycle: LIFECYCLE_STATES.ALIVE,
    };

    actor.addAnt(antData);

    const worldDto = actor.getDto();
    expect(worldDto.ants.size).toBe(1);
    const antDto = worldDto.ants.get("1");
    expect(antDto).toBeInstanceOf(AntDto);
    expect(antDto?.id).toBe("1");
  });

  it("should update an ant", () => {
    const actor = new WorldActor({ width: 100, height: 100 });
    const antData = {
      id: "1",
      position: { x: 10, y: 10 },
      state: ANT_STATES.FORAGING,
      lifecycle: LIFECYCLE_STATES.ALIVE,
    };
    actor.addAnt(antData);

    const updatedAntData = { ...antData, position: { x: 20, y: 20 } };
    actor.updateAnt(updatedAntData);

    const worldDto = actor.getDto();
    const antDto = worldDto.ants.get("1");
    expect(antDto?.position).toEqual({ x: 20, y: 20 });
  });

  it("should remove an ant", () => {
    const actor = new WorldActor({ width: 100, height: 100 });
    const antData = {
      id: "1",
      position: { x: 10, y: 10 },
      state: ANT_STATES.FORAGING,
      lifecycle: LIFECYCLE_STATES.ALIVE,
    };
    actor.addAnt(antData);
    expect(actor.getDto().ants.size).toBe(1);

    actor.removeAnt("1");
    expect(actor.getDto().ants.size).toBe(0);
  });
});
