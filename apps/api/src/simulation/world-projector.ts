import { Ant } from "./ant";
import { ANT_CREATED, type SimulationEvent } from "./events";
import { World } from "./world";

/**
 * Creates a world state from a series of events.
 *
 * @param events The list of events to apply.
 * @returns The final, projected state of the world.
 */
export function projectWorld(events: SimulationEvent[]) {
  const world = new World(100, 100);

  for (const event of events) {
    switch (event.type) {
      case ANT_CREATED: {
        const { id, position } = event.payload;
        const ant = new Ant(id, position);
        world.ants.set(ant.id, ant);
        break;
      }
    }
  }

  return world;
}
