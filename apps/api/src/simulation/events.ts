import type { Position } from "./ant";

export const ANT_EVENT_TYPES = {
  CREATED: "ANT_CREATED",
  MOVED: "ANT_MOVED",
} as const;

export type AntEventType = (typeof ANT_EVENT_TYPES)[keyof typeof ANT_EVENT_TYPES];

/**
 * The base interface for all domain events.
 */
interface DomainEvent<T extends string, P> {
  type: T;
  payload: P;
  timestamp: number;
}

export type AntCreatedEvent = DomainEvent<
  typeof ANT_EVENT_TYPES.CREATED,
  {
    id: string;
    position: Position;
  }
>;

export type AntMovedEvent = DomainEvent<
  typeof ANT_EVENT_TYPES.MOVED,
  {
    id: string;
    position: Position;
  }
>;

export type SimulationEvent = AntCreatedEvent | AntMovedEvent;
