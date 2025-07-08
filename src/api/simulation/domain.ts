export type Position = {
  x: number;
  y: number;
};

export type Nest = {
  readonly position: Position;
};

export const ANT_STATES = {
  FORAGING: "FORAGING",
  RETURNING_TO_NEST: "RETURNING_TO_NEST",
} as const;

export type AntState = (typeof ANT_STATES)[keyof typeof ANT_STATES];

export type FoodSource = {
  readonly position: Position;
  amount: number;
};

export const PHEROMONE_TYPES = {
  TO_FOOD: "to_food",
  TO_HOME: "to_home",
} as const;

export type PheromoneType =
  (typeof PHEROMONE_TYPES)[keyof typeof PHEROMONE_TYPES];

export type Pheromone = {
  position: Position;
  intensity: number;
  type: PheromoneType;
};
