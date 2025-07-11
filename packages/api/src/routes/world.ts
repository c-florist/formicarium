import type { FastifyInstance } from "fastify";
import type { Simulation } from "../core/simulation";

declare module "fastify" {
  interface FastifyInstance {
    simulation: Simulation;
  }
}

export default async function worldRouter(fastify: FastifyInstance) {
  fastify.get("/ws/world", { websocket: true }, (socket, _request) => {
    const tickListener = () => {
      socket.send(JSON.stringify(fastify.simulation.getWorld()));
    };

    fastify.simulation.addTickListener(tickListener);

    // Immediately send the current state upon connection
    tickListener();

    socket.on("close", () => {
      fastify.simulation.removeTickListener(tickListener);
    });
  });
}
