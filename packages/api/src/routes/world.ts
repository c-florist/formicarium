import { Simulation } from "@formicarium/core";
import type { FastifyInstance } from "fastify";

declare module "fastify" {
  interface FastifyInstance {
    simulation: Simulation;
  }
}

export default async function worldRouter(fastify: FastifyInstance) {
  fastify.get("/ws/world", { websocket: true }, (socket, request) => {
    const { width, height } = request.query as { width?: string; height?: string };

    const simulation = new Simulation({
      width: width ? parseInt(width, 10) : 800,
      height: height ? parseInt(height, 10) : 600,
    });
    simulation.start();

    // TODO: Remove once there's a mechanism for generating a starting state
    simulation.createAnt({ x: 50, y: 20 });
    simulation.createAnt({ x: 44, y: 22 });

    const tickListener = () => {
      socket.send(JSON.stringify(simulation.world.toJSON()));
    };

    simulation.addTickListener(tickListener);

    socket.on("close", () => {
      simulation.removeTickListener(tickListener);
      simulation.stop();
    });
  });
}
