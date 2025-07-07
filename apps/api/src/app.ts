import Fastify, { FastifyPluginAsync } from 'fastify';

const app: FastifyPluginAsync = async (fastify, opts): Promise<void> => {
  fastify.get('/', async function (request, reply) {
    return { hello: 'world' };
  });
};

export default app;

// This allows the file to be both imported as a plugin and run directly
if (import.meta.url.startsWith('file:')) {
  const server = Fastify({
    logger: {
      transport: {
        target: 'pino-pretty',
      },
    },
  });
  await server.register(app);
  await server.listen({ port: 3000 });
}
