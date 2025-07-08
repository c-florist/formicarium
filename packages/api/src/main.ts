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

  fastify.register((instance, _opts, done) => {
    simulator(instance);

    fastify.register(worldRouter);
    done();
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
