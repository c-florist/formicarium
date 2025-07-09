import { Simulation } from "@formicarium/core";
import type { FastifyInstance } from "fastify";

export default async function simulator(fastify: FastifyInstance) {
  const simulation = new Simulation();
  simulation.start();

  // TODO: Remove once there's a mechanism for generating a starting state
  simulation.createAnt({ x: 10, y: 30 });
  simulation.createAnt({ x: 200, y: 100 });

  fastify.decorate("simulation", simulation);
}
