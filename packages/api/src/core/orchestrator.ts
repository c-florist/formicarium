import type { WorldDto } from "@formicarium/domain";
import { Simulation } from "../../../core-rs/pkg/core_rs";

export class Orchestrator {
  private tickListeners = new Set<() => void>();
  private simulation: Simulation;

  constructor() {
    this.simulation = new Simulation();
  }

  start() {
    setInterval(() => {
      this.tick();
    }, 50);
  }

  private tick() {
    this.simulation.tick();
    this.tickListeners.forEach((listener) => listener());
  }

  getWorldState(): WorldDto | null {
    try {
      return this.simulation.get_world_state();
    } catch (error) {
      console.error("Error getting world state:", error);
      return null;
    }
  }

  addTickListener(listener: () => void) {
    this.tickListeners.add(listener);
  }

  removeTickListener(listener: () => void) {
    this.tickListeners.delete(listener);
  }
}
