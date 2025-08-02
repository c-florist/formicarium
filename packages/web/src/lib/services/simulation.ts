import { worldStore } from "$lib/stores/world";
import type {
  SimulationOptions,
  StatsDto,
  WorldDto,
} from "@formicarium/domain";
import {
  set_panic_hook,
  WasmSimulation,
} from "../../../../wasm-client/pkg/wasm_client";

// Allows reading Rust logs
set_panic_hook();

class SimulationService {
  private wasmSimulation: WasmSimulation | null = null;

  init = (options: SimulationOptions) => {
    this.wasmSimulation = new WasmSimulation(options);

    const initialWorldData = this.getWorldData();
    worldStore.set(initialWorldData);
  };

  getWorldData = (): { world: WorldDto; stats: StatsDto } => {
    if (!this.wasmSimulation) {
      throw new Error(
        "Simulation not initialised before attempting to get world statistics",
      );
    }
    return {
      world: this.wasmSimulation.get_world_state(),
      stats: this.wasmSimulation.get_world_statistics(),
    };
  };

  tick = () => {
    if (!this.wasmSimulation) {
      throw new Error(
        "Simulation not initialised before attempting to advance",
      );
    }
    this.wasmSimulation.tick();
  };

  destroy = () => {
    if (!this.wasmSimulation) {
      return;
    }
    this.wasmSimulation.free();
    this.wasmSimulation = null;
  };
}

export default new SimulationService();
