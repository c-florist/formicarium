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

    const initialWorldState = this.wasmSimulation.get_world_state();
    worldStore.set(initialWorldState);
  };

  getWorldStatistics = (): StatsDto => {
    if (!this.wasmSimulation) {
      throw new Error(
        "Simulation not initialised before attempting to get world statistics",
      );
    }
    return this.wasmSimulation.get_world_statistics();
  };

  advance = (): WorldDto => {
    if (!this.wasmSimulation) {
      throw new Error(
        "Simulation not initialised before attempting to advance",
      );
    }
    this.wasmSimulation.tick();

    return this.wasmSimulation.get_world_state();
  };
}

export default new SimulationService();
