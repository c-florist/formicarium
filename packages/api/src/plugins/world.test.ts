import Fastify from "fastify";
import { describe, expect, it } from "vitest";
import worldPlugin from "./world";

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
