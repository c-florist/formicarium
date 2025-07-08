import websocket from "@fastify/websocket";
import Fastify from "fastify";
import type { Simulation } from "../simulation/simulation";

export async function createServer(simulation: Simulation) {
  const fastify = Fastify({
    logger: {
      transport: {
        target: "pino-pretty",
      },
    },
  });

  await fastify.register(websocket);

  fastify.get("/", (_request, reply) => {
    reply.send("Welcome to the formicarium!");
  });

  fastify.get("/world", (_request, reply) => {
    reply.send(simulation.world);
  });

  return fastify;
}
