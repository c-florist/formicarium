import { describe, it, expect } from "vitest";
import { createServer } from "./server";
import { Simulation } from "../simulation/simulation";

describe("Server", () => {
  it("should return the world state for the /world route", async () => {
    const simulation = new Simulation();
    const server = await createServer(simulation);

    const response = await server.inject({
      method: "GET",
      url: "/world",
    });

    expect(response.statusCode).toBe(200);
    expect(response.json()).toEqual(simulation.world.toJSON());
  });
});
