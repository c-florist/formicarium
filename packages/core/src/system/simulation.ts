import { AntActor } from "../actors/Ant";
import { WorldActor } from "../actors/World";
import type { AntActorMessage, Perception } from "../domain";
import { ANT_ACTOR_ACTIONS } from "../domain";

const TICK_INTERVAL_MS = 100;

export class Simulation {
  private antActors: Map<string, AntActor> = new Map();
  private worldActor: WorldActor;
  private timer: NodeJS.Timeout | null = null;
  private tickListeners: Set<() => void> = new Set();

  constructor() {
    const width = 1000;
    const height = 600;

    this.worldActor = new WorldActor({
      width: width,
      height: height,
    });

    // Currently AntActors are funnelled into WorldActor.ManagedAnts - this should change in future
    for (let i = 0; i < 100; i++) {
      const ant = new AntActor({
        x: Math.random() * width,
        y: Math.random() * height,
      });
      this.antActors.set(ant.id, ant);
      this.worldActor.addAnt({
        id: ant.id,
        position: ant.getPosition(),
        state: ant.getState(),
        lifecycle: ant.getLifecycle(),
      });
    }
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

  getWorld() {
    return this.worldActor.getDto();
  }

  processActions(actions: Map<AntActor, AntActorMessage>) {
    for (const [antActor, action] of actions) {
      switch (action.actionType) {
        case ANT_ACTOR_ACTIONS.MOVE:
          antActor.move(action.payload.directionX, action.payload.directionY);
          break;

        case ANT_ACTOR_ACTIONS.TAKE_FOOD:
          this.worldActor.depleteFoodSource(action.payload.foodId);
          break;
      }
    }
  }

  tick() {
    const currentWorld = this.getWorld();
    const actions = new Map<AntActor, AntActorMessage>();

    // Collect actions to be processed
    for (const antActor of this.antActors.values()) {
      const antActorPosition = antActor.getPosition();

      const perception: Perception = {
        nearestFood: this.worldActor.getNearestFoodSource(antActorPosition),
        nestPosition: currentWorld.nest.position,
      };
      const antActorResponse = antActor.perceive(perception);
      actions.set(antActor, antActorResponse);
    }

    this.processActions(actions);

    // Synchronise world state - this is a departure from the actor modelling as its centralised
    for (const antActor of this.antActors.values()) {
      this.worldActor.updateAnt({
        id: antActor.id,
        position: antActor.getPosition(),
        lifecycle: antActor.getLifecycle(),
        state: antActor.getState(),
      });
    }

    this.worldActor.endOfTickCleanup();

    for (const listener of this.tickListeners) {
      listener();
    }
  }
}
