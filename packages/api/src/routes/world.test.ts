import Fastify from "fastify";
import { describe, expect, it } from "vitest";
import simulator from "../plugins/simulator";
import worldRouter from "./world";

describe("World Router", () => {
  it("should return the world state for the /world route", async () => {
    const fastify = Fastify();
    fastify.register((instance, _opts, done) => {
      simulator(instance);
      fastify.register(worldRouter);
      done();
    });
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
