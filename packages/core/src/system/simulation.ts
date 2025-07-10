import { randomUUID } from "node:crypto";
import { AntActor } from "../actors/ant";
import type { Position, Action, Perception } from "../domain";
import { ACTOR_ACTIONS } from "../domain";
import { Ant, World } from "./world";
import { distance } from "../utils/maths";

const TICK_INTERVAL_MS = 100;

export class Simulation {
  private actors: Map<string, AntActor> = new Map();
  world: World;
  private timer: NodeJS.Timeout | null = null;
  private tickListeners: Set<() => void> = new Set();

  constructor() {
    const width = 1000;
    const height = 600;

    const nestPosition = {
      x: Math.floor(Math.random() * width),
      y: Math.floor(Math.random() * height),
    };

    const foodSources = Array.from({ length: 5 }, () => ({
      id: randomUUID(),
      position: {
        x: Math.floor(Math.random() * width),
        y: Math.floor(Math.random() * height),
      },
      amount: 5,
    }));

    this.world = new World({
      width,
      height,
      nestPosition,
      foodSources,
    });
  }

  private findNearestFood(position: Position) {
    if (this.world.food.length === 0) {
      return null;
    }

    let nearestFood = this.world.food[0];
    if (!nearestFood) {
      return null;
    }

    let minDistance = distance(position, nearestFood.position);

    for (const food of this.world.food) {
      const d = distance(position, food.position);
      if (d < minDistance) {
        minDistance = d;
        nearestFood = food;
      }
    }
    return nearestFood;
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
    const actions = new Map<string, Action>();

    // Collect actions from actors based on their perception of the world
    for (const actor of this.actors.values()) {
      const perception: Perception = {
        nearestFood: this.findNearestFood(actor.getPosition()),
        nestPosition: this.world.nest.position,
      };
      const action = actor.perceive(perception);
      actions.set(actor.id, action);
    }

    // Update the world based on the actors' actions
    for (const [actorId, action] of actions.entries()) {
      const actor = this.actors.get(actorId);
      if (!actor) {
        continue;
      }

      switch (action.type) {
        case ACTOR_ACTIONS.MOVE:
          actor.move(action.payload.directionX, action.payload.directionY);
          break;
        case ACTOR_ACTIONS.TAKE_FOOD: {
          const food = this.world.food.find((f) => f.id === action.payload.foodId);
          if (food) {
            food.amount -= 1;
          }
          break;
        }
        case ACTOR_ACTIONS.IDLE:
          break;
      }
    }

    // Sync actor state back to the public world for the next tick
    for (const actor of this.actors.values()) {
      const ant = this.world.ants.get(actor.id);
      if (ant) {
        ant.position = actor.getPosition();
        ant.state = actor.getState();
        ant.lifecycle = actor.getLifecycle();
      }
    }

    // Remove depleted food sources
    this.world.food = this.world.food.filter((food) => food.amount > 0);

    for (const listener of this.tickListeners) {
      listener();
    }
  }

  createAnt(position: Position) {
    const actor = new AntActor(position);
    this.actors.set(actor.id, actor);

    const ant = new Ant({
      id: actor.id,
      position: actor.getPosition(),
      state: actor.getState(),
      lifecycle: actor.getLifecycle(),
    });
    this.world.ants.set(ant.id, ant);
  }

  addTickListener(listener: () => void) {
    this.tickListeners.add(listener);
  }

  removeTickListener(listener: () => void) {
    this.tickListeners.delete(listener);
  }
}
