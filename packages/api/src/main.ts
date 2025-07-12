import websocket from "@fastify/websocket";
import Fastify from "fastify";
import simulator from "./plugins/simulator";
import baseRouter from "./routes/base";
import worldRouter from "./routes/world";

async function main() {
  const fastify = Fastify({
    logger: {
      transport: {
        target: "pino-pretty",
      },
    },
  });

  await fastify.register(websocket);

  fastify.register(baseRouter);

  fastify.register(async (instance) => {
    await simulator(instance);
    await instance.register(worldRouter);
  });

  try {
    await fastify.listen({ port: 3000 });
    console.log("Server listening on port 3000");
  } catch (error) {
    fastify.log.error(error);
    process.exit(1);
  }
}

main();
