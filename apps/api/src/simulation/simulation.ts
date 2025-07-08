import { AntActor } from "./ant-actor";
import type { Position } from "./domain";
import { Ant, World } from "./world";

const TICK_INTERVAL_MS = 100;

export class Simulation {
  private actors: Map<string, AntActor> = new Map();
  world: World;
  private timer: NodeJS.Timeout | null = null;

  constructor() {
    this.world = new World({
      width: 100,
      height: 100,
      nestPosition: { x: 50, y: 50 },
    });
  }

  start() {
    if (this.timer) {
      console.log("Simulation is already running");
      return;
    }
    this.timer = setInterval(() => this.tick(), TICK_INTERVAL_MS);
  }

  stop() {
    if (this.timer) {
      clearInterval(this.timer);
      this.timer = null;
    }
  }

  tick() {
    // Actors update their internal state based on the current world
    for (const actor of this.actors.values()) {
      actor.update(this.world);
    }

    // Sync actor state back to the public world for the next tick
    for (const actor of this.actors.values()) {
      const ant = this.world.ants.get(actor.id);
      if (ant) {
        ant.position = actor.position;
        ant.state = actor.state;
      }
    }
  }

  /**
   * Command handler to create a new ant.
   */
  createAnt(position: Position) {
    const actor = new AntActor(position);
    this.actors.set(actor.id, actor);

    const ant = new Ant(actor.id, actor.position, actor.state);
    this.world.ants.set(ant.id, ant);
  }
}
