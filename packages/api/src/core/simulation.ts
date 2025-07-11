// This class will be responsible for loading and managing the Wasm module.
// For now, it's a placeholder.
export class Simulation {
  public start() {
    console.log("Simulation started!");
  }

  getWorld() {
    // In the future, this will return the DTO from the Wasm module.
    return {
      width: 1000,
      height: 600,
      ants: new Map(),
      foodSources: [],
      pheromones: new Map(),
      nest: { position: { x: 500, y: 300 } },
      toJSON: () => {},
    };
  }

  public addTickListener(_listener: () => void) {
    // Placeholder
  }

  public removeTickListener(_listener: () => void) {
    // Placeholder
  }
}
