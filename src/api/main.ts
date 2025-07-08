import Fastify from "fastify";
import websocket from "@fastify/websocket";
import { base, world } from "./plugins";

async function main() {
  const fastify = Fastify({
    logger: {
      transport: {
        target: "pino-pretty",
      },
    },
  });

  await fastify.register(websocket);

  fastify.register(base);
  fastify.register(world);

  try {
    await fastify.listen({ port: 3000 });
    console.log("Server listening on port 3000");
  } catch (error) {
    fastify.log.error(error);
    process.exit(1);
  }
}

main();
