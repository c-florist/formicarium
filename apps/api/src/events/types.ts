import type { Position } from "../domain/types";

/**
 * Base interface for all domain events
 */
export type DomainEvent<T extends string, P> = {
  type: T;
  payload: P;
  timestamp: number;
};

export const ANT_EVENT_TYPES = {
  CREATED: "ANT_CREATED",
  MOVED: "ANT_MOVED",
} as const;

export type AntEventType =
  (typeof ANT_EVENT_TYPES)[keyof typeof ANT_EVENT_TYPES];

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
