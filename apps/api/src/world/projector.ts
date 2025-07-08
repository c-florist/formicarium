import { ANT_EVENT_TYPES, type SimulationEvent } from "../events/types";
import { Ant, World } from "./world";

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
      case ANT_EVENT_TYPES.CREATED: {
        const { id, position } = event.payload;
        const ant = new Ant(id, position);
        world.ants.set(ant.id, ant);
        break;
      }
      case ANT_EVENT_TYPES.MOVED: {
        const { id, position } = event.payload;
        const ant = world.ants.get(id);
        if (ant) {
          ant.position = position;
        }
        break;
      }
    }
  }

  return world;
}
