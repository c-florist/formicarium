import type { FastifyInstance } from "fastify";

export default async function simulator(fastify: FastifyInstance) {
  // We will now create the simulation instance dynamically in the route handler.
  // This decorator will be populated on-demand.
  fastify.decorate("simulation");
}
