import Fastify from "fastify";
import { describe, expect, it } from "vitest";
import basePlugin from "./base";

describe("baseRouter", () => {
  it("should return a welcome message for the root route", async () => {
    const fastify = Fastify();
    fastify.register(basePlugin);
    await fastify.ready();

    const response = await fastify.inject({
      method: "GET",
      url: "/",
    });

    expect(response.statusCode).toBe(200);
    expect(response.payload).toBe("Welcome to the formicarium!");
  });
});
