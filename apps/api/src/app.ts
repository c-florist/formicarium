import Fastify, { type FastifyPluginAsync } from "fastify";
import { World } from "./simulation/world";

// Test world
const world = new World(100, 100);

world.addAnt({ x: 50, y: 50 });
world.addAnt({ x: 51, y: 50 });
world.addAnt({ x: 49, y: 50 });

const app: FastifyPluginAsync = async (fastify, _opts): Promise<void> => {
  fastify.get("/", async (_request, _reply) => world);
};

export default app;

// This allows the file to be both imported as a plugin and run directly
if (import.meta.url.startsWith("file:")) {
  const server = Fastify({
    logger: {
      transport: {
        target: "pino-pretty",
      },
    },
  });
  await server.register(app);
  await server.listen({ port: 3000 });
}
