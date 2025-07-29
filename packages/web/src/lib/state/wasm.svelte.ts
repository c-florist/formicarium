import type { SimulationService } from "$lib/services/simulation-service";

type WasmState = {
  simulationService: SimulationService | null;
  animationFrameId: number | null;
};

export const wasmState = $state<WasmState>({
  simulationService: null,
  animationFrameId: null,
});
