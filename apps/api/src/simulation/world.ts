import { Ant, type Position } from "./ant";

export type FoodSource = {
  position: Position;
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

export class World {
  readonly width: number;
  readonly height: number;
  ants: Ant[] = [];
  food: FoodSource[] = [];
  pheromones: Map<string, Pheromone> = new Map();

  constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
  }

  /**
   * Adds a new ant to the simulation at a given position.
   * @param position The position to add the ant at.
   * @returns The newly created ant.
   */
  addAnt(position: Position) {
    const newAnt = new Ant(position);
    this.ants.push(newAnt);
    return newAnt;
  }

  /**
   * Adds a new food source to the world.
   * @param position The position of the food source.
   * @param amount The amount of food available.
   * @returns The newly created food source.
   */
  addFood(position: Position, amount: number) {
    const newFood = { position, amount };
    this.food.push(newFood);
    return newFood;
  }
}
