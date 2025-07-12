import type { FastifyInstance } from "fastify";
import { Orchestrator } from "../core/orchestrator";

export default async function simulator(fastify: FastifyInstance) {
  const simOrchestrator = new Orchestrator();
  simOrchestrator.start();

  fastify.decorate("simulator", simOrchestrator);
}
