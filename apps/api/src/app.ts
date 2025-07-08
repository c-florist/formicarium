import Fastify from "fastify";
import { Simulation } from "./simulation";

const fastify = Fastify({
  logger: {
    transport: {
      target: "pino-pretty",
    },
  },
});

const simulation = new Simulation();
simulation.start();

// Create a couple of ants for starting state
simulation.createAnt({ x: 50, y: 50 });
simulation.createAnt({ x: 52, y: 52 });

fastify.get("/", (_request, reply) => {
  reply.send("Welcome to the formicarium!");
});

fastify.get("/world", (_request, reply) => {
  reply.send(simulation.world);
});

async function main() {
  try {
    await fastify.listen({ port: 3000 });
  } catch (err) {
    fastify.log.error(err);
    process.exit(1);
  }
}

main();
