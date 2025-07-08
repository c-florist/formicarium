import type { Position } from "./ant-actor";

export const ANT_CREATED = "ANT_CREATED";

/**
 * The base interface for all domain events.
 */
interface DomainEvent<T extends string, P> {
  type: T;
  payload: P;
  timestamp: number;
}

export type AntCreatedEvent = DomainEvent<
  typeof ANT_CREATED,
  {
    id: string;
    position: Position;
  }
>;

export type SimulationEvent = AntCreatedEvent;
