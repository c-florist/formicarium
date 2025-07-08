import { Simulation } from "@formicarium/core";
import type { FastifyInstance } from "fastify";

export default async function simulator(fastify: FastifyInstance) {
  const simulation = new Simulation();
  simulation.start();

  // TODO: Remove once there's a mechanism for generate a starting state
  simulation.createAnt({ x: 50, y: 20 });
  simulation.createAnt({ x: 44, y: 22 });

  fastify.decorate("simulation", simulation);
}
