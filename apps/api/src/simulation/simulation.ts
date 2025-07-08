import type { Position } from "./ant";
import { AntActor } from "./ant-actor";
import { EventStore } from "./event-store";
import { ANT_EVENT_TYPES, type AntCreatedEvent } from "./events";
import type { World } from "./world";
import { projectWorld } from "./world-projector";

const TICK_INTERVAL_MS = 1000;

export class Simulation {
  private eventStore = new EventStore();
  private actors: Map<string, AntActor> = new Map();
  world: World;
  private timer: NodeJS.Timeout | null = null;

  constructor() {
    this.world = projectWorld(this.eventStore.getAllEvents());
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

  private tick() {
    const events = [];
    for (const actor of this.actors.values()) {
      const event = actor.processTick();
      if (event) {
        events.push(event);
      }
    }

    for (const event of events) {
      this.eventStore.dispatch(event);
    }

    this.world = projectWorld(this.eventStore.getAllEvents());
  }

  /**
   * Command handler to create a new ant.
   *
   * @param position The position to create the ant at.
   */
  createAnt(position: Position) {
    const actor = new AntActor(position);
    this.actors.set(actor.id, actor);

    const event: AntCreatedEvent = {
      type: ANT_EVENT_TYPES.CREATED,
      payload: {
        id: actor.id,
        position,
      },
      timestamp: Date.now(),
    };

    this.eventStore.dispatch(event);
    this.world = projectWorld(this.eventStore.getAllEvents());
  }
}
