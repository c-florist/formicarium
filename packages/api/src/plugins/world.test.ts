import { describe, it, expect } from "vitest";
import Fastify from "fastify";
import worldPlugin from "./world";
import { Simulation } from "@formicarium/core";
import WebSocket from "ws";

describe("World Plugin", () => {
  it("should return the world state for the /world route", async () => {
    const fastify = Fastify();
    fastify.register(worldPlugin);
    await fastify.ready();

    const response = await fastify.inject({
      method: "GET",
      url: "/world",
    });

    expect(response.statusCode).toBe(200);
    const payload = JSON.parse(response.payload);
    expect(payload).toHaveProperty("width");
    expect(payload).toHaveProperty("height");
  });
});
