import type { SimulationEvent } from "./types";

/**
 * Stores and manages simulation events
 */
export class EventStore {
  private events: SimulationEvent[] = [];

  dispatch(event: SimulationEvent) {
    this.events.push(event);
  }

  getAllEvents() {
    return [...this.events];
  }
}
