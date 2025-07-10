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
  readonly id: string;
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

export const ACTOR_ACTIONS = {
  MOVE: "MOVE",
  TAKE_FOOD: "TAKE_FOOD",
  IDLE: "IDLE",
} as const;

export type ActorAction = (typeof ACTOR_ACTIONS)[keyof typeof ACTOR_ACTIONS];

export type MoveAction = {
  type: typeof ACTOR_ACTIONS.MOVE;
  payload: {
    directionX: number;
    directionY: number;
  };
};

export type TakeFoodAction = {
  type: typeof ACTOR_ACTIONS.TAKE_FOOD;
  payload: {
    foodId: string;
  };
};

export type IdleAction = {
  type: typeof ACTOR_ACTIONS.IDLE;
};

export type Action = MoveAction | TakeFoodAction | IdleAction;

export const LIFECYCLE_STATES = {
  ALIVE: "ALIVE",
  DEAD: "DEAD",
} as const;

export type LifecycleState =
  (typeof LIFECYCLE_STATES)[keyof typeof LIFECYCLE_STATES];

export type Perception = {
  nearestFood: FoodSource | null;
  nestPosition: Position;
};
