import { AntActor } from "../actors/ant";
import type { Position } from "../domain";
import { Ant, World } from "./world";

const TICK_INTERVAL_MS = 100;

type SimulationOptions = {
  width?: number;
  height?: number;
};

export class Simulation {
  private actors: Map<string, AntActor> = new Map();
  world: World;
  private timer: NodeJS.Timeout | null = null;
  private tickListeners: Set<() => void> = new Set();

  constructor(options: SimulationOptions = {}) {
    const { width = 800, height = 600 } = options;

    const nestPosition = {
      x: Math.floor(Math.random() * width),
      y: Math.floor(Math.random() * height),
    };

    const foodSources = Array.from({ length: 5 }, () => ({
      position: {
        x: Math.floor(Math.random() * width),
        y: Math.floor(Math.random() * height),
      },
      amount: 100,
    }));

    this.world = new World({
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

  tick() {
    // Actors update their internal state based on the current world
    for (const actor of this.actors.values()) {
      actor.update(this.world);
    }

    // Sync actor state back to the public world for the next tick
    for (const actor of this.actors.values()) {
      const ant = this.world.ants.get(actor.id);
      if (ant) {
        ant.position = actor.getPosition();
        ant.state = actor.getState();
      }
    }

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
