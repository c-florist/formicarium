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

  fastify.get("/ws/world", { websocket: true }, (socket, _request) => {
    const tickListener = () => {
      socket.send(JSON.stringify(simulation.world.toJSON()));
    };

    simulation.addTickListener(tickListener);

    socket.on("close", () => {
      simulation.removeTickListener(tickListener);
    });
  });

  return fastify;
}
