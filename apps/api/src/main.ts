import { createServer } from "./http/server";
import { Simulation } from "./simulation/simulation";

const simulation = new Simulation();
simulation.start();

// Create a couple of ants for starting state
simulation.createAnt({ x: 50, y: 50 });
simulation.createAnt({ x: 52, y: 52 });

const server = createServer(simulation);

async function main() {
  try {
    await server.listen({ port: 3000 });
  } catch (err) {
    server.log.error(err);
    process.exit(1);
  }
}

main();
