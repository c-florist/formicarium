import type {
  SimulationOptions,
  StatsDto,
  WorldDto,
} from "@formicarium/domain";
import {
  set_panic_hook,
  WasmSimulation,
} from "../../../../wasm-client/pkg/wasm_client";

set_panic_hook();

export class SimulationService {
  private simulation: WasmSimulation;

  constructor(options: SimulationOptions) {
    this.simulation = new WasmSimulation(options);
  }

  public tick = (): void => {
    this.simulation.tick();
  };

  public getWorldState = (): WorldDto => {
    return this.simulation.get_world_state();
  };

  public getWorldStatistics = (): StatsDto => {
    return this.simulation.get_world_statistics();
  };
}
