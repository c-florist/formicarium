import { describe, it, expect } from "vitest";
import { createServer } from "./server";
import { Simulation } from "../simulation/simulation";
import WebSocket from "ws";

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

  it("should broadcast world updates on /ws/world to connected clients", async () => {
    const simulation = new Simulation();
    const server = await createServer(simulation);
    await server.listen({ port: 0 });

    const port = (server.server.address() as WebSocket.AddressInfo).port;

    const ws = new WebSocket(`ws://localhost:${port}/ws/world`);

    const message = await new Promise((resolve, reject) => {
      ws.on("open", () => {
        simulation.tick();
      });
      ws.on("message", (data) => {
        resolve(JSON.parse(data.toString()));
      });
      ws.on("error", reject);
    });

    expect(message).toEqual(simulation.world.toJSON());

    ws.close();
    await server.close();
  });
});
