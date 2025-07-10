import type { FastifyInstance } from "fastify";
import { Simulation } from "../simulation/simulation";

export default async function simulator(fastify: FastifyInstance) {
  const simulation = new Simulation();
  simulation.start();

  fastify.decorate("simulation", simulation);
}
