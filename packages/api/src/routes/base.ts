import type { FastifyInstance } from "fastify";

export default async function baseRouter(fastify: FastifyInstance) {
  fastify.get("/", (_request, reply) => {
    reply.send("Welcome to the formicarium!");
  });
}
