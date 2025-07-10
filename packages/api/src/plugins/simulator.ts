import { Simulation } from "@formicarium/orchestrator";
import type { FastifyInstance } from "fastify";

export default async function simulator(fastify: FastifyInstance) {
  const simulation = new Simulation();
  simulation.start();

  fastify.decorate("simulation", simulation);
}
