import type { FastifyInstance } from "fastify";
import type { Orchestrator } from "../core/orchestrator";

declare module "fastify" {
  interface FastifyInstance {
    simulator: Orchestrator;
  }
}

export default async function worldRouter(fastify: FastifyInstance) {
  fastify.get("/ws/world", { websocket: true }, (socket, _request) => {
    const tickListener = () => {
      socket.send(JSON.stringify(fastify.simulator.getWorldState()));
    };

    fastify.simulator.addTickListener(tickListener);

    // Immediately send the current state upon connection
    tickListener();

    socket.on("close", () => {
      fastify.simulator.removeTickListener(tickListener);
    });
  });
}
