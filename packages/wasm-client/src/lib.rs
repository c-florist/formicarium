use simulation::{Simulation, SimulationOptions};
use wasm_bindgen::prelude::*;

// Provides better error messages in the browser console when a panic occurs.
#[wasm_bindgen]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct WasmSimulation {
    simulation: Simulation,
}

#[wasm_bindgen]
impl WasmSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new(options: JsValue) -> Result<WasmSimulation, JsValue> {
        let sim_options: SimulationOptions = serde_wasm_bindgen::from_value(options)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse SimulationOptions: {}", e)))?;

        let simulation = Simulation::new(sim_options);
        Ok(WasmSimulation { simulation })
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) {
        self.simulation.tick();
    }

    /// Gets the current state of the world.
    #[wasm_bindgen]
    pub fn get_world_state(&mut self) -> Result<JsValue, JsValue> {
        let world_dto = self
            .simulation
            .get_world_state_dto()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        serde_wasm_bindgen::to_value(&world_dto).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Gets the current statistics of the simulation.
    #[wasm_bindgen]
    pub fn get_world_statistics(&mut self) -> Result<JsValue, JsValue> {
        let stats_dto = self
            .simulation
            .get_world_statistics_dto()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        serde_wasm_bindgen::to_value(&stats_dto).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
