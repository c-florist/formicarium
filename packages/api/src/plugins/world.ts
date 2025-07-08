import { Simulation } from "@formicarium/core";
import type { FastifyInstance } from "fastify";

declare module "fastify" {
  interface FastifyInstance {
    simulation: Simulation;
  }
}

async function worldRouter(fastify: FastifyInstance) {
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

export default async function worldPlugin(fastify: FastifyInstance) {
  const simulation = new Simulation();
  simulation.start();

  // TODO: Remove once there's a mechanism for generate a starting state
  simulation.createAnt({ x: 50, y: 20 });
  simulation.createAnt({ x: 44, y: 22 });

  fastify.decorate("simulation", simulation);
  fastify.register(worldRouter);
}
