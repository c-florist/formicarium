import type { FastifyInstance } from "fastify";
import { Simulation } from "../core/simulation";

export default async function simulator(fastify: FastifyInstance) {
  const simulation = new Simulation();
  simulation.start();

  fastify.decorate("simulation", simulation);
}
