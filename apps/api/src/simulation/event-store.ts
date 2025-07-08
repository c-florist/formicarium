import type { SimulationEvent } from "./events";

/**
 * A simple in-memory event store.
 * TODO: Use a real event store in future.
 */
export class EventStore {
  private events: SimulationEvent[] = [];

  /**
   * Appends a new event to the event log.
   * @param event The event to dispatch.
   */
  dispatch(event: SimulationEvent) {
    this.events.push(event);
  }

  /**
   * Retrieves all events in the log.
   * @returns A chronological list of all events.
   */
  getAllEvents() {
    return this.events;
  }
}
