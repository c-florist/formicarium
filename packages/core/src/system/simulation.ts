import { randomUUID } from "node:crypto";
import { AntActor } from "../actors/Ant";
import type { AntActorMessage, Perception, Position } from "../domain";
import { ANT_ACTOR_ACTIONS, LIFECYCLE_STATES } from "../domain";
import { distance } from "../utils/maths";
import { AntDto, WorldDto } from "./Dto";

const TICK_INTERVAL_MS = 100;

export class Simulation {
  private antActors: Map<string, AntActor> = new Map();
  world: WorldDto;
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

    this.world = new WorldDto({
      width,
      height,
      nestPosition,
      foodSources,
    });
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

  addTickListener(listener: () => void) {
    this.tickListeners.add(listener);
  }

  removeTickListener(listener: () => void) {
    this.tickListeners.delete(listener);
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

  tick() {
    const actions = new Map<string, AntActorMessage>();

    // Collect actions from actors based on their perception of the world
    for (const actor of this.antActors.values()) {
      const perception: Perception = {
        nearestFood: this.findNearestFood(actor.getPosition()),
        nestPosition: this.world.nest.position,
      };
      const action = actor.perceive(perception);
      actions.set(actor.id, action);
    }

    // Update the world based on the actors' actions
    for (const [actorId, action] of actions.entries()) {
      const actor = this.antActors.get(actorId);
      if (!actor) {
        continue;
      }

      switch (action.actionType) {
        case ANT_ACTOR_ACTIONS.MOVE:
          actor.move(action.payload.directionX, action.payload.directionY);
          break;
        case ANT_ACTOR_ACTIONS.TAKE_FOOD: {
          const food = this.world.food.find(
            (f) => f.id === action.payload.foodId,
          );
          if (food) {
            food.amount -= 1;
          }
          break;
        }
        case ANT_ACTOR_ACTIONS.IDLE:
          break;
      }
    }

    // Sync actor state back to the public world for the next tick
    for (const actor of this.antActors.values()) {
      const ant = this.world.ants.get(actor.id);
      if (ant) {
        ant.position = actor.getPosition();
        ant.state = actor.getState();
        ant.lifecycle = actor.getLifecycle();
      }
    }

    // Remove depleted food sources
    this.world.food = this.world.food.filter((food) => food.amount > 0);

    // Remove dead ants
    for (const actor of this.antActors.values()) {
      if (actor.getLifecycle() === LIFECYCLE_STATES.DEAD) {
        this.antActors.delete(actor.id);
        this.world.ants.delete(actor.id);
      }
    }

    for (const listener of this.tickListeners) {
      listener();
    }
  }

  createAnt(position: Position) {
    const actor = new AntActor(position);
    this.antActors.set(actor.id, actor);

    const ant = new AntDto({
      id: actor.id,
      position: actor.getPosition(),
      state: actor.getState(),
      lifecycle: actor.getLifecycle(),
    });
    this.world.ants.set(ant.id, ant);
  }

  killAnt(antId: string) {
    const actor = this.antActors.get(antId);
    if (actor) {
      actor.kill();
    }
  }
}
