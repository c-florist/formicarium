import type { Simulation } from "@formicarium/core";
import type { FastifyInstance } from "fastify";

declare module "fastify" {
  interface FastifyInstance {
    simulation: Simulation;
  }
}

export default async function worldRouter(fastify: FastifyInstance) {
  fastify.get("/world", async (_request, reply) => {
    reply.send(fastify.simulation.world);
  });

  fastify.get("/ws/world", { websocket: true }, (socket, _request) => {
    const tickListener = () => {
      socket.send(JSON.stringify(fastify.simulation.world.toJSON()));
    };

    fastify.simulation.addTickListener(tickListener);

    socket.on("close", () => {
      fastify.simulation.removeTickListener(tickListener);
    });
  });
}
